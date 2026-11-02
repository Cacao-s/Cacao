package com.ruru.cacao.domain

import java.time.LocalDate
import org.junit.Assert.assertEquals
import org.junit.Test

class AllowanceDatesTest {

    private fun d(s: String) = LocalDate.parse(s)

    @Test
    fun daily() {
        assertEquals(
            listOf(d("2026-11-01"), d("2026-11-02"), d("2026-11-03")),
            AllowanceDates.due(Period.DAILY, null, null, d("2026-11-01"), d("2026-10-01"), d("2026-11-03")),
        )
    }

    @Test
    fun weeklyOnMonday() {
        // 2026-11-02 是星期一
        assertEquals(
            listOf(d("2026-11-02"), d("2026-11-09"), d("2026-11-16")),
            AllowanceDates.due(Period.WEEKLY, 1, null, d("2026-11-01"), d("2026-11-01"), d("2026-11-20")),
        )
    }

    @Test
    fun biweeklyKeepsAnchorWhenResumingFromLaterDate() {
        val all = AllowanceDates.due(Period.BIWEEKLY, 5, null, d("2026-11-01"), d("2026-11-01"), d("2026-12-31"))
        assertEquals(listOf(d("2026-11-06"), d("2026-11-20"), d("2026-12-04"), d("2026-12-18")), all)
        val later = AllowanceDates.due(Period.BIWEEKLY, 5, null, d("2026-11-01"), d("2026-11-21"), d("2026-12-31"))
        assertEquals(listOf(d("2026-12-04"), d("2026-12-18")), later)
    }

    @Test
    fun monthlyOn31UsesMonthEnd() {
        assertEquals(
            listOf(d("2026-11-30"), d("2026-12-31"), d("2027-01-31"), d("2027-02-28")),
            AllowanceDates.due(Period.MONTHLY, null, 31, d("2026-11-01"), d("2026-11-01"), d("2027-03-01")),
        )
    }

    @Test
    fun nothingBeforeActiveFromOrAfterTo() {
        assertEquals(emptyList<LocalDate>(), AllowanceDates.due(Period.DAILY, null, null, d("2026-12-01"), d("2026-11-01"), d("2026-11-30")))
        assertEquals(emptyList<LocalDate>(), AllowanceDates.due(Period.MONTHLY, null, 15, d("2026-11-16"), d("2026-11-16"), d("2026-12-14")))
    }
}
