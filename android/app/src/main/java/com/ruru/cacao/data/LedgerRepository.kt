package com.ruru.cacao.data

import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.db.EntriesForChild
import com.ruru.cacao.domain.Account
import com.ruru.cacao.domain.BalanceCalculator
import com.ruru.cacao.domain.Balances
import com.ruru.cacao.domain.GoalState
import com.ruru.cacao.domain.LedgerLine
import java.time.Clock
import java.time.Instant
import java.time.LocalDate
import java.time.ZoneId
import java.util.UUID

data class Growth(val saved: Long, val tasksDone: Long, val goalsDone: Long)

/** 孩子「N 歲這一年」的區間：[生日＋N 年, 生日＋N＋1 年) */
fun ageYear(birthday: LocalDate, age: Int): Pair<LocalDate, LocalDate> =
    birthday.plusYears(age.toLong()) to birthday.plusYears(age.toLong() + 1)

sealed interface LedgerResult {
    data object Ok : LedgerResult
    data class OverAvailable(val available: Long) : LedgerResult
    data object WouldGoNegative : LedgerResult
    data object InvalidAmount : LedgerResult
}

class LedgerRepository(
    private val db: CacaoDatabase,
    private val clock: Clock = Clock.systemDefaultZone(),
    private val goals: (childId: String) -> List<GoalState> = { emptyList() },
) {
    private val queries get() = db.ledgerQueries

    fun balances(childId: String): Balances = BalanceCalculator.calculate(lines(childId), voidedGroups(childId), goals(childId))

    fun entries(childId: String): List<EntriesForChild> = queries.entriesForChild(childId).executeAsList()

    /** 記收入；現金留在孩子手上時同一組再記一筆出金到錢包（三筆） */
    fun addIncome(
        childId: String,
        amount: Long,
        sourceId: String,
        recordedBy: String,
        note: String? = null,
        message: String? = null,
        goalId: String? = null,
        cashToWallet: Boolean = false,
    ): LedgerResult {
        if (amount <= 0) return LedgerResult.InvalidAmount
        val group = UUID.randomUUID().toString()
        db.transaction {
            insert(group, childId, Account.SAVINGS, "income_onetime", amount, sourceId, goalId, note, message, recordedBy)
            if (cashToWallet) {
                insert(group, childId, Account.SAVINGS, "payout", -amount, null, null, note, null, recordedBy)
                insert(group, childId, Account.WALLET, "payout", amount, null, null, note, null, recordedBy)
            }
        }
        return LedgerResult.Ok
    }

    /** 出金：從存款扣、進錢包，不能超過可出金額（DQ-02、DQ-13） */
    fun payout(childId: String, amount: Long, recordedBy: String, note: String? = null, goalId: String? = null): LedgerResult {
        if (amount <= 0) return LedgerResult.InvalidAmount
        val available = balances(childId).available
        if (amount > available) return LedgerResult.OverAvailable(available)
        val group = UUID.randomUUID().toString()
        db.transaction {
            insert(group, childId, Account.SAVINGS, "payout", -amount, null, goalId, note, null, recordedBy)
            insert(group, childId, Account.WALLET, "payout", amount, null, goalId, note, null, recordedBy)
        }
        return LedgerResult.Ok
    }

    /** 錢包記花費；不擋超過餘額，錢包本來就不需要對上實際現金 */
    fun addWalletExpense(childId: String, amount: Long, recordedBy: String, note: String?): LedgerResult {
        if (amount <= 0) return LedgerResult.InvalidAmount
        insert(UUID.randomUUID().toString(), childId, Account.WALLET, "wallet_expense", -amount, null, null, note, null, recordedBy)
        return LedgerResult.Ok
    }

    /** 校正餘額：輸入實際剩多少，差額記成「未記錄支出」；沒有差額就不記 */
    fun calibrateWallet(childId: String, actual: Long, recordedBy: String): LedgerResult {
        if (actual < 0) return LedgerResult.InvalidAmount
        val diff = actual - balances(childId).wallet
        if (diff != 0L) insert(UUID.randomUUID().toString(), childId, Account.WALLET, "wallet_adjustment", diff, null, null, null, null, recordedBy)
        return LedgerResult.Ok
    }

    /** 成長紀錄（簡單版）：[from, to) 區間內存了多少、完成幾個任務、達成幾個目標 */
    fun growth(childId: String, from: LocalDate, to: LocalDate): Growth = Growth(
        saved = queries.incomeBetween(childId, from.toString(), to.toString()).executeAsOne(),
        tasksDone = queries.tasksBetween(childId, from.toString(), to.toString()).executeAsOne(),
        goalsDone = queries.goalsCompletedBetween(childId, from.toString(), to.toString()).executeAsOne(),
    )

    /** 任務獎勵入帳存款，來源是出題人；回傳 group_id */
    internal fun addTaskReward(childId: String, amount: Long, issuerId: String, verifiedBy: String, title: String): String {
        val group = UUID.randomUUID().toString()
        insert(group, childId, Account.SAVINGS, "income_task", amount, issuerId, null, title, null, verifiedBy)
        return group
    }

    /** 目標出金：額度檢查由 GoalRepository 負責（鎖住的錢），回傳 group_id */
    internal fun payoutForGoal(childId: String, amount: Long, recordedBy: String, note: String?, goalId: String): String {
        val group = UUID.randomUUID().toString()
        insert(group, childId, Account.SAVINGS, "payout", -amount, null, goalId, note, null, recordedBy)
        insert(group, childId, Account.WALLET, "payout", amount, null, goalId, note, null, recordedBy)
        return group
    }

    /** 作廢一整組；作廢後存款不能變負數（例如收入已經被出金用掉） */
    fun voidGroup(groupId: String, voidedBy: String): LedgerResult {
        val groupLines = queries.groupEntries(groupId).executeAsList()
        val childId = groupLines.firstOrNull()?.child_id ?: return LedgerResult.Ok
        val savingsChange = groupLines.filter { it.account == "savings" }.sumOf { it.amount }
        if (balances(childId).savings - savingsChange < 0) return LedgerResult.WouldGoNegative
        queries.voidGroup(UUID.randomUUID().toString(), groupId, voidedBy, now())
        return LedgerResult.Ok
    }

    private fun lines(childId: String): List<LedgerLine> = entries(childId).map {
        LedgerLine(it.group_id, Account.valueOf(it.account.uppercase()), it.kind, it.amount, it.goal_id)
    }

    private fun voidedGroups(childId: String): Set<String> = queries.voidedGroupsForChild(childId).executeAsList().toSet()

    private fun insert(
        group: String, childId: String, account: Account, kind: String, amount: Long,
        sourceId: String?, goalId: String?, note: String?, message: String?, recordedBy: String,
    ) {
        queries.insertEntry(
            id = UUID.randomUUID().toString(),
            group_id = group,
            child_id = childId,
            account = account.name.lowercase(),
            kind = kind,
            amount = amount,
            occurred_on = LocalDate.now(clock.withZone(ZoneId.systemDefault())).toString(),
            source_id = sourceId,
            goal_id = goalId,
            note = note?.takeIf { it.isNotBlank() },
            message = message?.takeIf { it.isNotBlank() },
            recorded_by = recordedBy,
            created_at = now(),
        )
    }

    private fun now(): String = Instant.now(clock).toString()
}
