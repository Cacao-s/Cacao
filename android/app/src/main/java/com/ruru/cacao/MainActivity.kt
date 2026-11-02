package com.ruru.cacao

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.activity.compose.BackHandler
import androidx.activity.result.IntentSenderRequest
import androidx.activity.result.contract.ActivityResultContracts
import com.google.android.gms.auth.api.identity.AuthorizationRequest
import com.google.android.gms.auth.api.identity.Identity
import com.google.android.gms.common.api.Scope
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalContext
import com.ruru.cacao.ui.components.CelebrationOverlay
import com.ruru.cacao.ui.components.CelebrationSound
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.PinPurpose
import com.ruru.cacao.ui.allowance.AllowanceScreen
import com.ruru.cacao.ui.backup.BackupScreen
import com.ruru.cacao.ui.settings.FamilyScreen
import com.ruru.cacao.ui.settings.SettingsScreen
import com.ruru.cacao.ui.theme.generated.GeneratedThemes
import com.ruru.cacao.ui.wallet.GrowthScreen
import com.ruru.cacao.ui.wallet.WalletScreen
import com.ruru.cacao.ui.tasks.ChildTasksScreen
import com.ruru.cacao.ui.tasks.NewTaskScreen
import com.ruru.cacao.ui.tasks.TasksScreen
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.goals.GoalDetailScreen
import com.ruru.cacao.ui.goals.GoalsScreen
import com.ruru.cacao.ui.goals.NewGoalScreen
import com.ruru.cacao.ui.home.ChildHomeScreen
import com.ruru.cacao.ui.home.ParentHomeScreen
import com.ruru.cacao.ui.identity.ChildPasswordScreen
import com.ruru.cacao.ui.identity.ForgotPinScreen
import com.ruru.cacao.ui.identity.OnboardingScreen
import com.ruru.cacao.ui.identity.PinScreen
import com.ruru.cacao.ui.identity.SelectUserScreen
import com.ruru.cacao.ui.ledger.AddIncomeScreen
import com.ruru.cacao.ui.ledger.ChildOverviewScreen
import com.ruru.cacao.ui.ledger.LedgerScreen
import com.ruru.cacao.ui.ledger.PayoutScreen
import com.ruru.cacao.ui.theme.CacaoTheme

/** AppCompatActivity：Android 12 以下才能用 App 內切換語言 */
class MainActivity : AppCompatActivity() {

    private val vm: AppViewModel by viewModels()

    /** 螢幕關掉就把父母模式鎖回去 */
    private val screenOff = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) = vm.lockParentMode()
    }

    private var pendingToken: ((String) -> Unit)? = null
    private var pendingError: ((String) -> Unit)? = null

    private val authLauncher = registerForActivityResult(ActivityResultContracts.StartIntentSenderForResult()) { result ->
        val token = runCatching { Identity.getAuthorizationClient(this).getAuthorizationResultFromIntent(result.data).accessToken }.getOrNull()
        if (token != null) pendingToken?.invoke(token) else pendingError?.invoke("authorization cancelled")
        pendingToken = null
        pendingError = null
    }

    /** 只要 drive.appdata：只能存取 Cacao 自己的隱藏資料夾（D-05） */
    private fun authorizeDrive(interactive: Boolean, onToken: (String) -> Unit, onError: (String) -> Unit) {
        val request = AuthorizationRequest.builder()
            .setRequestedScopes(listOf(Scope("https://www.googleapis.com/auth/drive.appdata")))
            .build()
        Identity.getAuthorizationClient(this).authorize(request)
            .addOnSuccessListener { result ->
                val pending = result.pendingIntent
                when {
                    result.hasResolution() && pending != null && interactive -> {
                        pendingToken = onToken
                        pendingError = onError
                        authLauncher.launch(IntentSenderRequest.Builder(pending.intentSender).build())
                    }
                    result.accessToken != null -> onToken(result.accessToken!!)
                    else -> onError("needs sign-in")
                }
            }
            .addOnFailureListener { onError(it.message ?: "Google services unavailable") }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        vm.cloudAuth = ::authorizeDrive
        registerReceiver(screenOff, IntentFilter(Intent.ACTION_SCREEN_OFF))
        setContent {
            val tokens = GeneratedThemes.all.firstOrNull { it.id == vm.themeId } ?: GeneratedThemes.cocoaMilk
            CacaoTheme(tokens) { AppRoot(vm) }
        }
    }

    override fun onStart() {
        super.onStart()
        vm.onForeground(System.currentTimeMillis())
    }

    override fun onStop() {
        super.onStop()
        vm.onBackground(System.currentTimeMillis())
    }

    override fun onDestroy() {
        vm.cloudAuth = null
        unregisterReceiver(screenOff)
        super.onDestroy()
    }
}

@OptIn(ExperimentalComposeUiApi::class)
@Composable
private fun AppRoot(vm: AppViewModel) {
    val screen = vm.screen
    BackHandler(enabled = screen !is Screen.SelectUser && screen !is Screen.Onboarding) {
        when (screen) {
            is Screen.ChildPassword -> vm.go(Screen.ChildHome(screen.childId))
            is Screen.ChildOverview -> vm.go(Screen.ParentHome(screen.parentId))
            is Screen.AddIncome -> vm.go(Screen.ChildOverview(screen.parentId, screen.childId))
            is Screen.Payout -> vm.go(Screen.ChildOverview(screen.parentId, screen.childId))
            is Screen.Ledger -> vm.go(Screen.ChildOverview(screen.parentId, screen.childId))
            is Screen.Goals -> vm.go(screen.parentId?.let { Screen.ChildOverview(it, screen.childId) } ?: Screen.ChildHome(screen.childId))
            is Screen.GoalDetail -> vm.go(Screen.Goals(screen.childId, screen.parentId))
            is Screen.NewGoal -> vm.go(Screen.Goals(screen.childId, screen.parentId))
            is Screen.Tasks -> vm.go(Screen.ParentHome(screen.parentId))
            is Screen.NewTask -> vm.go(Screen.Tasks(screen.parentId))
            is Screen.ChildTasks -> vm.go(Screen.ChildHome(screen.childId))
            is Screen.Allowance -> vm.go(Screen.ChildOverview(screen.parentId, screen.childId))
            is Screen.Wallet -> vm.go(screen.parentId?.let { Screen.ChildOverview(it, screen.childId) } ?: Screen.ChildHome(screen.childId))
            is Screen.Growth -> vm.go(screen.parentId?.let { Screen.ChildOverview(it, screen.childId) } ?: Screen.ChildHome(screen.childId))
            is Screen.Settings -> vm.go(Screen.ParentHome(screen.parentId))
            is Screen.Family -> vm.go(Screen.Settings(screen.parentId))
            is Screen.Backup -> vm.go(Screen.Settings(screen.parentId))
            is Screen.Pin -> vm.go((screen.purpose as? PinPurpose.VerifyTask)?.let { Screen.ChildTasks(it.childId) } ?: Screen.SelectUser)
            else -> vm.go(Screen.SelectUser)
        }
    }
    val context = LocalContext.current
    val sound = remember { CelebrationSound(context) }
    DisposableEffect(Unit) { onDispose { sound.release() } }
    LaunchedEffect(vm.celebration) {
        if (vm.celebration != null && vm.soundOn) sound.play()
    }
    Box(Modifier.fillMaxSize().safeDrawingPadding().semantics { testTagsAsResourceId = true }) {
        when (screen) {
            Screen.Onboarding -> OnboardingScreen(vm)
            Screen.SelectUser -> SelectUserScreen(vm)
            is Screen.Pin -> PinScreen(vm, screen.purpose)
            is Screen.ParentHome -> ParentHomeScreen(vm, screen.parentId)
            is Screen.ChildHome -> ChildHomeScreen(vm, screen.childId)
            is Screen.ForgotPin -> ForgotPinScreen(vm, screen.parentId)
            is Screen.ChildPassword -> ChildPasswordScreen(vm, screen.childId)
            is Screen.ChildOverview -> ChildOverviewScreen(vm, screen.parentId, screen.childId)
            is Screen.AddIncome -> AddIncomeScreen(vm, screen.parentId, screen.childId)
            is Screen.Payout -> PayoutScreen(vm, screen.parentId, screen.childId)
            is Screen.Ledger -> LedgerScreen(vm, screen.parentId, screen.childId)
            is Screen.Goals -> GoalsScreen(vm, screen.childId, screen.parentId)
            is Screen.GoalDetail -> GoalDetailScreen(vm, screen.childId, screen.goalId, screen.parentId)
            is Screen.NewGoal -> NewGoalScreen(vm, screen.childId, screen.parentId)
            is Screen.Tasks -> TasksScreen(vm, screen.parentId)
            is Screen.NewTask -> NewTaskScreen(vm, screen.parentId)
            is Screen.ChildTasks -> ChildTasksScreen(vm, screen.childId)
            is Screen.Allowance -> AllowanceScreen(vm, screen.parentId, screen.childId)
            is Screen.Wallet -> WalletScreen(vm, screen.childId, screen.parentId)
            is Screen.Growth -> GrowthScreen(vm, screen.childId, screen.parentId)
            is Screen.Settings -> SettingsScreen(vm, screen.parentId)
            is Screen.Family -> FamilyScreen(vm, screen.parentId)
            is Screen.Backup -> BackupScreen(vm, screen.parentId)
        }
        vm.celebration?.let { CelebrationOverlay(it) { vm.dismissCelebration() } }
    }
}
