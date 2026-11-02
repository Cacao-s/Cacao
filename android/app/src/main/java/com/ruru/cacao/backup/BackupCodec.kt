package com.ruru.cacao.backup

import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.security.SecureRandom
import java.util.Base64
import java.util.zip.ZipEntry
import java.util.zip.ZipInputStream
import java.util.zip.ZipOutputStream
import javax.crypto.AEADBadTagException
import javax.crypto.Cipher
import javax.crypto.spec.GCMParameterSpec
import javax.crypto.spec.SecretKeySpec
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import org.bouncycastle.crypto.generators.Argon2BytesGenerator
import org.bouncycastle.crypto.params.Argon2Parameters

/** 備份檔的說明（auth-design §3）；兩平台同一格式 */
@Serializable
data class BackupManifest(
    val formatVersion: Int = 1,
    val schemaVersion: Long,
    val appVersion: String,
    val platform: String = "android",
    val createdAt: String,
    val encrypted: Boolean = false,
    val kdf: KdfParams? = null,
)

@Serializable
data class KdfParams(val salt: String, val memoryKib: Int, val iterations: Int, val parallelism: Int, val nonce: String)

sealed interface UnpackResult {
    data class Ok(val manifest: BackupManifest, val database: ByteArray) : UnpackResult
    data object NotCacao : UnpackResult
    data object NeedsPassword : UnpackResult
    data object WrongPassword : UnpackResult
    data class TooNew(val schemaVersion: Long) : UnpackResult
}

/** `.cacao` = ZIP（manifest.json ＋ cacao.db 或加密的 cacao.db.enc） */
object BackupCodec {
    private const val MANIFEST = "manifest.json"
    private const val DB = "cacao.db"
    private const val DB_ENC = "cacao.db.enc"
    private val json = Json { ignoreUnknownKeys = true; encodeDefaults = true }
    private val random = SecureRandom()
    private val b64 = Base64.getEncoder().withoutPadding()
    private val unb64 = Base64.getDecoder()

    fun pack(database: ByteArray, manifest: BackupManifest, password: String?): ByteArray {
        var m = manifest.copy(encrypted = password != null)
        val payload = if (password == null) database else {
            val salt = ByteArray(16).also(random::nextBytes)
            val nonce = ByteArray(12).also(random::nextBytes)
            val kdf = KdfParams(b64.encodeToString(salt), 19_456, 2, 1, b64.encodeToString(nonce))
            m = m.copy(kdf = kdf)
            cipher(Cipher.ENCRYPT_MODE, password, kdf).doFinal(database)
        }
        val out = ByteArrayOutputStream()
        ZipOutputStream(out).use { zip ->
            zip.putNextEntry(ZipEntry(MANIFEST)); zip.write(json.encodeToString(BackupManifest.serializer(), m).toByteArray()); zip.closeEntry()
            zip.putNextEntry(ZipEntry(if (password == null) DB else DB_ENC)); zip.write(payload); zip.closeEntry()
        }
        return out.toByteArray()
    }

    fun unpack(bytes: ByteArray, password: String?, currentSchemaVersion: Long): UnpackResult {
        val files = runCatching {
            buildMap {
                ZipInputStream(ByteArrayInputStream(bytes)).use { zip ->
                    generateSequence { zip.nextEntry }.forEach { put(it.name, zip.readBytes()) }
                }
            }
        }.getOrNull() ?: return UnpackResult.NotCacao
        val manifest = files[MANIFEST]?.let { runCatching { json.decodeFromString(BackupManifest.serializer(), it.decodeToString()) }.getOrNull() }
            ?: return UnpackResult.NotCacao
        if (manifest.schemaVersion > currentSchemaVersion) return UnpackResult.TooNew(manifest.schemaVersion)
        if (!manifest.encrypted) return files[DB]?.let { UnpackResult.Ok(manifest, it) } ?: UnpackResult.NotCacao
        if (password == null) return UnpackResult.NeedsPassword
        val kdf = manifest.kdf ?: return UnpackResult.NotCacao
        val encrypted = files[DB_ENC] ?: return UnpackResult.NotCacao
        return try {
            UnpackResult.Ok(manifest, cipher(Cipher.DECRYPT_MODE, password, kdf).doFinal(encrypted))
        } catch (_: AEADBadTagException) {
            UnpackResult.WrongPassword
        }
    }

    private fun cipher(mode: Int, password: String, kdf: KdfParams): Cipher {
        val params = Argon2Parameters.Builder(Argon2Parameters.ARGON2_id)
            .withVersion(Argon2Parameters.ARGON2_VERSION_13)
            .withMemoryAsKB(kdf.memoryKib)
            .withIterations(kdf.iterations)
            .withParallelism(kdf.parallelism)
            .withSalt(unb64.decode(kdf.salt))
            .build()
        val key = ByteArray(32).also { Argon2BytesGenerator().apply { init(params) }.generateBytes(password.toByteArray(), it) }
        return Cipher.getInstance("AES/GCM/NoPadding").apply {
            init(mode, SecretKeySpec(key, "AES"), GCMParameterSpec(128, unb64.decode(kdf.nonce)))
        }
    }
}
