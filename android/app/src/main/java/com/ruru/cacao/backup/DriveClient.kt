package com.ruru.cacao.backup

import java.io.ByteArrayOutputStream
import java.net.HttpURLConnection
import java.net.URL
import java.net.URLEncoder
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class DriveFile(val id: String, val name: String, val createdTime: String = "", val size: String = "")

@Serializable
private data class DriveList(val files: List<DriveFile> = emptyList())

/** Google 雲端硬碟 appDataFolder（D-05）：只有 Cacao 讀得到，使用者在雲端硬碟裡看不到 */
class DriveClient(private val accessToken: String) {
    private val json = Json { ignoreUnknownKeys = true }

    fun upload(name: String, bytes: ByteArray) {
        val boundary = "cacao-${System.nanoTime()}"
        val metadata = """{"name":"$name","parents":["appDataFolder"]}"""
        val body = ByteArrayOutputStream().apply {
            write("--$boundary\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n$metadata\r\n".toByteArray())
            write("--$boundary\r\nContent-Type: application/octet-stream\r\n\r\n".toByteArray())
            write(bytes)
            write("\r\n--$boundary--\r\n".toByteArray())
        }.toByteArray()
        request("POST", "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart", body, "multipart/related; boundary=$boundary")
    }

    /** 新到舊 */
    fun list(): List<DriveFile> {
        val q = URLEncoder.encode("files(id,name,createdTime,size)", "UTF-8")
        val text = request("GET", "https://www.googleapis.com/drive/v3/files?spaces=appDataFolder&orderBy=createdTime%20desc&pageSize=100&fields=$q")
        return json.decodeFromString(DriveList.serializer(), text.decodeToString()).files
    }

    fun download(id: String): ByteArray = request("GET", "https://www.googleapis.com/drive/v3/files/$id?alt=media")

    fun delete(id: String) {
        request("DELETE", "https://www.googleapis.com/drive/v3/files/$id")
    }

    /** 雲端只留最近 keep 份 */
    fun prune(keep: Int = 7) {
        list().drop(keep).forEach { delete(it.id) }
    }

    private fun request(method: String, url: String, body: ByteArray? = null, contentType: String? = null): ByteArray {
        val conn = (URL(url).openConnection() as HttpURLConnection).apply {
            requestMethod = method
            connectTimeout = 15_000
            readTimeout = 60_000
            setRequestProperty("Authorization", "Bearer $accessToken")
            if (body != null) {
                doOutput = true
                setRequestProperty("Content-Type", contentType)
                outputStream.use { it.write(body) }
            }
        }
        try {
            val code = conn.responseCode
            if (code !in 200..299) {
                val err = conn.errorStream?.readBytes()?.decodeToString().orEmpty()
                error("Drive $method $code $err")
            }
            return conn.inputStream.use { it.readBytes() }
        } finally {
            conn.disconnect()
        }
    }
}
