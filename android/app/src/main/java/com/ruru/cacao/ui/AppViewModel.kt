package com.ruru.cacao.ui

import android.app.Application
import android.content.Context
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.ruru.cacao.backup.BackupManager
import com.ruru.cacao.backup.DriveClient
import com.ruru.cacao.backup.DriveFile
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import com.ruru.cacao.data.AllowanceRepository
import com.ruru.cacao.data.GoalRepository
import com.ruru.cacao.data.LedgerRepository
import com.ruru.cacao.data.MemberRepository
import com.ruru.cacao.data.TaskRepository
import com.ruru.cacao.data.TaskResult
import com.ruru.cacao.ui.components.Celebration
import com.ruru.cacao.ui.ledger.money
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

    /** 孩子按任務「完成」：任一位父母當場驗收（DQ-09） */
    data class VerifyTask(val taskId: String, val childId: String) : PinPurpose { override val limiterKey = "any_parent" }
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

    /** 目標畫面父母與孩子共用（UQ-05）；parentId 為 null 表示孩子模式 */
    data class Goals(val childId: String, val parentId: String?) : Screen
    data class GoalDetail(val childId: String, val goalId: String, val parentId: String?) : Screen
    data class NewGoal(val childId: String, val parentId: String?) : Screen
    data class Tasks(val parentId: String) : Screen
    data class NewTask(val parentId: String) : Screen
    data class ChildTasks(val childId: String) : Screen
    data class Allowance(val parentId: String, val childId: String) : Screen

    /** 錢包、成長紀錄父母與孩子共用；parentId 為 null 表示孩子模式 */
    data class Wallet(val childId: String, val parentId: String?) : Screen
    data class Growth(val childId: String, val parentId: String?) : Screen
    data class Settings(val parentId: String) : Screen
    data class Family(val parentId: String) : Screen
    data class Backup(val parentId: String) : Screen
}

val Screen.isParentMode: Boolean
    get() = when (this) {
        is Screen.ParentHome, is Screen.ForgotPin, is Screen.ChildOverview,
        is Screen.AddIncome, is Screen.Payout, is Screen.Ledger, is Screen.Tasks, is Screen.NewTask, is Screen.Allowance,
        is Screen.Settings, is Screen.Family, is Screen.Backup -> true
        is Screen.Goals -> parentId != null
        is Screen.GoalDetail -> parentId != null
        is Screen.NewGoal -> parentId != null
        is Screen.Wallet -> parentId != null
        is Screen.Growth -> parentId != null
        else -> false
    }

class AppViewModel(application: Application) : AndroidViewModel(application) {

    private val driver = AndroidSqliteDriver(CacaoDatabase.Schema, application, BackupManager.DB_NAME)
    private val db = CacaoDatabase(driver)
    val backup = BackupManager(application, driver)

    /** 由 MainActivity 提供：向 Google 取得 drive.appdata 授權；interactive=false 時不跳畫面 */
    var cloudAuth: ((interactive: Boolean, onToken: (String) -> Unit, onError: (String) -> Unit) -> Unit)? = null

    var cloudEnabled: Boolean by mutableStateOf(false)
        private set
    var lastBackupMillis: Long by mutableStateOf(0L)
        private set
    var cloudStatus: String? by mutableStateOf(null)
        private set
    var cloudFiles: List<DriveFile> by mutableStateOf(emptyList())
        private set

    private fun markBackedUp() {
        lastBackupMillis = System.currentTimeMillis()
        prefs.edit().putLong("last_backup", lastBackupMillis).apply()
    }

    fun markExported() = markBackedUp()

    /** 開啟雲端備份：授權後立刻備份一次 */
    fun enableCloud() = withToken(interactive = true) { token ->
        prefs.edit().putBoolean("cloud", true).apply()
        cloudEnabled = true
        uploadNow(token)
    }

    fun disableCloud() {
        prefs.edit().putBoolean("cloud", false).apply()
        cloudEnabled = false
        cloudFiles = emptyList()
    }

    fun backupNow() = withToken(interactive = true) { uploadNow(it) }

    fun loadCloudFiles(interactive: Boolean = true) = withToken(interactive) { token ->
        viewModelScope.launch(Dispatchers.IO) {
            runCatching { DriveClient(token).list() }
                .onSuccess { cloudFiles = it; cloudStatus = null }
                .onFailure { cloudStatus = it.message }
        }
    }

    fun downloadCloud(file: DriveFile, onBytes: (ByteArray) -> Unit) = withToken(interactive = true) { token ->
        viewModelScope.launch(Dispatchers.IO) {
            runCatching { DriveClient(token).download(file.id) }
                .onSuccess { bytes -> viewModelScope.launch { onBytes(bytes) } }
                .onFailure { cloudStatus = it.message }
        }
    }

    /** 已開啟雲端備份且超過 30 天沒備份 → 背景自動備份一次；沒網路就等下次開 App */
    private fun autoBackupIfDue() {
        if (!cloudEnabled) return
        if (System.currentTimeMillis() - lastBackupMillis < 30L * 24 * 60 * 60 * 1000) return
        withToken(interactive = false) { uploadNow(it) }
    }

    private fun uploadNow(token: String) {
        viewModelScope.launch(Dispatchers.IO) {
            runCatching {
                val client = DriveClient(token)
                client.upload("cacao-${java.time.LocalDateTime.now().toString().replace(":", "")}.cacao", backup.snapshot())
                client.prune(keep = 7)
                client.list()
            }.onSuccess { files ->
                markBackedUp()
                cloudFiles = files
                cloudStatus = null
            }.onFailure { cloudStatus = it.message }
        }
    }

    private fun withToken(interactive: Boolean, block: (String) -> Unit) {
        val auth = cloudAuth ?: return
        auth(interactive, block) { cloudStatus = it }
    }
    val members = MemberRepository(db)
    val goals = GoalRepository(db)
    val ledger = LedgerRepository(db, goals = { goals.states(it) }).also { goals.ledger = it }
    val tasks = TaskRepository(db, ledger)
    val allowance = AllowanceRepository(db)

    /** 開 App（含從背景回來）時補記常態零用錢 */
    fun backfillAllowance() {
        if (allowance.backfillAll() > 0) ledgerChanged()
    }

    /** 每次記帳後遞增，讓畫面重新讀餘額 */
    var ledgerVersion by mutableStateOf(0)
        private set

    fun ledgerChanged() {
        ledgerVersion++
    }

    var celebration: Celebration? by mutableStateOf(null)
        private set

    fun celebrate(c: Celebration) {
        celebration = c
    }

    fun dismissCelebration() {
        celebration = null
    }
    private val prefs = application.getSharedPreferences("cacao_device", Context.MODE_PRIVATE)
    private var backgroundSince = 0L

    /** 主題、音效是這台手機的設定，存偏好設定、不進備份 */
    var themeId: String by mutableStateOf(prefs.getString("theme", "cocoa_milk") ?: "cocoa_milk")
        private set

    var soundOn: Boolean by mutableStateOf(prefs.getBoolean("sound", true))
        private set

    init {
        cloudEnabled = prefs.getBoolean("cloud", false)
        lastBackupMillis = prefs.getLong("last_backup", 0L)
    }

    fun setTheme(id: String) {
        themeId = id
        prefs.edit().putString("theme", id).apply()
    }

    fun setSound(on: Boolean) {
        soundOn = on
        prefs.edit().putBoolean("sound", on).apply()
    }

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
            is PinPurpose.VerifyTask -> members.parentForPin(pin)?.let { parent ->
                val task = tasks.task(purpose.taskId)
                if (tasks.complete(purpose.taskId, parent.id) == TaskResult.Ok && task != null) {
                    celebrate(Celebration.TaskDone(task.title, money(task.reward_amount)))
                }
                ledgerChanged()
                Screen.ChildTasks(purpose.childId)
            }
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
        backfillAllowance()
        autoBackupIfDue()
    }

    private fun save(purpose: PinPurpose, state: AttemptState) {
        prefs.edit()
            .putInt("fail_${purpose.limiterKey}", state.failures)
            .putLong("lock_${purpose.limiterKey}", state.lockedUntilMillis)
            .apply()
    }
}
