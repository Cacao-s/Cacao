package com.ruru.cacao.ui.tasks

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.R
import com.ruru.cacao.data.Role
import com.ruru.cacao.data.TaskResult
import com.ruru.cacao.data.roleEnum
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.PinPurpose
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.ErrorText
import com.ruru.cacao.ui.components.PlainField
import com.ruru.cacao.ui.components.PrimaryButton
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.ledger.money
import com.ruru.cacao.ui.theme.Cacao

@Composable
private fun Card(tag: String, content: @Composable () -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
            .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
            .padding(14.dp)
            .testTag(tag),
        verticalAlignment = Alignment.CenterVertically,
    ) { content() }
}

@Composable
private fun Chip(text: String, selected: Boolean, tag: String, onClick: () -> Unit) {
    Text(
        text,
        modifier = Modifier
            .background(if (selected) Cacao.colors.primary else Cacao.colors.surface, RoundedCornerShape(50))
            .border(1.dp, Cacao.colors.outline, RoundedCornerShape(50))
            .clickable(onClick = onClick)
            .padding(horizontal = 16.dp, vertical = 10.dp)
            .testTag(tag),
        color = if (selected) Cacao.colors.onPrimary else Cacao.colors.onSurface,
    )
}

/** P08 任務管理：所有孩子待完成的任務 */
@Composable
fun TasksScreen(vm: AppViewModel, parentId: String) {
    val pending = remember(vm.ledgerVersion) { vm.tasks.pendingAll() }
    val names = vm.memberList.associate { it.id to it.name }
    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            TextButton(onClick = { vm.go(Screen.ParentHome(parentId)) }, modifier = Modifier.testTag("back")) {
                Text("‹", color = Cacao.colors.primary, fontSize = 22.sp)
            }
            Title(stringResource(R.string.parent_tasks), Modifier.weight(1f))
            TextButton(onClick = { vm.go(Screen.NewTask(parentId)) }, modifier = Modifier.testTag("task_new")) {
                Text(stringResource(R.string.task_new), color = Cacao.colors.primary, fontWeight = FontWeight.Bold)
            }
        }
        if (pending.isEmpty()) Body(stringResource(R.string.ledger_empty))
        pending.forEach { task ->
            Card("ptask_${task.title}_${names[task.child_id]}") {
                Column(Modifier.weight(1f)) {
                    Text(task.title, fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
                    Text(
                        stringResource(R.string.task_for, names[task.child_id].orEmpty()) + "・" +
                            stringResource(R.string.task_by, names[task.issuer_id].orEmpty(), money(task.reward_amount)),
                        fontSize = 13.sp,
                        color = Cacao.colors.muted,
                    )
                }
                TextButton(onClick = { vm.tasks.delete(task.id); vm.ledgerChanged() }) {
                    Text(stringResource(R.string.task_delete), color = Cacao.colors.danger)
                }
            }
        }
    }
}

/** 新增任務：可多選孩子、選出題人 */
@Composable
fun NewTaskScreen(vm: AppViewModel, parentId: String) {
    var title by remember { mutableStateOf("") }
    var reward by remember { mutableStateOf("") }
    var issuerId by remember { mutableStateOf(parentId) }
    var picked by remember { mutableStateOf(setOf<String>()) }
    var error by remember { mutableStateOf<String?>(null) }
    val issuers = vm.memberList.filter { it.roleEnum == Role.PARENT || it.roleEnum == Role.ELDER }
    val children = vm.memberList.filter { it.roleEnum == Role.CHILD }
    val invalid = stringResource(R.string.task_need_fields)

    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Title(stringResource(R.string.task_new))
        PlainField(stringResource(R.string.task_title_label), title, "task_title") { title = it }
        PlainField(stringResource(R.string.task_reward_label), reward, "task_reward", KeyboardType.Number) { reward = it.filter(Char::isDigit).take(6) }
        Body(stringResource(R.string.task_children_label))
        FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            children.forEach { c ->
                Chip(c.name, c.id in picked, "task_child_${c.name}") { picked = if (c.id in picked) picked - c.id else picked + c.id }
            }
        }
        Body(stringResource(R.string.task_issuer_label))
        FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
            issuers.forEach { m -> Chip(m.name, m.id == issuerId, "task_issuer_${m.name}") { issuerId = m.id } }
        }
        ErrorText(error)
        Spacer(Modifier.height(8.dp))
        PrimaryButton(stringResource(R.string.action_save), "task_save") {
            val result = vm.tasks.create(title, reward.toLongOrNull() ?: 0, issuerId, children.filter { it.id in picked }.map { it.id }, parentId)
            if (result == TaskResult.Ok) {
                vm.ledgerChanged()
                vm.go(Screen.Tasks(parentId))
            } else {
                error = invalid
            }
        }
        TextButton(onClick = { vm.go(Screen.Tasks(parentId)) }, modifier = Modifier.fillMaxWidth()) {
            Text(stringResource(R.string.action_cancel), color = Cacao.colors.primary)
        }
    }
}

/** C04 孩子的任務：按「完成」→ 父母當場輸入 PIN 驗收 */
@Composable
fun ChildTasksScreen(vm: AppViewModel, childId: String) {
    val pending = remember(vm.ledgerVersion, childId) { vm.tasks.pending(childId) }
    val done = remember(vm.ledgerVersion, childId) { vm.tasks.completed(childId) }
    val names = vm.memberList.associate { it.id to it.name }
    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            TextButton(onClick = { vm.go(Screen.ChildHome(childId)) }, modifier = Modifier.testTag("back")) {
                Text("‹", color = Cacao.colors.primary, fontSize = 22.sp)
            }
            Title(stringResource(R.string.tasks_title))
        }
        Body(stringResource(R.string.task_hint))
        if (pending.isEmpty()) Body(stringResource(R.string.task_pending_empty))
        pending.forEach { task ->
            Card("task_${task.title}") {
                Column(Modifier.weight(1f)) {
                    Text(task.title, fontWeight = FontWeight.Bold, fontSize = 17.sp, color = Cacao.colors.onSurface)
                    Text(stringResource(R.string.task_by, names[task.issuer_id].orEmpty(), money(task.reward_amount)), fontSize = 13.sp, color = Cacao.colors.muted)
                }
                Button(
                    onClick = { vm.go(Screen.Pin(PinPurpose.VerifyTask(task.id, childId))) },
                    shape = RoundedCornerShape(50),
                    colors = ButtonDefaults.buttonColors(containerColor = Cacao.colors.primary, contentColor = Cacao.colors.onPrimary),
                    modifier = Modifier.testTag("task_done_${task.title}"),
                ) { Text(stringResource(R.string.action_done), fontWeight = FontWeight.Bold) }
            }
        }
        if (done.isNotEmpty()) Body(stringResource(R.string.task_done_section))
        done.forEach { task ->
            Card("task_completed_${task.title}") {
                Text(task.title, color = Cacao.colors.muted, modifier = Modifier.weight(1f))
                Text("+" + money(task.reward_amount), fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, color = Cacao.colors.secondary)
            }
        }
    }
}
