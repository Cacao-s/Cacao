package com.ruru.cacao

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.Screen
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

class MainActivity : ComponentActivity() {

    private val vm: AppViewModel by viewModels()

    /** 螢幕關掉就把父母模式鎖回去 */
    private val screenOff = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) = vm.lockParentMode()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        registerReceiver(screenOff, IntentFilter(Intent.ACTION_SCREEN_OFF))
        setContent { CacaoTheme { AppRoot(vm) } }
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
            else -> vm.go(Screen.SelectUser)
        }
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
        }
    }
}
