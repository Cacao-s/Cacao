package com.ruru.cacao.data

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.ruru.cacao.db.CacaoDatabase
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test

class MemberRepositoryTest {

    private lateinit var repo: MemberRepository
    private lateinit var db: CacaoDatabase
    private lateinit var driver: JdbcSqliteDriver

    @Before
    fun setUp() {
        driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        CacaoDatabase.Schema.create(driver)
        db = CacaoDatabase(driver)
        repo = MemberRepository(db)
    }

    @Test
    fun onboardingCreatesFamily() {
        assertFalse(repo.hasFamily())
        val (momId, code) = repo.createParent("媽媽", "1234")
        repo.createChild("小安", "2017-03-12")

        assertTrue(repo.hasFamily())
        assertEquals(listOf("媽媽", "小安"), repo.members().map { it.name })
        assertTrue(repo.verifyPin(momId, "1234"))
        assertFalse(repo.verifyPin(momId, "0000"))
        assertEquals(19, code.length)
    }

    @Test
    fun recoveryCodeResetsPinOnce() {
        val (momId, code) = repo.createParent("媽媽", "1234")

        assertNull(repo.resetPinByRecovery(momId, "WRONG-CODE", "5678"))
        val newCode = repo.resetPinByRecovery(momId, code.lowercase(), "5678")
        assertNotNull(newCode)
        assertTrue(repo.verifyPin(momId, "5678"))
        assertFalse(repo.verifyPin(momId, "1234"))
        assertNull("用過的救援碼不能再用", repo.resetPinByRecovery(momId, code, "9999"))
        assertNotNull(repo.resetPinByRecovery(momId, newCode!!, "9999"))
    }

    @Test
    fun otherParentResetsPin() {
        val (momId, _) = repo.createParent("媽媽", "1234")
        val (dadId, _) = repo.createParent("爸爸", "4321")

        assertNull("另一位父母 PIN 錯誤", repo.resetPinByOtherParent(momId, dadId, "0000", "5678"))
        assertNull("不能自己幫自己", repo.resetPinByOtherParent(momId, momId, "1234", "5678"))
        assertNotNull(repo.resetPinByOtherParent(momId, dadId, "4321", "5678"))
        assertTrue(repo.verifyPin(momId, "5678"))
    }

    @Test
    fun anyParentPinUnlocksParentMode() {
        repo.createParent("媽媽", "1234")
        repo.createParent("爸爸", "4321")
        assertEquals("爸爸", repo.parentForPin("4321")?.name)
        assertNull(repo.parentForPin("0000"))
    }

    @Test
    fun elderBelongsToParentWhoAddedIt() {
        val (momId, _) = repo.createParent("媽媽", "1234")
        val (dadId, _) = repo.createParent("爸爸", "4321")
        val grandma = repo.createElder("外婆", momId)
        val nana = repo.createElder("奶奶", dadId)
        assertEquals(mapOf(grandma to momId, nana to dadId), repo.elderSides())
    }

    @Test
    fun unassignedElderCanBeClaimed() {
        val (momId, _) = repo.createParent("媽媽", "1234")
        val (dadId, _) = repo.createParent("爸爸", "4321")
        val auntie = repo.createElder("乾媽", momId)
        driver.execute(null, "DELETE FROM elder_side WHERE elder_id = '$auntie'", 0)
        assertNull(repo.elderSides()[auntie])

        repo.setElderSide(auntie, dadId)
        assertEquals(dadId, repo.elderSides()[auntie])
    }

    @Test
    fun childPasswordIsOptional() {
        val childId = repo.createChild("小明", "2021-08-05")
        assertTrue("沒設密碼就直接進", repo.verifyChildPassword(childId, ""))

        repo.setChildPassword(childId, "2468")
        assertTrue(repo.verifyChildPassword(childId, "2468"))
        assertFalse(repo.verifyChildPassword(childId, "1111"))

        repo.setChildPassword(childId, null)
        assertTrue(repo.verifyChildPassword(childId, ""))
    }
}
