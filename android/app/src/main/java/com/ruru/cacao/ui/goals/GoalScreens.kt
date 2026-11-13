package com.ruru.cacao.ui.goals

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectVerticalDragGestures
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Checkbox
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateMapOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.zIndex
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.R
import com.ruru.cacao.data.GoalResult
import com.ruru.cacao.data.GoalView
import com.ruru.cacao.domain.GoalStatus
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.Celebration
import com.ruru.cacao.ui.components.ErrorText
import com.ruru.cacao.ui.components.PlainField
import com.ruru.cacao.ui.components.PrimaryButton
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.ledger.money
import com.ruru.cacao.ui.theme.Cacao

private fun goalsBack(vm: AppViewModel, childId: String, parentId: String?) =
    vm.go(if (parentId != null) Screen.ChildOverview(parentId, childId) else Screen.ChildHome(childId))

@Composable
fun statusLabel(view: GoalView): String = stringResource(
    when {
        view.state.status == GoalStatus.COMPLETED -> R.string.goal_completed
        view.state.status == GoalStatus.ACHIEVED -> R.string.goal_achieved
        view.achievable -> R.string.goal_achievable
        else -> R.string.goal_in_progress
    },
)

/** C02 目標列表（父母與孩子共用） */
@Composable
fun GoalsScreen(vm: AppViewModel, childId: String, parentId: String?) {
    val views = remember(vm.ledgerVersion, childId) { vm.goals.views(childId) }
    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            TextButton(onClick = { goalsBack(vm, childId, parentId) }, modifier = Modifier.testTag("back")) {
                Text("‹", color = Cacao.colors.primary, fontSize = 22.sp)
            }
            Title(stringResource(R.string.goals_title), Modifier.weight(1f))
            TextButton(onClick = { vm.go(Screen.NewGoal(childId, parentId)) }, modifier = Modifier.testTag("goal_new")) {
                Text(stringResource(R.string.goal_new), color = Cacao.colors.primary, fontWeight = FontWeight.Bold)
            }
        }
        Body(stringResource(R.string.goals_hint))
        if (views.isEmpty()) Body(stringResource(R.string.goals_empty))
        GoalList(vm, views, childId, parentId)
    }
}

/** 拖右邊的 ≡ 上下調整順序，放開才存（只上下） */
@Composable
private fun GoalList(vm: AppViewModel, views: List<GoalView>, childId: String, parentId: String?) {
    var order by remember(views) { mutableStateOf(views) }
    var draggingId by remember { mutableStateOf<String?>(null) }
    var offset by remember { mutableFloatStateOf(0f) }
    val heights = remember { mutableStateMapOf<String, Float>() }
    val gap = with(LocalDensity.current) { 16.dp.toPx() }

    fun finish() {
        if (draggingId == null) return
        draggingId = null
        offset = 0f
        if (order.map { it.goal.id } != views.map { it.goal.id }) {
            vm.goals.reorder(childId, order.map { it.goal.id })
            vm.ledgerChanged()
        }
    }

    fun drag(id: String, dy: Float) {
        offset += dy
        val i = order.indexOfFirst { it.goal.id == id }
        val j = if (offset > 0) i + 1 else i - 1
        if (j !in order.indices) return
        val step = (heights[order[j].goal.id] ?: return) + gap
        if (kotlin.math.abs(offset) < step / 2) return
        order = order.toMutableList().apply { add(j, removeAt(i)) }
        offset -= if (j > i) step else -step
    }

    order.forEach { view ->
        val id = view.goal.id
        key(id) {
            val dragging = draggingId == id
            GoalCard(
                vm, view, childId, parentId,
                modifier = Modifier
                    .onSizeChanged { heights[id] = it.height.toFloat() }
                    .zIndex(if (dragging) 1f else 0f)
                    .graphicsLayer { translationY = if (dragging) offset else 0f }
                    .then(if (dragging) Modifier.shadow(8.dp, RoundedCornerShape(Cacao.shape.radius)) else Modifier),
                handle = Modifier.pointerInput(id, views) {
                    detectVerticalDragGestures(
                        onDragStart = { draggingId = id; offset = 0f },
                        onDragEnd = { finish() },
                        onDragCancel = { finish() },
                        onVerticalDrag = { change, dy -> change.consume(); drag(id, dy) },
                    )
                },
            )
        }
    }
}

@Composable
private fun GoalCard(vm: AppViewModel, view: GoalView, childId: String, parentId: String?, modifier: Modifier, handle: Modifier) {
    val goal = view.goal
    val locked = view.state.status == GoalStatus.ACHIEVED
    val shape = RoundedCornerShape(Cacao.shape.radius)
    Column(
        modifier = modifier
            .fillMaxWidth()
            .background(Cacao.colors.surface, shape)
            .border(if (locked || view.achievable) 2.dp else maxOf(Cacao.shape.border, 1.dp), if (locked || view.achievable) Cacao.colors.locked else Cacao.colors.outline, shape)
            .clickable { vm.go(Screen.GoalDetail(childId, goal.id, parentId)) }
            .padding(14.dp)
            .testTag("goal_${goal.name}"),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(goal.name, fontWeight = FontWeight.Bold, fontSize = 17.sp, color = Cacao.colors.onSurface, modifier = Modifier.weight(1f))
            Text(
                statusLabel(view),
                fontSize = 12.sp,
                fontWeight = FontWeight.Bold,
                color = if (locked || view.achievable) Cacao.colors.onLocked else Cacao.colors.muted,
                modifier = Modifier
                    .background(if (locked || view.achievable) Cacao.colors.locked else Cacao.colors.background, RoundedCornerShape(50))
                    .padding(horizontal = 10.dp, vertical = 3.dp)
                    .testTag("goal_status_${goal.name}"),
            )
        }
        ProgressBar(view.saved.toFloat() / goal.target_amount)
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(
                "${money(view.saved)} / ${money(goal.target_amount)}",
                fontFamily = Cacao.fonts.number,
                fontSize = 13.sp,
                color = Cacao.colors.muted,
                modifier = Modifier.weight(1f).testTag("goal_saved_${goal.name}"),
            )
            if (view.state.status == GoalStatus.IN_PROGRESS) {
                Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.clickable { vm.goals.setFill(goal.id, goal.fill_enabled == 0L); vm.ledgerChanged() }.testTag("goal_fill_${goal.name}")) {
                    Checkbox(checked = goal.fill_enabled == 1L, onCheckedChange = { vm.goals.setFill(goal.id, it); vm.ledgerChanged() })
                    Text(stringResource(R.string.goal_fill), fontSize = 12.sp, color = Cacao.colors.muted)
                }
            }
            Text(
                "≡",
                fontSize = 24.sp,
                color = Cacao.colors.muted,
                modifier = handle.padding(horizontal = 12.dp, vertical = 4.dp).testTag("goal_drag_${goal.name}"),
            )
        }
    }
}

@Composable
private fun ProgressBar(fraction: Float) {
    Box(
        Modifier
            .fillMaxWidth()
            .height(10.dp)
            .background(Cacao.colors.background, RoundedCornerShape(50)),
    ) {
        Box(
            Modifier
                .fillMaxWidth(fraction.coerceIn(0f, 1f))
                .fillMaxHeight()
                .background(Cacao.colors.locked, RoundedCornerShape(50)),
        )
    }
}

/** 新增目標 */
@Composable
fun NewGoalScreen(vm: AppViewModel, childId: String, parentId: String?) {
    var name by remember { mutableStateOf("") }
    var target by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<String?>(null) }
    val invalid = stringResource(R.string.amount_invalid)
    ScreenColumn {
        Title(stringResource(R.string.goal_new))
        PlainField(stringResource(R.string.goal_name_label), name, "goal_name") { name = it }
        PlainField(stringResource(R.string.goal_target_label), target, "goal_target", KeyboardType.Number) { target = it.filter(Char::isDigit).take(7) }
        ErrorText(error)
        Spacer(Modifier.weight(1f))
        PrimaryButton(stringResource(R.string.action_save), "goal_save") {
            if (vm.goals.create(childId, name, target.toLongOrNull() ?: 0, parentId ?: childId) == GoalResult.Ok) {
                vm.ledgerChanged()
                vm.go(Screen.Goals(childId, parentId))
            } else {
                error = invalid
            }
        }
        TextButton(onClick = { vm.go(Screen.Goals(childId, parentId)) }, modifier = Modifier.fillMaxWidth()) {
            Text(stringResource(R.string.action_cancel), color = Cacao.colors.primary)
        }
    }
}

/** C03 目標詳情：孩子確認／退回、改金額、刪除；父母多「出金」 */
@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun GoalDetailScreen(vm: AppViewModel, childId: String, goalId: String, parentId: String?) {
    val view = remember(vm.ledgerVersion, goalId) { vm.goals.view(childId, goalId) } ?: return
    val goal = view.goal
    val actor = parentId ?: childId
    var editingTarget by remember { mutableStateOf(false) }
    var newTarget by remember { mutableStateOf(goal.target_amount.toString()) }
    var payingOut by remember { mutableStateOf(false) }
    var price by remember { mutableStateOf(view.state.lockedAmount.toString()) }
    var askDelete by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }
    val overText = stringResource(R.string.goal_payout_over, money(view.state.lockedAmount))
    val invalid = stringResource(R.string.amount_invalid)
    fun changed() { vm.ledgerChanged() }

    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        TextButton(onClick = { vm.go(Screen.Goals(childId, parentId)) }, modifier = Modifier.testTag("back")) {
            Text("‹ " + stringResource(R.string.back), color = Cacao.colors.primary)
        }
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                .padding(20.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            Title(goal.name)
            Text(statusLabel(view), color = Cacao.colors.muted, modifier = Modifier.testTag("detail_status"))
            Text(money(goal.target_amount), fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 36.sp, color = Cacao.colors.onSurface)
            ProgressBar(view.saved.toFloat() / goal.target_amount)
            val designated = view.progress?.designated ?: 0
            if (designated > 0) Body(stringResource(R.string.goal_designated, money(designated)))
            if (view.state.status == GoalStatus.IN_PROGRESS && !view.achievable) {
                Body(stringResource(R.string.goal_need_more, money(goal.target_amount - view.saved)))
            }
        }
        ErrorText(error)

        if (view.achievable && parentId == null) {
            PrimaryButton(stringResource(R.string.goal_confirm), "goal_confirm") {
                if (vm.goals.confirm(childId, goalId, childId) == GoalResult.Ok) vm.celebrate(Celebration.GoalReached(goal.name))
                changed()
            }
        }
        if (view.state.status == GoalStatus.ACHIEVED) {
            if (parentId != null) {
                if (payingOut) {
                    PlainField(stringResource(R.string.goal_payout_amount), price, "goal_price", KeyboardType.Number) { price = it.filter(Char::isDigit).take(7) }
                    PrimaryButton(stringResource(R.string.goal_payout), "goal_payout_save") {
                        when (vm.goals.payout(childId, goalId, price.toLongOrNull() ?: 0, parentId, null)) {
                            GoalResult.Ok -> { payingOut = false; error = null; changed() }
                            is GoalResult.OverLocked -> error = overText
                            else -> error = invalid
                        }
                    }
                } else {
                    PrimaryButton(stringResource(R.string.goal_payout), "goal_payout") { payingOut = true }
                }
            }
            TextButton(onClick = { vm.goals.revert(childId, goalId, actor); changed() }, modifier = Modifier.fillMaxWidth().testTag("goal_revert")) {
                Text(stringResource(R.string.goal_revert), color = Cacao.colors.primary)
            }
        }
        if (view.state.status == GoalStatus.IN_PROGRESS) {
            if (editingTarget) {
                PlainField(stringResource(R.string.goal_target_label), newTarget, "goal_new_target", KeyboardType.Number) { newTarget = it.filter(Char::isDigit).take(7) }
                PrimaryButton(stringResource(R.string.action_save), "goal_target_save") {
                    if (vm.goals.changeTarget(childId, goalId, newTarget.toLongOrNull() ?: 0, actor) == GoalResult.Ok) {
                        editingTarget = false; error = null; changed()
                    } else {
                        error = invalid
                    }
                }
            } else {
                TextButton(onClick = { editingTarget = true }, modifier = Modifier.fillMaxWidth().testTag("goal_change_target")) {
                    Text(stringResource(R.string.goal_change_target), color = Cacao.colors.primary)
                }
            }
        }
        TextButton(onClick = { askDelete = true }, modifier = Modifier.fillMaxWidth().testTag("goal_delete")) {
            Text(stringResource(R.string.goal_delete), color = Cacao.colors.danger)
        }
    }

    if (askDelete) {
        AlertDialog(
            onDismissRequest = { askDelete = false },
            title = { Text(stringResource(R.string.goal_delete)) },
            text = { Text(stringResource(R.string.goal_delete_body)) },
            confirmButton = {
                TextButton(
                    onClick = { vm.goals.delete(goalId); changed(); vm.go(Screen.Goals(childId, parentId)) },
                    modifier = Modifier.semantics { testTagsAsResourceId = true }.testTag("goal_delete_confirm"),
                ) { Text(stringResource(R.string.goal_delete), color = Cacao.colors.danger) }
            },
            dismissButton = { TextButton(onClick = { askDelete = false }) { Text(stringResource(R.string.action_cancel)) } },
        )
    }
}
