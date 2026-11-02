package com.ruru.cacao.backup

import android.content.Context
import android.content.Intent
import android.database.sqlite.SQLiteDatabase
import android.net.Uri
import app.cash.sqldelight.db.SqlDriver
import com.ruru.cacao.BuildConfig
import com.ruru.cacao.db.CacaoDatabase
import java.io.File
import java.time.Instant

sealed interface ImportResult {
    data object Ok : ImportResult
    data object NotCacao : ImportResult
    data object NeedsPassword : ImportResult
    data object WrongPassword : ImportResult
    data object TooNew : ImportResult
    data object Corrupted : ImportResult
}

/** 產生備份、匯出、匯入（整個覆蓋，覆蓋前先在手機裡留一份目前資料） */
class BackupManager(private val context: Context, private val driver: SqlDriver) {

    private val dbFile: File get() = context.getDatabasePath(DB_NAME)
    private val preImport: File get() = File(context.filesDir, "pre-import.cacao")

    /** `VACUUM INTO` 產生完整快照（App 執行中也能做，避開 -wal 不完整） */
    fun snapshot(password: String? = null): ByteArray {
        val temp = File(context.cacheDir, "snapshot-${System.nanoTime()}.db")
        try {
            driver.execute(null, "VACUUM INTO '${temp.absolutePath.replace("'", "''")}'", 0)
            val manifest = BackupManifest(
                schemaVersion = CacaoDatabase.Schema.version,
                appVersion = BuildConfig.VERSION_NAME,
                createdAt = Instant.now().toString(),
            )
            return BackupCodec.pack(temp.readBytes(), manifest, password)
        } finally {
            temp.delete()
        }
    }

    fun exportTo(uri: Uri, password: String?) {
        val bytes = snapshot(password)
        context.contentResolver.openOutputStream(uri, "wt")?.use { it.write(bytes) } ?: error("無法寫入")
    }

    fun read(uri: Uri): ByteArray = context.contentResolver.openInputStream(uri)?.use { it.readBytes() } ?: ByteArray(0)

    fun hasPreImportBackup(): Boolean = preImport.exists()

    /** 檢查後才取代；成功時呼叫端要重新啟動 App */
    fun import(bytes: ByteArray, password: String?): ImportResult {
        val db = when (val r = BackupCodec.unpack(bytes, password, CacaoDatabase.Schema.version)) {
            is UnpackResult.Ok -> r.database
            UnpackResult.NotCacao -> return ImportResult.NotCacao
            UnpackResult.NeedsPassword -> return ImportResult.NeedsPassword
            UnpackResult.WrongPassword -> return ImportResult.WrongPassword
            is UnpackResult.TooNew -> return ImportResult.TooNew
        }
        val candidate = File(context.cacheDir, "import-candidate.db").apply { writeBytes(db) }
        if (!integrityOk(candidate)) {
            candidate.delete()
            return ImportResult.Corrupted
        }
        preImport.writeBytes(snapshot())
        driver.close()
        File(dbFile.path + "-wal").delete()
        File(dbFile.path + "-shm").delete()
        candidate.copyTo(dbFile, overwrite = true)
        candidate.delete()
        return ImportResult.Ok
    }

    /** 把匯入前的資料還原回來 */
    fun restorePreImport(): ImportResult = if (preImport.exists()) import(preImport.readBytes(), null) else ImportResult.NotCacao

    private fun integrityOk(file: File): Boolean = runCatching {
        SQLiteDatabase.openDatabase(file.path, null, SQLiteDatabase.OPEN_READONLY).use { db ->
            db.rawQuery("PRAGMA integrity_check", null).use { c -> c.moveToFirst() && c.getString(0) == "ok" }
        }
    }.getOrDefault(false)

    companion object {
        const val DB_NAME = "cacao.db"

        fun restartApp(context: Context) {
            val intent = context.packageManager.getLaunchIntentForPackage(context.packageName)!!
                .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK)
            context.startActivity(intent)
            Runtime.getRuntime().exit(0)
        }
    }
}
