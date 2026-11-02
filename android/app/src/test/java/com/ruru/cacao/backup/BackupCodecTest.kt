package com.ruru.cacao.backup

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class BackupCodecTest {

    private val db = "SQLite format 3\u0000 假資料".toByteArray()
    private val manifest = BackupManifest(schemaVersion = 1, appVersion = "0.1.0", createdAt = "2026-11-02T00:00:00Z")

    @Test
    fun plainRoundTrip() {
        val packed = BackupCodec.pack(db, manifest, null)
        val result = BackupCodec.unpack(packed, null, currentSchemaVersion = 1) as UnpackResult.Ok
        assertArrayEquals(db, result.database)
        assertEquals("android", result.manifest.platform)
        assertEquals(false, result.manifest.encrypted)
    }

    @Test
    fun encryptedNeedsRightPassword() {
        val packed = BackupCodec.pack(db, manifest, "secret")
        assertEquals(UnpackResult.NeedsPassword, BackupCodec.unpack(packed, null, 1))
        assertEquals(UnpackResult.WrongPassword, BackupCodec.unpack(packed, "wrong", 1))
        val ok = BackupCodec.unpack(packed, "secret", 1) as UnpackResult.Ok
        assertArrayEquals(db, ok.database)
        assertTrue(ok.manifest.encrypted)
    }

    @Test
    fun rejectsNewerSchemaAndGarbage() {
        val newer = BackupCodec.pack(db, manifest.copy(schemaVersion = 9), null)
        assertEquals(UnpackResult.TooNew(9), BackupCodec.unpack(newer, null, 1))
        assertEquals(UnpackResult.NotCacao, BackupCodec.unpack("not a zip".toByteArray(), null, 1))
    }
}
