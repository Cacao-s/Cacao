package com.ruru.cacao.domain

import java.time.LocalDate

/** 生日輸入：接受 20230830、2023-08-30、2023/08/30、2023.08.30；未來日期與不存在的日期回 null */
object BirthdayInput {

    private val separated = Regex("""(\d{4})[-/.](\d{1,2})[-/.](\d{1,2})""")
    private val compact = Regex("""(\d{4})(\d{2})(\d{2})""")

    fun parse(input: String, today: LocalDate = LocalDate.now()): LocalDate? {
        val text = input.trim()
        val match = separated.matchEntire(text) ?: compact.matchEntire(text) ?: return null
        val (y, m, d) = match.destructured
        val date = runCatching { LocalDate.of(y.toInt(), m.toInt(), d.toInt()) }.getOrNull() ?: return null
        return date.takeUnless { it.isAfter(today) }
    }
}
