package com.ruru.cacao.data

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.domain.Period
import java.time.Clock
import java.time.Instant
import java.time.LocalDate
import java.time.ZoneId
import java.time.ZoneOffset
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Before
import org.junit.Test

private class MovableClock(var date: LocalDate) : Clock() {
    override fun getZone(): ZoneId = ZoneOffset.UTC
    override fun withZone(zone: ZoneId?): Clock = this
    override fun instant(): Instant = date.atTime(9, 0).toInstant(ZoneOffset.UTC)
}

class AllowanceRepositoryTest {

    private val clock = MovableClock(LocalDate.parse("2026-11-02"))
    private lateinit var allowance: AllowanceRepository
    private lateinit var ledger: LedgerRepository
    private lateinit var mom: String
    private lateinit var kid: String

    @Before
    fun setUp() {
        val driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        CacaoDatabase.Schema.create(driver)
        val db = CacaoDatabase(driver)
        val members = MemberRepository(db)
        mom = members.createParent("媽媽", "1234").first
        kid = members.createChild("小安", "2017-03-12")
        ledger = LedgerRepository(db, clock)
        allowance = AllowanceRepository(db, clock)
    }

    private fun savings() = ledger.balances(kid).savings

    @Test
    fun backfillUsesOriginalDueDates() {
        allowance.create(kid, ScheduleInput(mom, 100, Period.WEEKLY, 1, null))
        assertEquals("2026-11-02 是星期一，當天就發", 100, savings())

        clock.date = LocalDate.parse("2026-11-20")
        assertEquals(2, allowance.backfillAll())
        assertEquals(300, savings())
        assertEquals(
            listOf("2026-11-16", "2026-11-09", "2026-11-02"),
            ledger.entries(kid).map { it.occurred_on },
        )
        assertEquals("再開一次不會重複發", 0, allowance.backfillAll())
    }

    @Test
    fun pausedPeriodIsNotPaid() {
        allowance.create(kid, ScheduleInput(mom, 10, Period.DAILY, null, null))
        val id = allowance.schedules(kid).single().id
        clock.date = LocalDate.parse("2026-11-04")
        allowance.setPaused(id, true)
        assertEquals("暫停前先補到今天：2、3、4 號", 30, savings())

        clock.date = LocalDate.parse("2026-11-10")
        assertEquals(0, allowance.backfillAll())
        allowance.setPaused(id, false)
        assertEquals("恢復當天開始發", 40, savings())
    }

    @Test
    fun amountChangeAppliesFromNextDue() {
        allowance.create(kid, ScheduleInput(mom, 10, Period.DAILY, null, null))
        val id = allowance.schedules(kid).single().id
        clock.date = LocalDate.parse("2026-11-03")
        allowance.update(id, ScheduleInput(mom, 50, Period.DAILY, null, null))
        assertEquals("2、3 號照舊金額", 20, savings())

        clock.date = LocalDate.parse("2026-11-05")
        allowance.backfillAll()
        assertEquals("4、5 號用新金額", 120, savings())
    }

    @Test
    fun invalidInputRejected() {
        assertFalse(allowance.create(kid, ScheduleInput(mom, 0, Period.DAILY, null, null)))
        assertFalse(allowance.create(kid, ScheduleInput(mom, 100, Period.WEEKLY, null, null)))
        assertFalse(allowance.create(kid, ScheduleInput(mom, 100, Period.MONTHLY, null, 32)))
    }
}
