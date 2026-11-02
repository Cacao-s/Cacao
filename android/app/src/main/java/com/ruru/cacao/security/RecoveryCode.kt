package com.ruru.cacao.security

import java.security.SecureRandom

/** 忘記 PIN 用的救援碼：16 字元分 4 組，排除 0／O、1／I／L 這類容易看錯的字 */
object RecoveryCode {
    const val ALPHABET = "23456789ABCDEFGHJKMNPQRSTUVWXYZ"
    private val random = SecureRandom()

    fun generate(): String =
        (1..16).map { ALPHABET[random.nextInt(ALPHABET.length)] }
            .chunked(4)
            .joinToString("-") { it.joinToString("") }

    fun normalize(input: String): String =
        input.uppercase().filter { it in ALPHABET }
}
