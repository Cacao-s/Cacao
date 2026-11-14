package com.ruru.cacao.data

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.ruru.cacao.db.CacaoDatabase
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test

class LedgerRepositoryTest {

    private lateinit var ledger: LedgerRepository
    private lateinit var momId: String
    private lateinit var grandpaId: String
    private lateinit var childId: String

    @Before
    fun setUp() {
        val driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        CacaoDatabase.Schema.create(driver)
        val db = CacaoDatabase(driver)
        val members = MemberRepository(db)
        momId = members.createParent("媽媽", "1234").first
        grandpaId = members.createElder("爺爺", momId)
        childId = members.createChild("小安", "2017-03-12")
        ledger = LedgerRepository(db)
    }

    @Test
    fun incomeAndPayout() {
        assertEquals(LedgerResult.Ok, ledger.addIncome(childId, 500, grandpaId, momId, note = "紅包", message = "新年快樂"))
        assertEquals(LedgerResult.Ok, ledger.payout(childId, 300, momId, note = "校外教學"))

        val b = ledger.balances(childId)
        assertEquals(200, b.savings)
        assertEquals(300, b.wallet)
        assertEquals(200, b.available)
        assertEquals(3, ledger.entries(childId).size)
    }

    @Test
    fun payoutCannotExceedAvailable() {
        ledger.addIncome(childId, 500, momId, momId)
        assertEquals(LedgerResult.OverAvailable(500), ledger.payout(childId, 501, momId))
        assertEquals(LedgerResult.InvalidAmount, ledger.payout(childId, 0, momId))
        assertEquals(500, ledger.balances(childId).savings)
    }

    @Test
    fun cashStaysWithChild() {
        ledger.addIncome(childId, 1000, grandpaId, momId, cashToWallet = true)
        val b = ledger.balances(childId)
        assertEquals(0, b.savings)
        assertEquals(1000, b.wallet)
        assertEquals("同一組三筆", 1, ledger.entries(childId).map { it.group_id }.distinct().size)
    }

    @Test
    fun voidPayoutUndoesBothSides() {
        ledger.addIncome(childId, 500, momId, momId)
        ledger.payout(childId, 300, momId)
        val payoutGroup = ledger.entries(childId).first { it.kind == "payout" }.group_id

        assertEquals(LedgerResult.Ok, ledger.voidGroup(payoutGroup, momId))
        val b = ledger.balances(childId)
        assertEquals(500, b.savings)
        assertEquals(0, b.wallet)
        assertEquals(2, ledger.entries(childId).count { it.voided > 0 })
    }

    @Test
    fun voidCannotMakeSavingsNegative() {
        ledger.addIncome(childId, 500, momId, momId)
        val incomeGroup = ledger.entries(childId).single().group_id
        ledger.payout(childId, 300, momId)

        assertEquals(LedgerResult.WouldGoNegative, ledger.voidGroup(incomeGroup, momId))
        assertEquals(200, ledger.balances(childId).savings)
    }
}
