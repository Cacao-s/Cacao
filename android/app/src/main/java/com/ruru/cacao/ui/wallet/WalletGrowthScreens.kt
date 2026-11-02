package com.ruru.cacao.ui.wallet

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.R
import com.ruru.cacao.data.LedgerResult
import com.ruru.cacao.data.ageYear
import com.ruru.cacao.db.EntriesForChild
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
import java.time.LocalDate
import java.time.Period

private fun back(vm: AppViewModel, childId: String, parentId: String?) =
    vm.go(parentId?.let { Screen.ChildOverview(it, childId) } ?: Screen.ChildHome(childId))

@Composable
private fun Header(title: String, onBack: () -> Unit) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        TextButton(onClick = onBack, modifier = Modifier.testTag("back")) { Text("‹", color = Cacao.colors.primary, fontSize = 22.sp) }
        Title(title)
    }
}

private enum class WalletForm { NONE, EXPENSE, CALIBRATE }

/** C05／P06 錢包：孩子記花費、校正；父母另外可以長按作廢 */
@OptIn(ExperimentalFoundationApi::class, ExperimentalComposeUiApi::class)
@Composable
fun WalletScreen(vm: AppViewModel, childId: String, parentId: String?) {
    val child = vm.member(childId) ?: return
    val balance = remember(vm.ledgerVersion, childId) { vm.ledger.balances(childId).wallet }
    val entries = remember(vm.ledgerVersion, childId) { vm.ledger.entries(childId).filter { it.account == "wallet" } }
    val actor = parentId ?: childId
    var form by remember { mutableStateOf(WalletForm.NONE) }
    var amount by remember { mutableStateOf("") }
    var note by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<String?>(null) }
    var pendingVoid by remember { mutableStateOf<EntriesForChild?>(null) }
    val invalid = stringResource(R.string.amount_invalid)

    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Header(if (parentId != null) stringResource(R.string.wallet_title_child, child.name) else stringResource(R.string.wallet_title)) { back(vm, childId, parentId) }
        Column(
            Modifier
                .fillMaxWidth()
                .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
                .padding(20.dp),
        ) {
            Text(stringResource(R.string.wallet_left), color = Cacao.colors.muted, fontSize = 14.sp)
            Text(money(balance), fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 36.sp, color = Cacao.colors.onSurface, modifier = Modifier.testTag("wallet_balance"))
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            Button(
                onClick = { form = WalletForm.EXPENSE; amount = ""; note = ""; error = null },
                modifier = Modifier.weight(1f).height(52.dp).testTag("wallet_expense"),
                shape = RoundedCornerShape(14.dp),
                colors = ButtonDefaults.buttonColors(containerColor = Cacao.colors.primary, contentColor = Cacao.colors.onPrimary),
            ) { Text(stringResource(R.string.wallet_add_expense), fontWeight = FontWeight.Bold) }
            OutlinedButton(
                onClick = { form = WalletForm.CALIBRATE; amount = ""; error = null },
                modifier = Modifier.weight(1f).height(52.dp).testTag("wallet_calibrate"),
                shape = RoundedCornerShape(14.dp),
            ) { Text(stringResource(R.string.wallet_calibrate), fontWeight = FontWeight.Bold, color = Cacao.colors.primary) }
        }
        when (form) {
            WalletForm.EXPENSE -> {
                PlainField(stringResource(R.string.amount_label), amount, "expense_amount", KeyboardType.Number) { amount = it.filter(Char::isDigit).take(6) }
                PlainField(stringResource(R.string.payout_note_label), note, "expense_note") { note = it }
                ErrorText(error)
                PrimaryButton(stringResource(R.string.action_save), "expense_save") {
                    if (vm.ledger.addWalletExpense(childId, amount.toLongOrNull() ?: 0, actor, note) == LedgerResult.Ok) {
                        form = WalletForm.NONE; vm.ledgerChanged()
                    } else error = invalid
                }
            }
            WalletForm.CALIBRATE -> {
                PlainField(stringResource(R.string.wallet_calibrate_label), amount, "calibrate_amount", KeyboardType.Number) { amount = it.filter(Char::isDigit).take(6) }
                ErrorText(error)
                PrimaryButton(stringResource(R.string.action_save), "calibrate_save") {
                    val actual = amount.toLongOrNull()
                    if (actual != null && vm.ledger.calibrateWallet(childId, actual, actor) == LedgerResult.Ok) {
                        form = WalletForm.NONE; vm.ledgerChanged()
                    } else error = invalid
                }
            }
            WalletForm.NONE -> Unit
        }
        Body(stringResource(if (parentId != null) R.string.wallet_parent_hint else R.string.wallet_hint))
        entries.forEach { e ->
            val voided = e.voided > 0
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius / 2))
                    .combinedClickable(onClick = {}, onLongClick = { if (parentId != null && !voided) pendingVoid = e })
                    .padding(14.dp)
                    .testTag("wallet_entry_${e.note ?: e.kind}_${e.amount}"),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Column(Modifier.weight(1f)) {
                    Text(e.note ?: kindLabel(e.kind), color = if (voided) Cacao.colors.muted else Cacao.colors.onSurface, textDecoration = if (voided) TextDecoration.LineThrough else null)
                    Text(e.occurred_on + if (voided) "・" + stringResource(R.string.ledger_voided) else "", color = Cacao.colors.muted, fontSize = 13.sp)
                }
                Text(
                    (if (e.amount > 0) "+" else "−") + money(kotlin.math.abs(e.amount)).drop(1),
                    fontFamily = Cacao.fonts.number,
                    fontWeight = FontWeight.Bold,
                    color = if (voided) Cacao.colors.muted else if (e.amount > 0) Cacao.colors.secondary else Cacao.colors.onSurface,
                    textDecoration = if (voided) TextDecoration.LineThrough else null,
                )
            }
        }
    }

    pendingVoid?.let { e ->
        AlertDialog(
            onDismissRequest = { pendingVoid = null },
            title = { Text(stringResource(R.string.void_confirm_title)) },
            text = { Text(stringResource(R.string.void_confirm_body)) },
            confirmButton = {
                TextButton(
                    onClick = { vm.ledger.voidGroup(e.group_id, parentId!!); vm.ledgerChanged(); pendingVoid = null },
                    modifier = Modifier.semantics { testTagsAsResourceId = true }.testTag("void_confirm"),
                ) { Text(stringResource(R.string.action_void), color = Cacao.colors.danger) }
            },
            dismissButton = { TextButton(onClick = { pendingVoid = null }) { Text(stringResource(R.string.action_cancel)) } },
        )
    }
}

@Composable
private fun kindLabel(kind: String): String = stringResource(
    when (kind) {
        "payout" -> R.string.kind_payout
        "wallet_adjustment" -> R.string.kind_wallet_adjustment
        else -> R.string.kind_wallet_expense
    },
)

/** C06 成長紀錄（簡單版）：依生日切「N 歲這一年」 */
@Composable
fun GrowthScreen(vm: AppViewModel, childId: String, parentId: String?) {
    val child = vm.member(childId) ?: return
    val birthday = child.birthday?.let(LocalDate::parse) ?: return
    val currentAge = Period.between(birthday, LocalDate.now()).years.coerceAtLeast(0)
    var age by remember { mutableIntStateOf(currentAge) }
    val growth = remember(vm.ledgerVersion, childId, age) {
        val (from, to) = ageYear(birthday, age)
        vm.ledger.growth(childId, from, to)
    }
    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Header(stringResource(R.string.growth_title)) { back(vm, childId, parentId) }
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            listOf(currentAge, currentAge - 1).filter { it >= 0 }.forEach { a ->
                val selected = a == age
                Text(
                    stringResource(R.string.growth_age_year, a),
                    modifier = Modifier
                        .background(if (selected) Cacao.colors.primary else Cacao.colors.surface, RoundedCornerShape(50))
                        .border(1.dp, Cacao.colors.outline, RoundedCornerShape(50))
                        .clickable { age = a }
                        .padding(horizontal = 16.dp, vertical = 10.dp)
                        .testTag("growth_age_$a"),
                    color = if (selected) Cacao.colors.onPrimary else Cacao.colors.onSurface,
                )
            }
        }
        Column(
            Modifier
                .fillMaxWidth()
                .background(Cacao.colors.primary, RoundedCornerShape(Cacao.shape.radius))
                .padding(24.dp),
        ) {
            Text(stringResource(R.string.growth_saved), color = Cacao.colors.onPrimary, fontSize = 15.sp)
            Text(money(growth.saved), color = Cacao.colors.onPrimary, fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 40.sp, modifier = Modifier.testTag("growth_saved"))
        }
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            Stat(stringResource(R.string.growth_tasks), stringResource(R.string.growth_count, growth.tasksDone.toInt()), "growth_tasks", Modifier.weight(1f))
            Stat(stringResource(R.string.growth_goals), stringResource(R.string.growth_count, growth.goalsDone.toInt()), "growth_goals", Modifier.weight(1f))
        }
        Body(stringResource(R.string.growth_hint))
    }
}

@Composable
private fun Stat(label: String, value: String, tag: String, modifier: Modifier) {
    Column(
        modifier
            .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
            .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
            .padding(16.dp),
    ) {
        Text(label, color = Cacao.colors.muted, fontSize = 14.sp)
        Text(value, fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 26.sp, color = Cacao.colors.onSurface, modifier = Modifier.testTag(tag))
    }
}
