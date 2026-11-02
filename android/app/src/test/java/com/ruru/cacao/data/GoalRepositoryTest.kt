package com.ruru.cacao.data

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.domain.GoalStatus
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test

class GoalRepositoryTest {

    private lateinit var goals: GoalRepository
    private lateinit var ledger: LedgerRepository
    private lateinit var mom: String
    private lateinit var grandma: String
    private lateinit var kid: String

    @Before
    fun setUp() {
        val driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        CacaoDatabase.Schema.create(driver)
        val db = CacaoDatabase(driver)
        val members = MemberRepository(db)
        mom = members.createParent("媽媽", "1234").first
        grandma = members.createElder("外婆")
        kid = members.createChild("小安", "2017-03-12")
        goals = GoalRepository(db)
        ledger = LedgerRepository(db, goals = { goals.states(it) })
        goals.ledger = ledger
    }

    private fun goal(name: String) = goals.views(kid).first { it.goal.name == name }

    @Test
    fun waterfallFillsByPriority() {
        ledger.addIncome(kid, 1000, mom, mom)
        goals.create(kid, "樂高", 600, kid)
        goals.create(kid, "腳踏車", 3000, kid)

        assertTrue(goal("樂高").achievable)
        assertEquals(400, goal("腳踏車").saved)

        goals.move(kid, goal("腳踏車").goal.id, up = true)
        assertEquals(1000, goal("腳踏車").saved)
        assertFalse(goal("樂高").achievable)

        goals.setFill(goal("腳踏車").goal.id, false)
        assertTrue("取消勾選後錢讓給下一個", goal("樂高").achievable)
    }

    @Test
    fun confirmLocksMoneyAndRevertUnlocks() {
        ledger.addIncome(kid, 1000, mom, mom)
        goals.create(kid, "樂高", 600, kid)
        val id = goal("樂高").goal.id

        assertEquals(GoalResult.Ok, goals.confirm(kid, id, kid))
        assertEquals(GoalStatus.ACHIEVED, goal("樂高").state.status)
        assertEquals(600, ledger.balances(kid).locked)
        assertEquals(400, ledger.balances(kid).available)
        assertEquals("鎖住的錢不能一般出金", LedgerResult.OverAvailable(400), ledger.payout(kid, 500, mom))
        assertEquals("已達成不能改金額", GoalResult.NotAllowed, goals.changeTarget(kid, id, 700, mom))

        assertEquals(GoalResult.Ok, goals.revert(kid, id, kid))
        assertEquals(0, ledger.balances(kid).locked)
        assertEquals(GoalResult.Ok, goals.changeTarget(kid, id, 700, mom))
        assertEquals(700, goal("樂高").goal.target_amount)
    }

    @Test
    fun cannotConfirmWhenNotEnough() {
        ledger.addIncome(kid, 100, mom, mom)
        goals.create(kid, "樂高", 600, kid)
        assertEquals(GoalResult.NotAllowed, goals.confirm(kid, goal("樂高").goal.id, kid))
    }

    @Test
    fun goalPayoutCompletesAndCheaperPriceUnlocksRest() {
        ledger.addIncome(kid, 3000, mom, mom)
        goals.create(kid, "腳踏車", 3000, kid)
        val id = goal("腳踏車").goal.id
        goals.confirm(kid, id, kid)

        assertEquals(GoalResult.OverLocked(3000), goals.payout(kid, id, 3480, mom, null))
        assertEquals(GoalResult.Ok, goals.payout(kid, id, 2800, mom, "特價"))

        val b = ledger.balances(kid)
        assertEquals(200, b.savings)
        assertEquals(0, b.locked)
        assertEquals("特價剩下的回到存款池", 200, b.available)
        assertEquals(2800, b.wallet)
        assertEquals(GoalStatus.COMPLETED, goals.states(kid).single().status)
    }

    @Test
    fun voidGoalPayoutReturnsToAchieved() {
        ledger.addIncome(kid, 3000, mom, mom)
        goals.create(kid, "腳踏車", 3000, kid)
        val id = goal("腳踏車").goal.id
        goals.confirm(kid, id, kid)
        goals.payout(kid, id, 3000, mom, null)
        val group = ledger.entries(kid).first { it.goal_id == id }.group_id

        assertEquals(LedgerResult.Ok, ledger.voidGroup(group, mom))
        assertEquals(GoalStatus.ACHIEVED, goals.states(kid).single().status)
        assertEquals(3000, ledger.balances(kid).locked)
    }

    @Test
    fun designatedMoneyAndDelete() {
        ledger.addIncome(kid, 500, mom, mom)
        goals.create(kid, "繪本", 800, kid)
        val id = goal("繪本").goal.id
        ledger.addIncome(kid, 500, grandma, mom, goalId = id)

        assertEquals(500, goal("繪本").state.let { goal("繪本").progress!!.designated })
        assertEquals("指定的錢不能一般出金（DQ-13）", 500, ledger.balances(kid).available)

        goals.delete(id)
        assertEquals("刪除後回到存款池（DQ-05）", 1000, ledger.balances(kid).available)
        assertTrue(goals.views(kid).isEmpty())
    }
}
