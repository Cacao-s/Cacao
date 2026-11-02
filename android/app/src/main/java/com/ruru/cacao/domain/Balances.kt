package com.ruru.cacao.domain

enum class Account { SAVINGS, WALLET }

enum class GoalStatus { IN_PROGRESS, ACHIEVED, COMPLETED }

data class LedgerLine(
    val groupId: String,
    val account: Account,
    val kind: String,
    val amount: Long,
    val goalId: String? = null,
)

data class GoalState(
    val id: String,
    val target: Long,
    val priority: Double,
    val fillEnabled: Boolean,
    val status: GoalStatus,
    val lockedAmount: Long = 0,
    val deleted: Boolean = false,
)

data class GoalProgress(
    val designated: Long,
    val filled: Long,
    val achievable: Boolean,
)

data class Balances(
    val savings: Long,
    val wallet: Long,
    val locked: Long,
    val available: Long,
    val goals: Map<String, GoalProgress>,
)

object BalanceCalculator {

    fun calculate(lines: List<LedgerLine>, voidedGroups: Set<String>, goals: List<GoalState>): Balances {
        val valid = lines.filter { it.groupId !in voidedGroups }
        val savings = valid.filter { it.account == Account.SAVINGS }.sumOf { it.amount }
        val wallet = valid.filter { it.account == Account.WALLET }.sumOf { it.amount }

        val activeGoals = goals.filter { !it.deleted }
        val locked = activeGoals.filter { it.status == GoalStatus.ACHIEVED }.sumOf { it.lockedAmount }

        val inProgress = activeGoals
            .filter { it.status == GoalStatus.IN_PROGRESS }
            .sortedBy { it.priority }

        val designatedByGoal = inProgress.associate { goal ->
            val received = valid
                .filter { it.account == Account.SAVINGS && it.kind.startsWith("income_") && it.goalId == goal.id }
                .sumOf { it.amount }
            goal.id to minOf(received, goal.target)
        }

        val available = maxOf(0L, savings - locked - designatedByGoal.values.sum())

        var pool = available
        val progress = inProgress.associate { goal ->
            val designated = designatedByGoal.getValue(goal.id)
            val filled = if (goal.fillEnabled) minOf(goal.target - designated, pool) else 0L
            pool -= filled
            goal.id to GoalProgress(designated, filled, designated + filled >= goal.target)
        }

        return Balances(savings, wallet, locked, available, progress)
    }
}
