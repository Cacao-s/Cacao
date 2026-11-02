package com.ruru.cacao.domain

import java.io.File
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import org.junit.Assert.assertEquals
import org.junit.Test

class SharedBalanceCasesTest {

    @Serializable
    private data class CaseFile(val cases: List<Case>)

    @Serializable
    private data class Case(
        val name: String,
        val entries: List<Entry>,
        val voided: List<String>,
        val goals: List<Goal>,
        val expected: Expected,
    )

    @Serializable
    private data class Entry(val group: String, val account: String, val kind: String, val amount: Long, val goal: String? = null)

    @Serializable
    private data class Goal(
        val id: String,
        val target: Long,
        val priority: Double,
        val fillEnabled: Boolean,
        val status: String,
        val locked: Long = 0,
        val deleted: Boolean = false,
    )

    @Serializable
    private data class Expected(
        val savings: Long,
        val wallet: Long,
        val locked: Long,
        val available: Long,
        val goals: Map<String, ExpectedGoal>,
    )

    @Serializable
    private data class ExpectedGoal(val designated: Long, val filled: Long, val achievable: Boolean)

    @Test
    fun sharedCases() {
        val sharedDir = System.getProperty("cacao.sharedDir") ?: error("cacao.sharedDir 沒有設定")
        val file = File(sharedDir, "test-cases/balances.json")
        val cases = Json { ignoreUnknownKeys = true }.decodeFromString<CaseFile>(file.readText()).cases

        cases.forEach { case ->
            val result = BalanceCalculator.calculate(
                lines = case.entries.map {
                    LedgerLine(it.group, Account.valueOf(it.account.uppercase()), it.kind, it.amount, it.goal)
                },
                voidedGroups = case.voided.toSet(),
                goals = case.goals.map {
                    GoalState(it.id, it.target, it.priority, it.fillEnabled, GoalStatus.valueOf(it.status.uppercase()), it.locked, it.deleted)
                },
            )
            val expected = case.expected
            assertEquals("${case.name}：存款", expected.savings, result.savings)
            assertEquals("${case.name}：錢包", expected.wallet, result.wallet)
            assertEquals("${case.name}：已鎖", expected.locked, result.locked)
            assertEquals("${case.name}：可出金額", expected.available, result.available)
            assertEquals(
                "${case.name}：目標進度",
                expected.goals.mapValues { GoalProgress(it.value.designated, it.value.filled, it.value.achievable) },
                result.goals,
            )
        }
    }
}
