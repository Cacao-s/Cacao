package com.ruru.cacao.db

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Before
import org.junit.Test

class SchemaTest {

    private lateinit var db: CacaoDatabase
    private val now = "2026-11-02T00:00:00Z"

    @Before
    fun setUp() {
        val driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        driver.execute(null, "PRAGMA foreign_keys = ON", 0)
        CacaoDatabase.Schema.create(driver)
        db = CacaoDatabase(driver)
        db.ledgerQueries.insertMember("p1", "parent", "媽媽", null, "brown", "\$argon2id\$x", now, now)
        db.ledgerQueries.insertMember("c1", "child", "小安", "2017-03-12", "peach", null, now, now)
    }

    @Test
    fun balanceSkipsVoidedGroups() {
        val q = db.ledgerQueries
        q.insertEntry("e1", "e1", "c1", "savings", "income_onetime", 500, "2026-11-01", "p1", null, null, null, "p1", now)
        q.insertEntry("e2", "e2", "c1", "savings", "income_onetime", 5000, "2026-11-01", "p1", null, null, null, "p1", now)
        q.voidGroup("v1", "e2", "p1", now)

        assertEquals(500L, q.balance("c1", "savings").executeAsOne())
        assertEquals(0L, q.balance("c1", "wallet").executeAsOne())
    }

    @Test
    fun childNeedsBirthday() {
        assertThrows(Exception::class.java) {
            db.ledgerQueries.insertMember("c2", "child", "小明", null, "mint", null, now, now)
        }
    }

    @Test
    fun amountCannotBeZero() {
        assertThrows(Exception::class.java) {
            db.ledgerQueries.insertEntry("e3", "e3", "c1", "wallet", "wallet_expense", 0, "2026-11-01", null, null, null, null, "c1", now)
        }
    }
}
