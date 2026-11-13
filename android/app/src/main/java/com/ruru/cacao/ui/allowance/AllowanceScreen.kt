package com.ruru.cacao.ui.allowance

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
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
import com.ruru.cacao.data.ScheduleInput
import com.ruru.cacao.data.periodEnum
import com.ruru.cacao.data.roleEnum
import com.ruru.cacao.db.Allowance_schedule
import com.ruru.cacao.domain.DateInput
import com.ruru.cacao.domain.Period
import com.ruru.cacao.ui.AppViewModel
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
private fun Chip(text: String, selected: Boolean, tag: String, onClick: () -> Unit) {
    Text(
        text,
        modifier = Modifier
            .background(if (selected) Cacao.colors.primary else Cacao.colors.surface, RoundedCornerShape(50))
            .border(1.dp, Cacao.colors.outline, RoundedCornerShape(50))
            .clickable(onClick = onClick)
            .padding(horizontal = 12.dp, vertical = 8.dp)
            .testTag(tag),
        color = if (selected) Cacao.colors.onPrimary else Cacao.colors.onSurface,
        fontSize = 14.sp,
    )
}

@Composable
private fun periodLabel(period: Period): String = stringResource(
    when (period) {
        Period.DAILY -> R.string.period_daily
        Period.WEEKLY -> R.string.period_weekly
        Period.BIWEEKLY -> R.string.period_biweekly
        Period.MONTHLY -> R.string.period_monthly
    },
)

@Composable
private fun summary(s: Allowance_schedule, weekdays: List<String>): String = when (s.periodEnum) {
    Period.DAILY -> periodLabel(Period.DAILY)
    Period.WEEKLY, Period.BIWEEKLY -> stringResource(R.string.allowance_summary_weekly, periodLabel(s.periodEnum), weekdays[(s.weekday ?: 1).toInt() - 1])
    Period.MONTHLY -> stringResource(R.string.allowance_summary_monthly, (s.month_day ?: 1).toInt())
}

/** P07 常態零用錢 */
@Composable
fun AllowanceScreen(vm: AppViewModel, parentId: String, childId: String) {
    val child = vm.member(childId) ?: return
    val schedules = remember(vm.ledgerVersion, childId) { vm.allowance.schedules(childId) }
    val names = vm.memberList.associate { it.id to it.name }
    val sources = vm.memberList.filter { it.roleEnum == Role.PARENT || it.roleEnum == Role.ELDER }
    val weekdays = stringResource(R.string.weekday_short).split(",")
    val invalid = stringResource(R.string.allowance_invalid)
    val startInvalid = stringResource(R.string.allowance_start_invalid)

    var editingId by remember { mutableStateOf<String?>(null) }
    var showForm by remember { mutableStateOf(false) }
    var sourceId by remember { mutableStateOf(parentId) }
    var amount by remember { mutableStateOf("") }
    var period by remember { mutableStateOf(Period.WEEKLY) }
    var weekday by remember { mutableStateOf<Int?>(null) }
    var monthDay by remember { mutableStateOf("") }
    var startDate by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<String?>(null) }

    fun openForm(s: Allowance_schedule?) {
        editingId = s?.id
        sourceId = s?.source_id ?: parentId
        amount = s?.amount?.toString().orEmpty()
        period = s?.periodEnum ?: Period.WEEKLY
        weekday = s?.weekday?.toInt()
        monthDay = s?.month_day?.toString().orEmpty()
        startDate = ""
        error = null
        showForm = true
    }

    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            TextButton(onClick = { vm.go(Screen.ChildOverview(parentId, childId)) }, modifier = Modifier.testTag("back")) {
                Text("‹", color = Cacao.colors.primary, fontSize = 22.sp)
            }
            Title("${stringResource(R.string.allowance_title)}・${child.name}", Modifier.weight(1f))
        }
        Body(stringResource(R.string.allowance_hint))
        if (schedules.isEmpty()) Body(stringResource(R.string.allowance_empty))
        schedules.forEach { s ->
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                    .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
                    .clickable { openForm(s) }
                    .padding(14.dp)
                    .testTag("schedule_${names[s.source_id]}_${s.amount}"),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Column(Modifier.weight(1f)) {
                    Text("${names[s.source_id]}・${money(s.amount)}", fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
                    Text(summary(s, weekdays), fontSize = 13.sp, color = Cacao.colors.muted)
                }
                val paused = s.paused == 1L
                TextButton(
                    onClick = { vm.allowance.setPaused(s.id, !paused); vm.ledgerChanged() },
                    modifier = Modifier.testTag("schedule_toggle_${names[s.source_id]}_${s.amount}"),
                ) {
                    Text(stringResource(if (paused) R.string.allowance_resume else R.string.allowance_pause), color = Cacao.colors.primary)
                }
                Text(
                    stringResource(if (paused) R.string.allowance_paused else R.string.allowance_active),
                    fontSize = 12.sp,
                    color = if (paused) Cacao.colors.muted else Cacao.colors.secondary,
                    fontWeight = FontWeight.Bold,
                )
            }
        }

        if (!showForm) {
            TextButton(onClick = { openForm(null) }, modifier = Modifier.testTag("allowance_new")) {
                Text(stringResource(R.string.allowance_new), color = Cacao.colors.primary, fontWeight = FontWeight.Bold)
            }
        } else {
            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                Body(stringResource(R.string.income_source_label))
                FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    sources.forEach { m -> Chip(m.name, m.id == sourceId, "allowance_source_${m.name}") { sourceId = m.id } }
                }
                PlainField(stringResource(R.string.amount_label), amount, "allowance_amount", KeyboardType.Number) { amount = it.filter(Char::isDigit).take(6) }
                Body(stringResource(R.string.allowance_period))
                FlowRow(horizontalArrangement = Arrangement.spacedBy(6.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    Period.entries.forEach { p -> Chip(periodLabel(p), p == period, "period_${p.code}") { period = p } }
                }
                if (period == Period.WEEKLY || period == Period.BIWEEKLY) {
                    Body(stringResource(R.string.allowance_weekday))
                    FlowRow(horizontalArrangement = Arrangement.spacedBy(4.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
                        weekdays.forEachIndexed { i, label -> Chip(label, weekday == i + 1, "weekday_${i + 1}") { weekday = i + 1 } }
                    }
                }
                if (period == Period.MONTHLY) {
                    PlainField(stringResource(R.string.allowance_month_day), monthDay, "allowance_month_day", KeyboardType.Number) { monthDay = it.filter(Char::isDigit).take(2) }
                }
                if (editingId == null) {
                    PlainField(stringResource(R.string.allowance_start_label), startDate, "allowance_start", KeyboardType.Number) { startDate = it.take(10) }
                }
                ErrorText(error)
                PrimaryButton(stringResource(R.string.action_save), "allowance_save") {
                    val start = if (startDate.isBlank()) null else DateInput.parse(startDate)
                    if (startDate.isNotBlank() && (start == null || !vm.allowance.validStart(start))) {
                        error = startInvalid
                        return@PrimaryButton
                    }
                    val input = ScheduleInput(sourceId, amount.toLongOrNull() ?: 0, period, weekday, monthDay.toIntOrNull(), start)
                    val ok = editingId?.let { vm.allowance.update(it, input) } ?: vm.allowance.create(childId, input)
                    if (ok) {
                        showForm = false
                        vm.ledgerChanged()
                    } else {
                        error = invalid
                    }
                }
                editingId?.let { id ->
                    TextButton(onClick = { vm.allowance.delete(id); showForm = false; vm.ledgerChanged() }, modifier = Modifier.testTag("allowance_delete")) {
                        Text(stringResource(R.string.task_delete), color = Cacao.colors.danger)
                    }
                }
                TextButton(onClick = { showForm = false }) { Text(stringResource(R.string.action_cancel), color = Cacao.colors.primary) }
            }
        }
    }
}
