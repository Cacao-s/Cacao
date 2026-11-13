package com.ruru.cacao.ui.home

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.ui.graphics.Color
import androidx.compose.runtime.remember
import com.ruru.cacao.ui.ledger.money
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.R
import com.ruru.cacao.data.Role
import com.ruru.cacao.data.roleEnum
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.PinPurpose
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.components.Avatar
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.CuteLock
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.TileGrid
import com.ruru.cacao.ui.components.TileItem
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.theme.Cacao
import java.time.LocalDate
import java.time.Period

/** P01 孩子列表（M1 先做骨架，餘額在 M2 接上） */
@Composable
fun ParentHomeScreen(vm: AppViewModel, parentId: String) {
    val parent = vm.member(parentId)
    val children = vm.memberList.filter { it.roleEnum == Role.CHILD }
    ScreenColumn {
        Body(stringResource(R.string.parent_mode_header, parent?.name.orEmpty()))
        Title(stringResource(R.string.children_title), Modifier.testTag("parent_home_title"))
        children.forEach { child ->
            val b = remember(vm.ledgerVersion, child.id) { vm.ledger.balances(child.id) }
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                    .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
                    .clickable { vm.go(Screen.ChildOverview(parentId, child.id)) }
                    .padding(16.dp)
                    .testTag("child_row_${child.name}"),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(14.dp),
            ) {
                Avatar(child.name, child.avatar_color, 52)
                Column(Modifier.weight(1f)) {
                    Text(child.name, fontSize = 18.sp, fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
                    child.birthday?.let { Body(stringResource(R.string.age_years, ageOf(it))) }
                    Text(
                        "${stringResource(R.string.savings)} ${money(b.savings)}・${stringResource(R.string.wallet)} ${money(b.wallet)}",
                        fontFamily = Cacao.fonts.number,
                        fontSize = 14.sp,
                        color = Cacao.colors.muted,
                    )
                }
                Text("›", color = Cacao.colors.muted, fontSize = 20.sp)
            }
        }
        TextButton(onClick = { vm.go(Screen.Tasks(parentId)) }, modifier = Modifier.testTag("parent_tasks")) {
            Text(stringResource(R.string.parent_tasks) + " ›", color = Cacao.colors.primary, fontWeight = FontWeight.Bold)
        }
        TextButton(onClick = { vm.go(Screen.Settings(parentId)) }, modifier = Modifier.testTag("parent_settings")) {
            Text(stringResource(R.string.settings_title) + " ›", color = Cacao.colors.primary, fontWeight = FontWeight.Bold)
        }
        Spacer(Modifier.weight(1f))
        TextButton(onClick = { vm.go(Screen.SelectUser) }, modifier = Modifier.testTag("parent_logout")) {
            Text(stringResource(R.string.logout_parent), color = Cacao.colors.primary)
        }
    }
}

/** C01 孩子首頁：上半部存款、錢包、最近收到的錢；下半部彩色正方形按鈕一排 3 個（UQ-09） */
@Composable
fun ChildHomeScreen(vm: AppViewModel, childId: String) {
    val child = vm.member(childId) ?: return
    val b = remember(vm.ledgerVersion, childId) { vm.ledger.balances(childId) }
    val topGoal = remember(vm.ledgerVersion, childId) { vm.goals.views(childId).firstOrNull { it.state.status != com.ruru.cacao.domain.GoalStatus.COMPLETED } }
    val names = vm.memberList.associate { it.id to it.name }
    val gift = remember(vm.ledgerVersion, childId) {
        vm.ledger.entries(childId).firstOrNull { it.voided == 0L && it.kind.startsWith("income_") && it.source_id != null }
    }
    ScreenColumn {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Title(stringResource(R.string.child_greeting, child.name), Modifier.weight(1f).testTag("child_home_title"))
            TextButton(onClick = { vm.go(Screen.Pin(PinPurpose.AnyParent)) }, modifier = Modifier.testTag("child_lock")) {
                CuteLock(28.dp, stringResource(R.string.switch_to_parent))
            }
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            BalanceCard(stringResource(R.string.savings), b.savings, Cacao.colors.primary, Cacao.colors.onPrimary, "child_savings", Modifier.weight(1f))
            BalanceCard(stringResource(R.string.wallet), b.wallet, Cacao.colors.surface, Cacao.colors.onSurface, "child_wallet", Modifier.weight(1f))
        }
        gift?.let {
            Column(
                Modifier
                    .fillMaxWidth()
                    .background(Cacao.colors.locked.copy(alpha = 0.25f), RoundedCornerShape(Cacao.shape.radius))
                    .padding(14.dp),
            ) {
                Text(stringResource(R.string.gift_from, names[it.source_id].orEmpty(), money(it.amount)), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
                it.message?.let { m -> Text("「$m」", color = Cacao.colors.muted, fontSize = 14.sp) }
            }
        }
        val goalSub = topGoal?.let { "${it.goal.name} ${(it.saved * 100 / it.goal.target_amount)}%" } ?: stringResource(R.string.goals_empty)
        val pendingTasks = remember(vm.ledgerVersion, childId) { vm.tasks.pending(childId).size }
        TileGrid(
            listOf(
                TileItem(stringResource(R.string.menu_goals), goalSub, Cacao.colors.locked, "tile_goals") { vm.go(Screen.Goals(childId, null)) },
                TileItem(stringResource(R.string.menu_tasks), stringResource(R.string.tasks_pending_count, pendingTasks), Cacao.colors.secondary, "tile_tasks") { vm.go(Screen.ChildTasks(childId)) },
                TileItem(stringResource(R.string.menu_wallet), stringResource(R.string.wallet_add_expense), Cacao.colors.primary, "tile_wallet") { vm.go(Screen.Wallet(childId, null)) },
                TileItem(stringResource(R.string.menu_growth), stringResource(R.string.growth_title), Cacao.colors.outline, "tile_growth") { vm.go(Screen.Growth(childId, null)) },
            ),
        )
        Spacer(Modifier.weight(1f))
        if (child.password_hash == null) {
            TextButton(onClick = { vm.go(Screen.ChildPassword(childId)) }, modifier = Modifier.testTag("child_set_password")) {
                Text(stringResource(R.string.child_set_password), color = Cacao.colors.primary)
            }
        } else {
            TextButton(onClick = { vm.members.setChildPassword(childId, null); vm.go(Screen.ChildHome(childId)) }, modifier = Modifier.testTag("child_clear_password")) {
                Text(stringResource(R.string.child_clear_password), color = Cacao.colors.primary)
            }
        }
        TextButton(onClick = { vm.go(Screen.SelectUser) }, modifier = Modifier.testTag("child_back")) {
            Text(stringResource(R.string.back), color = Cacao.colors.primary)
        }
    }
}

@Composable
private fun BalanceCard(label: String, amount: Long, background: Color, content: Color, tag: String, modifier: Modifier) {
    Column(
        modifier
            .background(background, RoundedCornerShape(Cacao.shape.radius))
            .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
            .padding(16.dp),
    ) {
        Text(label, color = content, fontSize = 14.sp)
        Text(money(amount), color = content, fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 26.sp, modifier = Modifier.testTag(tag))
    }
}

private fun ageOf(birthday: String): Int =
    runCatching { Period.between(LocalDate.parse(birthday), LocalDate.now()).years }.getOrDefault(0)
