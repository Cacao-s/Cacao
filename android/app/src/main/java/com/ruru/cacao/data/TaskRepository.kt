package com.ruru.cacao.data

import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.db.CompletedForChild
import com.ruru.cacao.db.Task
import com.ruru.cacao.db.Task_proposal
import java.time.Clock
import java.time.Instant
import java.util.UUID

sealed interface TaskResult {
    data object Ok : TaskResult
    data object Invalid : TaskResult
    data object AlreadyDone : TaskResult
}

class TaskRepository(
    private val db: CacaoDatabase,
    private val ledger: LedgerRepository,
    private val clock: Clock = Clock.systemUTC(),
) {
    private val queries get() = db.taskQueries

    /** 勾選幾個孩子就產生幾份，各自完成、各自驗收（DQ-08） */
    fun create(title: String, reward: Long, issuerId: String, childIds: List<String>, createdBy: String): TaskResult {
        if (title.isBlank() || reward <= 0 || childIds.isEmpty()) return TaskResult.Invalid
        val batch = UUID.randomUUID().toString()
        val time = now()
        db.transaction {
            childIds.distinct().forEach { childId ->
                queries.insert(UUID.randomUUID().toString(), batch, childId, title.trim(), reward, issuerId, createdBy, time, time)
            }
        }
        return TaskResult.Ok
    }

    fun pending(childId: String): List<Task> = queries.pendingForChild(childId).executeAsList()

    fun pendingAll(): List<Task> = queries.pendingAll().executeAsList()

    fun completed(childId: String): List<CompletedForChild> = queries.completedForChild(childId).executeAsList()

    fun task(id: String): Task? = queries.byId(id).executeAsOneOrNull()

    /** 父母當場驗收通過：寫完成紀錄＋獎勵入帳，來源是出題人（DQ-09） */
    fun complete(taskId: String, verifiedBy: String): TaskResult {
        val task = task(taskId) ?: return TaskResult.Invalid
        if (task.deleted_at != null) return TaskResult.Invalid
        if (queries.isCompleted(taskId).executeAsOne() > 0) return TaskResult.AlreadyDone
        db.transaction {
            val group = ledger.addTaskReward(task.child_id, task.reward_amount, task.issuer_id, verifiedBy, task.title)
            queries.insertCompletion(UUID.randomUUID().toString(), taskId, verifiedBy, group, now())
        }
        return TaskResult.Ok
    }

    fun delete(taskId: String): TaskResult {
        if (queries.isCompleted(taskId).executeAsOne() > 0) return TaskResult.AlreadyDone
        val time = now()
        queries.softDelete(time, time, taskId)
        return TaskResult.Ok
    }

    /** 孩子提議：0 元，等父母設金額（discover WD-35） */
    fun propose(childId: String, title: String): TaskResult {
        if (title.isBlank()) return TaskResult.Invalid
        val time = now()
        queries.insertProposal(UUID.randomUUID().toString(), childId, title.trim(), time, time)
        return TaskResult.Ok
    }

    fun proposals(childId: String): List<Task_proposal> = queries.proposalsForChild(childId).executeAsList()

    fun proposalsAll(): List<Task_proposal> = queries.proposalsAll().executeAsList()

    /** 父母設金額＝同意：轉成正式任務，提議收起來 */
    fun price(proposalId: String, reward: Long, issuerId: String): TaskResult {
        val p = queries.proposalById(proposalId).executeAsOneOrNull() ?: return TaskResult.Invalid
        if (p.deleted_at != null || reward <= 0) return TaskResult.Invalid
        val time = now()
        db.transaction {
            queries.insert(UUID.randomUUID().toString(), UUID.randomUUID().toString(), p.child_id, p.title, reward, issuerId, p.child_id, time, time)
            queries.softDeleteProposal(time, time, proposalId)
        }
        return TaskResult.Ok
    }

    fun deleteProposal(proposalId: String) {
        val time = now()
        queries.softDeleteProposal(time, time, proposalId)
    }

    private fun now(): String = Instant.now(clock).toString()
}
