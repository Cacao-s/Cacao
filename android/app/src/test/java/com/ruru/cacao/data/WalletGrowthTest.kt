package com.ruru.cacao.data

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.ruru.cacao.db.CacaoDatabase
import java.time.LocalDate
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test

class WalletGrowthTest {

    private lateinit var ledger: LedgerRepository
    private lateinit var tasks: TaskRepository
    private lateinit var goals: GoalRepository
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
        goals = GoalRepository(db)
        ledger = LedgerRepository(db, goals = { goals.states(it) })
        goals.ledger = ledger
        tasks = TaskRepository(db, ledger)
    }

    @Test
    fun expenseAndCalibrate() {
        ledger.addIncome(kid, 500, mom, mom)
        ledger.payout(kid, 300, mom)
        ledger.addWalletExpense(kid, 80, kid, "文具")
        assertEquals(220, ledger.balances(kid).wallet)

        ledger.calibrateWallet(kid, 120, kid)
        assertEquals(120, ledger.balances(kid).wallet)
        assertEquals(-100, ledger.entries(kid).first { it.kind == "wallet_adjustment" }.amount)

        assertEquals("錢都花完了，歸零", LedgerResult.Ok, ledger.calibrateWallet(kid, 0, kid))
        assertEquals(0, ledger.balances(kid).wallet)
        ledger.calibrateWallet(kid, 0, kid)
        assertEquals("沒有差額不記", 2, ledger.entries(kid).count { it.kind == "wallet_adjustment" })
        assertEquals(LedgerResult.InvalidAmount, ledger.addWalletExpense(kid, 0, kid, null))
    }

    @Test
    fun growthCountsThisAgeYear() {
        ledger.addIncome(kid, 500, mom, mom)
        ledger.addIncome(kid, 200, mom, mom)
        val voided = ledger.entries(kid).first { it.amount == 200L }.group_id
        ledger.voidGroup(voided, mom)
        tasks.create("整理房間", 50, mom, listOf(kid), mom)
        tasks.complete(tasks.pending(kid).single().id, mom)
        goals.create(kid, "樂高", 300, kid)
        val goalId = goals.views(kid).single().goal.id
        goals.confirm(kid, goalId, kid)
        goals.payout(kid, goalId, 300, mom, null)

        val today = LocalDate.now()
        val g = ledger.growth(kid, today.minusDays(1), today.plusDays(1))
        assertEquals("作廢的不算，任務獎勵算", 550, g.saved)
        assertEquals(1, g.tasksDone)
        assertEquals(1, g.goalsDone)

        val lastYear = ledger.growth(kid, today.minusYears(1), today.minusDays(1))
        assertEquals(0, lastYear.saved)
    }

    @Test
    fun ageYearWindow() {
        val (from, to) = ageYear(LocalDate.parse("2017-03-12"), 9)
        assertEquals(LocalDate.parse("2026-03-12"), from)
        assertEquals(LocalDate.parse("2027-03-12"), to)
    }
}
