package com.ruru.cacao.security

import java.security.MessageDigest
import java.security.SecureRandom
import java.util.Base64
import org.bouncycastle.crypto.generators.Argon2BytesGenerator
import org.bouncycastle.crypto.params.Argon2Parameters

/** Argon2id 雜湊，輸出標準 PHC 字串，Phase 2 的 iOS（libsodium）可驗證同一個字串（D-08） */
object PinHasher {
    private const val MEMORY_KIB = 19_456
    private const val ITERATIONS = 2
    private const val PARALLELISM = 1
    private const val HASH_LENGTH = 32
    private const val SALT_LENGTH = 16

    private val encoder = Base64.getEncoder().withoutPadding()
    private val decoder = Base64.getDecoder()
    private val random = SecureRandom()

    fun hash(secret: String): String {
        val salt = ByteArray(SALT_LENGTH).also(random::nextBytes)
        val hash = derive(secret, salt, MEMORY_KIB, ITERATIONS, PARALLELISM, HASH_LENGTH)
        return "\$argon2id\$v=19\$m=$MEMORY_KIB,t=$ITERATIONS,p=$PARALLELISM\$${encoder.encodeToString(salt)}\$${encoder.encodeToString(hash)}"
    }

    fun verify(secret: String, phc: String): Boolean {
        val parts = phc.split("$")
        if (parts.size != 6 || parts[1] != "argon2id" || parts[2] != "v=19") return false
        val params = parts[3].split(",").associate { it.substringBefore("=") to it.substringAfter("=").toInt() }
        val salt = decoder.decode(parts[4])
        val expected = decoder.decode(parts[5])
        val actual = derive(secret, salt, params.getValue("m"), params.getValue("t"), params.getValue("p"), expected.size)
        return MessageDigest.isEqual(expected, actual)
    }

    private fun derive(secret: String, salt: ByteArray, memory: Int, iterations: Int, parallelism: Int, length: Int): ByteArray {
        val parameters = Argon2Parameters.Builder(Argon2Parameters.ARGON2_id)
            .withVersion(Argon2Parameters.ARGON2_VERSION_13)
            .withMemoryAsKB(memory)
            .withIterations(iterations)
            .withParallelism(parallelism)
            .withSalt(salt)
            .build()
        val generator = Argon2BytesGenerator().apply { init(parameters) }
        return ByteArray(length).also { generator.generateBytes(secret.toByteArray(Charsets.UTF_8), it) }
    }
}
