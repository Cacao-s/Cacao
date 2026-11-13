package com.ruru.cacao.data

import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.db.Goal
import com.ruru.cacao.domain.GoalProgress
import com.ruru.cacao.domain.GoalState
import com.ruru.cacao.domain.GoalStatus
import java.time.Clock
import java.time.Instant
import java.util.UUID

sealed interface GoalResult {
    data object Ok : GoalResult
    data object NotAllowed : GoalResult
    data object InvalidAmount : GoalResult
    data class OverLocked(val locked: Long) : GoalResult
}

/** 畫面用：目標本身＋狀態＋瀑布填充結果 */
data class GoalView(val goal: Goal, val state: GoalState, val progress: GoalProgress?) {
    val achievable: Boolean get() = state.status == GoalStatus.IN_PROGRESS && progress?.achievable == true
    val saved: Long get() = when (state.status) {
        GoalStatus.IN_PROGRESS -> (progress?.designated ?: 0) + (progress?.filled ?: 0)
        GoalStatus.ACHIEVED -> state.lockedAmount
        GoalStatus.COMPLETED -> goal.target_amount
    }
}

class GoalRepository(
    private val db: CacaoDatabase,
    private val clock: Clock = Clock.systemUTC(),
) {
    private val queries get() = db.goalQueries
    lateinit var ledger: LedgerRepository

    /** 從事件推算每個目標的狀態；出金被作廢的「完成」不算（DQ-07） */
    fun states(childId: String, includeDeleted: Boolean = true): List<GoalState> {
        val voided = queries.voidedGroups().executeAsList().toSet()
        val events = queries.eventsForChild(childId).executeAsList().groupBy { it.goal_id }
        val goals = if (includeDeleted) queries.allForChild(childId).executeAsList() else queries.forChild(childId).executeAsList()
        return goals.map { goal ->
            val statusEvents = events[goal.id].orEmpty()
                .filter { it.kind != "target_changed" }
                .filterNot { it.kind == "completed" && it.ledger_group_id in voided }
            val last = statusEvents.lastOrNull()
            val status = when (last?.kind) {
                "confirmed" -> GoalStatus.ACHIEVED
                "completed" -> GoalStatus.COMPLETED
                else -> GoalStatus.IN_PROGRESS
            }
            val locked = if (status == GoalStatus.ACHIEVED) statusEvents.last { it.kind == "confirmed" }.amount ?: 0 else 0
            GoalState(goal.id, goal.target_amount, goal.priority, goal.fill_enabled == 1L, status, locked, goal.deleted_at != null)
        }
    }

    fun views(childId: String): List<GoalView> {
        val progress = ledger.balances(childId).goals
        val states = states(childId, includeDeleted = false).associateBy { it.id }
        return queries.forChild(childId).executeAsList().map { GoalView(it, states.getValue(it.id), progress[it.id]) }
    }

    fun view(childId: String, goalId: String): GoalView? = views(childId).firstOrNull { it.goal.id == goalId }

    fun create(childId: String, name: String, target: Long, createdBy: String): GoalResult {
        if (target <= 0 || name.isBlank()) return GoalResult.InvalidAmount
        val last = queries.forChild(childId).executeAsList().maxOfOrNull { it.priority } ?: 0.0
        val time = now()
        queries.insert(UUID.randomUUID().toString(), childId, name.trim(), target, last + 1, createdBy, time, time)
        return GoalResult.Ok
    }

    /** 孩子確認：只有「可達成」才能確認，鎖住當時的目標金額 */
    fun confirm(childId: String, goalId: String, childMemberId: String): GoalResult {
        val v = view(childId, goalId) ?: return GoalResult.NotAllowed
        if (!v.achievable) return GoalResult.NotAllowed
        event(goalId, "confirmed", v.goal.target_amount, null, childMemberId)
        return GoalResult.Ok
    }

    /** 孩子反悔：已達成 → 進行中，解鎖 */
    fun revert(childId: String, goalId: String, memberId: String): GoalResult {
        val v = view(childId, goalId) ?: return GoalResult.NotAllowed
        if (v.state.status != GoalStatus.ACHIEVED) return GoalResult.NotAllowed
        event(goalId, "reverted", null, null, memberId)
        return GoalResult.Ok
    }

    fun changeTarget(childId: String, goalId: String, target: Long, memberId: String): GoalResult {
        if (target <= 0) return GoalResult.InvalidAmount
        val v = view(childId, goalId) ?: return GoalResult.NotAllowed
        if (v.state.status != GoalStatus.IN_PROGRESS) return GoalResult.NotAllowed
        queries.updateTarget(target, now(), goalId)
        event(goalId, "target_changed", target, null, memberId)
        return GoalResult.Ok
    }

    fun setFill(goalId: String, enabled: Boolean) {
        queries.updateFill(if (enabled) 1 else 0, now(), goalId)
    }

    /** 往前或往後移一格（與相鄰目標交換優先順序） */
    /** 拖曳排序後整份寫回：priority 依清單順序 1、2、3… */
    fun reorder(childId: String, goalIds: List<String>) {
        val current = queries.forChild(childId).executeAsList().map { it.id }
        if (goalIds.toSet() != current.toSet()) return
        val time = now()
        db.transaction {
            goalIds.forEachIndexed { index, id -> queries.updatePriority((index + 1).toDouble(), time, id) }
        }
    }

    /** 刪除：錢回到存款池（DQ-05）；已完成的只是從清單隱藏 */
    fun delete(goalId: String) {
        val time = now()
        queries.softDelete(time, time, goalId)
    }

    /** 目標出金：實際金額不能超過鎖住的錢；特價時剩下的自動解鎖 */
    fun payout(childId: String, goalId: String, amount: Long, parentId: String, note: String?): GoalResult {
        if (amount <= 0) return GoalResult.InvalidAmount
        val v = view(childId, goalId) ?: return GoalResult.NotAllowed
        if (v.state.status != GoalStatus.ACHIEVED) return GoalResult.NotAllowed
        if (amount > v.state.lockedAmount) return GoalResult.OverLocked(v.state.lockedAmount)
        db.transaction {
            val group = ledger.payoutForGoal(childId, amount, parentId, note ?: v.goal.name, goalId)
            event(goalId, "completed", amount, group, parentId)
        }
        return GoalResult.Ok
    }

    private fun event(goalId: String, kind: String, amount: Long?, groupId: String?, memberId: String) {
        queries.insertEvent(UUID.randomUUID().toString(), goalId, kind, amount, groupId, memberId, now())
    }

    private fun now(): String = Instant.now(clock).toString()
}
