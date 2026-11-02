package com.ruru.cacao.ui.ledger

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Checkbox
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.R
import com.ruru.cacao.data.LedgerResult
import com.ruru.cacao.data.Role
import com.ruru.cacao.data.roleEnum
import com.ruru.cacao.db.EntriesForChild
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.ErrorText
import com.ruru.cacao.ui.components.PlainField
import com.ruru.cacao.ui.components.PrimaryButton
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.theme.Cacao
import java.text.NumberFormat
import java.util.Locale

fun money(amount: Long): String = "$" + NumberFormat.getIntegerInstance(Locale.US).format(amount)

@Composable
private fun BackRow(onBack: () -> Unit) {
    TextButton(onClick = onBack, modifier = Modifier.testTag("back")) {
        Text("‹ " + stringResource(R.string.back), color = Cacao.colors.primary)
    }
}

/** P02 孩子總覽 */
@Composable
fun ChildOverviewScreen(vm: AppViewModel, parentId: String, childId: String) {
    val child = vm.member(childId) ?: return
    val b = remember(vm.ledgerVersion, childId) { vm.ledger.balances(childId) }
    ScreenColumn {
        BackRow { vm.go(Screen.ParentHome(parentId)) }
        Title(child.name)
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .background(Cacao.colors.primary, RoundedCornerShape(Cacao.shape.radius))
                .padding(20.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text(stringResource(R.string.savings), color = Cacao.colors.onPrimary, fontSize = 14.sp)
            Text(money(b.savings), color = Cacao.colors.onPrimary, fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 36.sp, modifier = Modifier.testTag("overview_savings"))
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                Stat(stringResource(R.string.locked), b.locked, "overview_locked", Modifier.weight(1f))
                Stat(stringResource(R.string.available), b.available, "overview_available", Modifier.weight(1f))
                Stat(stringResource(R.string.wallet), b.wallet, "overview_wallet", Modifier.weight(1f))
            }
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            Button(
                onClick = { vm.go(Screen.AddIncome(parentId, childId)) },
                modifier = Modifier.weight(1f).height(52.dp).testTag("overview_income"),
                shape = RoundedCornerShape(14.dp),
                colors = ButtonDefaults.buttonColors(containerColor = Cacao.colors.primary, contentColor = Cacao.colors.onPrimary),
            ) { Text(stringResource(R.string.action_add_income), fontWeight = FontWeight.Bold) }
            OutlinedButton(
                onClick = { vm.go(Screen.Payout(parentId, childId)) },
                modifier = Modifier.weight(1f).height(52.dp).testTag("overview_payout"),
                shape = RoundedCornerShape(14.dp),
            ) { Text(stringResource(R.string.action_payout), fontWeight = FontWeight.Bold, color = Cacao.colors.primary) }
        }
        MenuRow(stringResource(R.string.overview_history), "overview_history") { vm.go(Screen.Ledger(parentId, childId)) }
    }
}

@Composable
private fun Stat(label: String, amount: Long, tag: String, modifier: Modifier) {
    Column(modifier) {
        Text(label, color = Cacao.colors.onPrimary, fontSize = 12.sp)
        Text(money(amount), color = Cacao.colors.onPrimary, fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 17.sp, modifier = Modifier.testTag(tag))
    }
}

@Composable
private fun MenuRow(text: String, tag: String, onClick: () -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
            .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
            .clickable(onClick = onClick)
            .padding(16.dp)
            .testTag(tag),
    ) {
        Text(text, fontSize = 16.sp, color = Cacao.colors.onSurface, modifier = Modifier.weight(1f))
        Text("›", color = Cacao.colors.muted)
    }
}

/** P03 記收入 */
@Composable
fun AddIncomeScreen(vm: AppViewModel, parentId: String, childId: String) {
    val child = vm.member(childId) ?: return
    var amount by remember { mutableStateOf("") }
    var sourceId by remember { mutableStateOf(parentId) }
    var note by remember { mutableStateOf("") }
    var message by remember { mutableStateOf("") }
    var cashToWallet by remember { mutableStateOf(false) }
    var addingElder by remember { mutableStateOf(false) }
    var elderName by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<String?>(null) }
    val sources = vm.memberList.filter { it.roleEnum == Role.PARENT || it.roleEnum == Role.ELDER }
    val invalid = stringResource(R.string.amount_invalid)

    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        BackRow { vm.go(Screen.ChildOverview(parentId, childId)) }
        Title(stringResource(R.string.income_title, child.name))
        PlainField(stringResource(R.string.amount_label), amount, "income_amount", KeyboardType.Number) { amount = it.filter(Char::isDigit).take(7) }
        Body(stringResource(R.string.income_source_label))
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp), modifier = Modifier.fillMaxWidth()) {
            sources.forEach { source ->
                val selected = source.id == sourceId
                Text(
                    source.name,
                    modifier = Modifier
                        .background(if (selected) Cacao.colors.primary else Cacao.colors.surface, RoundedCornerShape(50))
                        .border(1.dp, Cacao.colors.outline, RoundedCornerShape(50))
                        .clickable { sourceId = source.id }
                        .padding(horizontal = 16.dp, vertical = 10.dp)
                        .testTag("source_${source.name}"),
                    color = if (selected) Cacao.colors.onPrimary else Cacao.colors.onSurface,
                )
            }
        }
        if (addingElder) {
            PlainField(stringResource(R.string.elder_name_label), elderName, "elder_name") { elderName = it }
            PrimaryButton(stringResource(R.string.action_save), "elder_save", enabled = elderName.isNotBlank()) {
                sourceId = vm.members.createElder(elderName)
                vm.go(Screen.AddIncome(parentId, childId))
                addingElder = false
                elderName = ""
            }
        } else {
            TextButton(onClick = { addingElder = true }, modifier = Modifier.testTag("add_elder")) {
                Text(stringResource(R.string.income_add_elder), color = Cacao.colors.primary)
            }
        }
        PlainField(stringResource(R.string.note_label), note, "income_note") { note = it }
        PlainField(stringResource(R.string.message_label), message, "income_message") { message = it }
        Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.clickable { cashToWallet = !cashToWallet }.testTag("income_cash")) {
            Checkbox(checked = cashToWallet, onCheckedChange = { cashToWallet = it })
            Text(stringResource(R.string.cash_to_wallet), color = Cacao.colors.onSurface, fontSize = 14.sp)
        }
        ErrorText(error)
        Spacer(Modifier.height(8.dp))
        PrimaryButton(stringResource(R.string.action_save), "income_save") {
            val result = vm.ledger.addIncome(childId, amount.toLongOrNull() ?: 0, sourceId, parentId, note, message, cashToWallet = cashToWallet)
            if (result == LedgerResult.Ok) {
                vm.ledgerChanged()
                vm.go(Screen.ChildOverview(parentId, childId))
            } else {
                error = invalid
            }
        }
    }
}

/** P04 出金 */
@Composable
fun PayoutScreen(vm: AppViewModel, parentId: String, childId: String) {
    val child = vm.member(childId) ?: return
    val available = remember(vm.ledgerVersion, childId) { vm.ledger.balances(childId).available }
    var amount by remember { mutableStateOf("") }
    var note by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<String?>(null) }
    val invalid = stringResource(R.string.amount_invalid)
    val over = stringResource(R.string.payout_over, money(available))

    ScreenColumn {
        BackRow { vm.go(Screen.ChildOverview(parentId, childId)) }
        Title(stringResource(R.string.payout_title, child.name))
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                .padding(16.dp),
        ) {
            Text(stringResource(R.string.available), color = Cacao.colors.muted, modifier = Modifier.weight(1f))
            Text(money(available), fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface, modifier = Modifier.testTag("payout_available"))
        }
        PlainField(stringResource(R.string.amount_label), amount, "payout_amount", KeyboardType.Number) { amount = it.filter(Char::isDigit).take(7); error = null }
        PlainField(stringResource(R.string.payout_note_label), note, "payout_note") { note = it }
        Body(stringResource(R.string.payout_to_wallet, child.name))
        ErrorText(error)
        Spacer(Modifier.weight(1f))
        PrimaryButton(stringResource(R.string.action_payout), "payout_save") {
            when (vm.ledger.payout(childId, amount.toLongOrNull() ?: 0, parentId, note)) {
                LedgerResult.Ok -> {
                    vm.ledgerChanged()
                    vm.go(Screen.ChildOverview(parentId, childId))
                }
                is LedgerResult.OverAvailable -> error = over
                else -> error = invalid
            }
        }
    }
}

/** P05 存款明細（長按作廢） */
@OptIn(ExperimentalFoundationApi::class, ExperimentalComposeUiApi::class)
@Composable
fun LedgerScreen(vm: AppViewModel, parentId: String, childId: String) {
    val entries = remember(vm.ledgerVersion, childId) {
        vm.ledger.entries(childId).filter { it.account == "savings" }
    }
    var pendingVoid by remember { mutableStateOf<EntriesForChild?>(null) }
    var error by remember { mutableStateOf<String?>(null) }
    val negative = stringResource(R.string.void_negative)
    val names = vm.memberList.associate { it.id to it.name }

    ScreenColumn {
        BackRow { vm.go(Screen.ChildOverview(parentId, childId)) }
        Title(stringResource(R.string.ledger_title))
        ErrorText(error)
        if (entries.isEmpty()) Body(stringResource(R.string.ledger_empty))
        LazyColumn(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            items(entries, key = { it.id }) { entry ->
                val voided = entry.voided > 0
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius / 2))
                        .combinedClickable(onClick = {}, onLongClick = { if (!voided) pendingVoid = entry })
                        .padding(14.dp)
                        .testTag("entry_${entry.note ?: entry.kind}_${entry.amount}"),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Column(Modifier.weight(1f)) {
                        val title = listOfNotNull(entry.source_id?.let(names::get), entry.note ?: kindLabel(entry.kind)).joinToString("・")
                        Text(title, color = if (voided) Cacao.colors.muted else Cacao.colors.onSurface, textDecoration = if (voided) TextDecoration.LineThrough else null)
                        val sub = listOfNotNull(entry.occurred_on, entry.message?.let { "「$it」" }, if (voided) stringResource(R.string.ledger_voided) else null)
                        Text(sub.joinToString("・"), color = Cacao.colors.muted, fontSize = 13.sp)
                    }
                    Text(
                        (if (entry.amount > 0) "+" else "−") + money(kotlin.math.abs(entry.amount)).drop(1),
                        fontFamily = Cacao.fonts.number,
                        fontWeight = FontWeight.Bold,
                        color = when {
                            voided -> Cacao.colors.muted
                            entry.amount > 0 -> Cacao.colors.secondary
                            else -> Cacao.colors.onSurface
                        },
                        textDecoration = if (voided) TextDecoration.LineThrough else null,
                    )
                }
            }
        }
    }

    pendingVoid?.let { entry ->
        AlertDialog(
            onDismissRequest = { pendingVoid = null },
            title = { Text(stringResource(R.string.void_confirm_title)) },
            text = { Text(stringResource(R.string.void_confirm_body)) },
            confirmButton = {
                TextButton(onClick = {
                    when (vm.ledger.voidGroup(entry.group_id, parentId)) {
                        LedgerResult.Ok -> { error = null; vm.ledgerChanged() }
                        else -> error = negative
                    }
                    pendingVoid = null
                }, modifier = Modifier.semantics { testTagsAsResourceId = true }.testTag("void_confirm")) { Text(stringResource(R.string.action_void), color = Cacao.colors.danger) }
            },
            dismissButton = { TextButton(onClick = { pendingVoid = null }) { Text(stringResource(R.string.action_cancel)) } },
        )
    }
}

@Composable
private fun kindLabel(kind: String): String = stringResource(
    when (kind) {
        "income_allowance" -> R.string.kind_income_allowance
        "income_task" -> R.string.kind_income_task
        "payout" -> R.string.kind_payout
        "wallet_expense" -> R.string.kind_wallet_expense
        "wallet_adjustment" -> R.string.kind_wallet_adjustment
        else -> R.string.kind_income_onetime
    },
)
