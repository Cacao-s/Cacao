package com.ruru.cacao.data

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.ruru.cacao.db.CacaoDatabase
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test

class TaskRepositoryTest {

    private lateinit var tasks: TaskRepository
    private lateinit var ledger: LedgerRepository
    private lateinit var mom: String
    private lateinit var grandpa: String
    private lateinit var ann: String
    private lateinit var ben: String

    @Before
    fun setUp() {
        val driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        CacaoDatabase.Schema.create(driver)
        val db = CacaoDatabase(driver)
        val members = MemberRepository(db)
        mom = members.createParent("媽媽", "1234").first
        grandpa = members.createElder("爺爺", mom)
        ann = members.createChild("小安", "2017-03-12")
        ben = members.createChild("小明", "2021-08-05")
        ledger = LedgerRepository(db)
        tasks = TaskRepository(db, ledger)
    }

    @Test
    fun oneTaskPerChild() {
        assertEquals(TaskResult.Ok, tasks.create("整理房間", 50, mom, listOf(ann, ben), mom))
        assertEquals(1, tasks.pending(ann).size)
        assertEquals(1, tasks.pending(ben).size)
        assertEquals(1, tasks.pendingAll().map { it.batch_id }.distinct().size)
    }

    @Test
    fun completeAddsRewardFromIssuer() {
        tasks.create("讀一本書", 100, grandpa, listOf(ann, ben), mom)
        val annTask = tasks.pending(ann).single()

        assertEquals(TaskResult.Ok, tasks.complete(annTask.id, mom))
        assertEquals(100, ledger.balances(ann).savings)
        assertEquals("小明那份不受影響", 0, ledger.balances(ben).savings)
        assertEquals(grandpa, ledger.entries(ann).single().source_id)
        assertEquals("income_task", ledger.entries(ann).single().kind)
        assertEquals(0, tasks.pending(ann).size)
        assertEquals(mom, tasks.completed(ann).single().verified_by)

        assertEquals("不能重複入帳", TaskResult.AlreadyDone, tasks.complete(annTask.id, mom))
        assertEquals(100, ledger.balances(ann).savings)
    }

    @Test
    fun invalidAndDelete() {
        assertEquals(TaskResult.Invalid, tasks.create("", 50, mom, listOf(ann), mom))
        assertEquals(TaskResult.Invalid, tasks.create("洗碗", 0, mom, listOf(ann), mom))
        assertEquals(TaskResult.Invalid, tasks.create("洗碗", 30, mom, emptyList(), mom))

        tasks.create("洗碗", 30, mom, listOf(ann), mom)
        val id = tasks.pending(ann).single().id
        assertEquals(TaskResult.Ok, tasks.delete(id))
        assertEquals(0, tasks.pending(ann).size)
        assertEquals("刪除的任務不能完成", TaskResult.Invalid, tasks.complete(id, mom))
    }

    @Test
    fun childProposalIsZeroUntilParentSetsReward() {
        assertEquals(TaskResult.Invalid, tasks.propose(ann, " "))
        assertEquals(TaskResult.Ok, tasks.propose(ann, "洗碗"))
        val p = tasks.proposals(ann).single()
        assertEquals("提議不算任務，不能完成", 0, tasks.pending(ann).size)

        assertEquals(TaskResult.Invalid, tasks.price(p.id, 0, mom))
        assertEquals(TaskResult.Ok, tasks.price(p.id, 30, mom))
        assertEquals(0, tasks.proposals(ann).size)
        val task = tasks.pending(ann).single()
        assertEquals(30, task.reward_amount)
        assertEquals(mom, task.issuer_id)
        assertEquals("建立者是提議的孩子", ann, task.created_by)
        assertEquals("同一份提議不能再設一次", TaskResult.Invalid, tasks.price(p.id, 30, mom))
    }

    @Test
    fun deletedProposalDisappears() {
        tasks.propose(ben, "澆花")
        tasks.deleteProposal(tasks.proposals(ben).single().id)
        assertEquals(0, tasks.proposalsAll().size)
    }
}
