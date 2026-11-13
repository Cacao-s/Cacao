package com.ruru.cacao.domain

import java.time.LocalDate

/** 日期輸入：接受 20230830、2023-08-30、2023/08/30、2023.08.30；不存在的日期回 null */
object DateInput {

    private val separated = Regex("""(\d{4})[-/.](\d{1,2})[-/.](\d{1,2})""")
    private val compact = Regex("""(\d{4})(\d{2})(\d{2})""")

    fun parse(input: String): LocalDate? {
        val match = separated.matchEntire(input.trim()) ?: compact.matchEntire(input.trim()) ?: return null
        val (y, m, d) = match.destructured
        return runCatching { LocalDate.of(y.toInt(), m.toInt(), d.toInt()) }.getOrNull()
    }
}

/** 生日：同 [DateInput]，未來日期回 null */
object BirthdayInput {
    fun parse(input: String, today: LocalDate = LocalDate.now()): LocalDate? =
        DateInput.parse(input)?.takeUnless { it.isAfter(today) }
}
