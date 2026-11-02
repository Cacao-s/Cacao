package com.ruru.cacao.security

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class SecurityTest {

    @Test
    fun pinHashVerifies() {
        val phc = PinHasher.hash("1234")
        assertTrue(phc.startsWith("\$argon2id\$v=19\$m=19456,t=2,p=1\$"))
        assertTrue(PinHasher.verify("1234", phc))
        assertFalse(PinHasher.verify("1235", phc))
    }

    @Test
    fun sameSecretGetsDifferentSalt() {
        assertNotEquals(PinHasher.hash("1234"), PinHasher.hash("1234"))
    }

    @Test
    fun brokenHashIsRejected() {
        assertFalse(PinHasher.verify("1234", "not-a-hash"))
    }

    @Test
    fun recoveryCodeFormat() {
        repeat(50) {
            val code = RecoveryCode.generate()
            assertTrue(code, Regex("^[${RecoveryCode.ALPHABET}]{4}(-[${RecoveryCode.ALPHABET}]{4}){3}$").matches(code))
            "0O1IL".forEach { bad -> assertFalse(code, bad in code) }
        }
        assertEquals("K7QM2XRP9TWDF4HN", RecoveryCode.normalize(" k7qm-2xrp 9twd-f4hn "))
    }

    @Test
    fun attemptLimiterWaits() {
        var state = AttemptState()
        repeat(4) { state = AttemptLimiter.onFailure(state, 0) }
        assertEquals(0, AttemptLimiter.remainingSeconds(state, 0))

        state = AttemptLimiter.onFailure(state, 0)
        assertEquals(30, AttemptLimiter.remainingSeconds(state, 0))
        assertEquals(0, AttemptLimiter.remainingSeconds(state, 30_000))

        repeat(4) { state = AttemptLimiter.onFailure(state, 30_000) }
        assertEquals(0, AttemptLimiter.remainingSeconds(state, 30_000))
        state = AttemptLimiter.onFailure(state, 30_000)
        assertEquals(300, AttemptLimiter.remainingSeconds(state, 30_000))

        repeat(5) { state = AttemptLimiter.onFailure(state, 400_000) }
        assertEquals(300, AttemptLimiter.remainingSeconds(state, 400_000))
    }
}
