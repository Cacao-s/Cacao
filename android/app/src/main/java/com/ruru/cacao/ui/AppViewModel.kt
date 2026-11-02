package com.ruru.cacao.ui

import android.app.Application
import android.content.Context
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.AndroidViewModel
import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.ruru.cacao.data.LedgerRepository
import com.ruru.cacao.data.MemberRepository
import com.ruru.cacao.data.Role
import com.ruru.cacao.data.roleEnum
import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.db.Member
import com.ruru.cacao.security.AttemptLimiter
import com.ruru.cacao.security.AttemptState

sealed interface PinPurpose {
    val limiterKey: String

    data class Parent(val parentId: String) : PinPurpose { override val limiterKey = parentId }
    data object AnyParent : PinPurpose { override val limiterKey = "any_parent" }
    data class Child(val childId: String) : PinPurpose { override val limiterKey = childId }
}

sealed interface Screen {
    data object Onboarding : Screen
    data object SelectUser : Screen
    data class Pin(val purpose: PinPurpose) : Screen
    data class ParentHome(val parentId: String) : Screen
    data class ChildHome(val childId: String) : Screen
    data class ForgotPin(val parentId: String) : Screen
    data class ChildPassword(val childId: String) : Screen
    data class ChildOverview(val parentId: String, val childId: String) : Screen
    data class AddIncome(val parentId: String, val childId: String) : Screen
    data class Payout(val parentId: String, val childId: String) : Screen
    data class Ledger(val parentId: String, val childId: String) : Screen
}

val Screen.isParentMode: Boolean
    get() = this is Screen.ParentHome || this is Screen.ForgotPin || this is Screen.ChildOverview ||
        this is Screen.AddIncome || this is Screen.Payout || this is Screen.Ledger

class AppViewModel(application: Application) : AndroidViewModel(application) {

    private val db = CacaoDatabase(AndroidSqliteDriver(CacaoDatabase.Schema, application, "cacao.db"))
    val members = MemberRepository(db)
    val ledger = LedgerRepository(db)

    /** 每次記帳後遞增，讓畫面重新讀餘額 */
    var ledgerVersion by mutableStateOf(0)
        private set

    fun ledgerChanged() {
        ledgerVersion++
    }
    private val prefs = application.getSharedPreferences("cacao_device", Context.MODE_PRIVATE)
    private var backgroundSince = 0L

    var screen: Screen by mutableStateOf(if (members.hasFamily()) Screen.SelectUser else Screen.Onboarding)
        private set

    var memberList: List<Member> by mutableStateOf(members.members())
        private set

    fun go(target: Screen) {
        memberList = members.members()
        screen = target
    }

    fun member(id: String): Member? = memberList.firstOrNull { it.id == id }

    fun selectMember(member: Member) {
        when (member.roleEnum) {
            Role.PARENT -> go(Screen.Pin(PinPurpose.Parent(member.id)))
            Role.CHILD -> go(if (member.password_hash == null) Screen.ChildHome(member.id) else Screen.Pin(PinPurpose.Child(member.id)))
            Role.ELDER -> Unit
        }
    }

    fun attemptState(purpose: PinPurpose): AttemptState = AttemptState(
        failures = prefs.getInt("fail_${purpose.limiterKey}", 0),
        lockedUntilMillis = prefs.getLong("lock_${purpose.limiterKey}", 0),
    )

    /** 回傳 true 表示通過並已切換畫面 */
    fun submitPin(purpose: PinPurpose, pin: String, nowMillis: Long = System.currentTimeMillis()): Boolean {
        if (AttemptLimiter.remainingSeconds(attemptState(purpose), nowMillis) > 0) return false
        val next: Screen? = when (purpose) {
            is PinPurpose.Parent -> if (members.verifyPin(purpose.parentId, pin)) Screen.ParentHome(purpose.parentId) else null
            PinPurpose.AnyParent -> members.parentForPin(pin)?.let { Screen.ParentHome(it.id) }
            is PinPurpose.Child -> if (members.verifyChildPassword(purpose.childId, pin)) Screen.ChildHome(purpose.childId) else null
        }
        if (next == null) {
            save(purpose, AttemptLimiter.onFailure(attemptState(purpose), nowMillis))
            return false
        }
        save(purpose, AttemptState())
        go(next)
        return true
    }

    fun lockParentMode() {
        if (screen.isParentMode) go(Screen.SelectUser)
    }

    fun onBackground(nowMillis: Long) {
        backgroundSince = nowMillis
    }

    /** 父母模式切到背景超過 1 分鐘就鎖回去（auth-design §1） */
    fun onForeground(nowMillis: Long) {
        if (backgroundSince > 0 && nowMillis - backgroundSince > 60_000) lockParentMode()
        backgroundSince = 0
    }

    private fun save(purpose: PinPurpose, state: AttemptState) {
        prefs.edit()
            .putInt("fail_${purpose.limiterKey}", state.failures)
            .putLong("lock_${purpose.limiterKey}", state.lockedUntilMillis)
            .apply()
    }
}
