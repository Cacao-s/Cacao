package com.ruru.cacao.domain

import java.time.LocalDate

enum class Period(val code: String) { DAILY("daily"), WEEKLY("weekly"), BIWEEKLY("biweekly"), MONTHLY("monthly") }

/** 常態零用錢的到期日（DQ-10）：每兩週以 activeFrom 之後第一個符合的星期為基準；每月選 29～31 號遇小月份在月底發 */
object AllowanceDates {

    fun due(
        period: Period,
        weekday: Int?,
        monthDay: Int?,
        activeFrom: LocalDate,
        from: LocalDate,
        to: LocalDate,
    ): List<LocalDate> {
        val start = maxOf(from, activeFrom)
        if (start.isAfter(to)) return emptyList()
        return when (period) {
            Period.DAILY -> generateSequence(start) { it.plusDays(1) }.takeWhile { !it.isAfter(to) }.toList()
            Period.WEEKLY -> {
                val first = nextWeekday(start, requireNotNull(weekday))
                generateSequence(first) { it.plusWeeks(1) }.takeWhile { !it.isAfter(to) }.toList()
            }
            Period.BIWEEKLY -> {
                val anchor = nextWeekday(activeFrom, requireNotNull(weekday))
                generateSequence(anchor) { it.plusWeeks(2) }
                    .takeWhile { !it.isAfter(to) }
                    .filter { !it.isBefore(start) }
                    .toList()
            }
            Period.MONTHLY -> {
                val day = requireNotNull(monthDay)
                generateSequence(start.withDayOfMonth(1)) { it.plusMonths(1) }
                    .takeWhile { !it.isAfter(to) }
                    .map { it.withDayOfMonth(minOf(day, it.lengthOfMonth())) }
                    .filter { !it.isBefore(start) && !it.isAfter(to) }
                    .toList()
            }
        }
    }

    private fun nextWeekday(date: LocalDate, weekday: Int): LocalDate {
        val diff = (weekday - date.dayOfWeek.value + 7) % 7
        return date.plusDays(diff.toLong())
    }
}
