package com.ruru.cacao.ui.identity

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.background
import androidx.compose.ui.platform.LocalContext
import com.ruru.cacao.ui.backup.ImportDialogs
import com.ruru.cacao.ui.backup.ImportState
import com.ruru.cacao.ui.backup.startImport
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Checkbox
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.R
import com.ruru.cacao.data.Role
import com.ruru.cacao.data.roleEnum
import com.ruru.cacao.db.Member
import com.ruru.cacao.security.AttemptLimiter
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.PinPurpose
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.components.Avatar
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.CuteLock
import com.ruru.cacao.ui.components.ErrorText
import com.ruru.cacao.ui.components.PinPad
import com.ruru.cacao.ui.components.PlainField
import com.ruru.cacao.ui.components.PrimaryButton
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.SecretField
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.theme.Cacao
import java.time.LocalDate
import kotlinx.coroutines.delay

private enum class OnboardingStep { WELCOME, PARENT, RECOVERY, CHILD }

@Composable
fun OnboardingScreen(vm: AppViewModel) {
    var step by remember { mutableStateOf(OnboardingStep.WELCOME) }
    var recovery by remember { mutableStateOf("") }

    when (step) {
        OnboardingStep.WELCOME -> ScreenColumn {
            Spacer(Modifier.weight(1f))
            Text("Cacao", fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 44.sp, color = Cacao.colors.primary, modifier = Modifier.align(Alignment.CenterHorizontally))
            Title(stringResource(R.string.onboarding_welcome), Modifier.align(Alignment.CenterHorizontally))
            Spacer(Modifier.weight(1f))
            PrimaryButton(stringResource(R.string.onboarding_new_family), "onboarding_new") { step = OnboardingStep.PARENT }
            val context = LocalContext.current
            val importState = remember { ImportState() }
            val picker = rememberLauncherForActivityResult(ActivityResultContracts.OpenDocument()) { uri ->
                if (uri != null) startImport(vm, importState, vm.backup.read(uri))
            }
            TextButton(onClick = { picker.launch(arrayOf("*/*")) }, modifier = Modifier.fillMaxWidth().testTag("onboarding_restore")) {
                Text(stringResource(R.string.onboarding_restore), color = Cacao.colors.primary)
            }
            ErrorText(importState.error?.let { stringResource(it) })
            ImportDialogs(vm, importState, context)
        }

        OnboardingStep.PARENT -> ParentForm(
            title = stringResource(R.string.parent_setup_title),
            showName = true,
        ) { name, pin ->
            recovery = vm.members.createParent(name, pin).second
            step = OnboardingStep.RECOVERY
        }

        OnboardingStep.RECOVERY -> RecoveryCodeView(
            title = stringResource(R.string.recovery_title),
            body = stringResource(R.string.recovery_body),
            code = recovery,
        ) { step = OnboardingStep.CHILD }

        OnboardingStep.CHILD -> ChildForm { name, birthday ->
            vm.members.createChild(name, birthday)
            vm.go(Screen.SelectUser)
        }
    }
}

@Composable
private fun ParentForm(title: String, showName: Boolean, onDone: (name: String, pin: String) -> Unit) {
    var name by remember { mutableStateOf("") }
    var pin by remember { mutableStateOf("") }
    var confirm by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<Int?>(null) }
    ScreenColumn {
        Title(title)
        Body(stringResource(R.string.parent_setup_body))
        if (showName) PlainField(stringResource(R.string.parent_name_label), name, "parent_name") { name = it }
        SecretField(stringResource(R.string.pin_label), pin, "pin_new") { pin = it }
        SecretField(stringResource(R.string.pin_confirm_label), confirm, "pin_confirm") { confirm = it }
        ErrorText(error?.let { stringResource(it) })
        Spacer(Modifier.weight(1f))
        PrimaryButton(stringResource(R.string.action_next), "parent_next") {
            error = when {
                showName && name.isBlank() -> R.string.name_required
                pin.length != 4 -> R.string.pin_need_4_digits
                pin != confirm -> R.string.pin_mismatch
                else -> null
            }
            if (error == null) onDone(name, pin)
        }
    }
}

@Composable
private fun RecoveryCodeView(title: String, body: String, code: String, onDone: () -> Unit) {
    var saved by remember { mutableStateOf(false) }
    ScreenColumn {
        Title(title)
        Body(body)
        Text(
            code,
            modifier = Modifier
                .fillMaxWidth()
                .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                .border(1.dp, Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
                .padding(vertical = 24.dp)
                .testTag("recovery_code"),
            textAlign = TextAlign.Center,
            fontFamily = Cacao.fonts.number,
            fontWeight = FontWeight.Bold,
            fontSize = 24.sp,
            letterSpacing = 1.sp,
            color = Cacao.colors.onSurface,
        )
        Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.clickable { saved = !saved }.testTag("recovery_saved")) {
            Checkbox(checked = saved, onCheckedChange = { saved = it })
            Text(stringResource(R.string.recovery_confirm), color = Cacao.colors.onSurface)
        }
        Spacer(Modifier.weight(1f))
        PrimaryButton(stringResource(R.string.action_next), "recovery_next", enabled = saved, onClick = onDone)
    }
}

@Composable
private fun ChildForm(onDone: (name: String, birthday: String) -> Unit) {
    var name by remember { mutableStateOf("") }
    var birthday by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<Int?>(null) }
    ScreenColumn {
        Title(stringResource(R.string.child_setup_title))
        PlainField(stringResource(R.string.child_name_label), name, "child_name") { name = it }
        PlainField(stringResource(R.string.child_birthday_label), birthday, "child_birthday") { birthday = it.trim() }
        ErrorText(error?.let { stringResource(it) })
        Spacer(Modifier.weight(1f))
        PrimaryButton(stringResource(R.string.action_done), "child_done") {
            val date = runCatching { LocalDate.parse(birthday) }.getOrNull()
            error = when {
                name.isBlank() -> R.string.name_required
                date == null || date.isAfter(LocalDate.now()) -> R.string.birthday_invalid
                else -> null
            }
            if (error == null) onDone(name, birthday)
        }
    }
}

@Composable
fun SelectUserScreen(vm: AppViewModel) {
    val parents = vm.memberList.filter { it.roleEnum == Role.PARENT }
    val children = vm.memberList.filter { it.roleEnum == Role.CHILD }
    ScreenColumn {
        Text("Cacao", fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 32.sp, color = Cacao.colors.primary, modifier = Modifier.align(Alignment.CenterHorizontally))
        Title(stringResource(R.string.who_is_using), Modifier.align(Alignment.CenterHorizontally))
        MemberGroup(stringResource(R.string.role_parent), parents, vm)
        MemberGroup(stringResource(R.string.role_child), children, vm)
    }
}

@Composable
private fun MemberGroup(label: String, members: List<Member>, vm: AppViewModel) {
    if (members.isEmpty()) return
    Body(label)
    members.chunked(2).forEach { row ->
        Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            row.forEach { member ->
                Column(
                    modifier = Modifier
                        .weight(1f)
                        .height(104.dp)
                        .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                        .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
                        .clickable { vm.selectMember(member) }
                        .testTag("member_${member.name}"),
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.Center,
                ) {
                    Avatar(member.name, member.avatar_color)
                    Row(
                        modifier = Modifier.padding(top = 6.dp),
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(6.dp),
                    ) {
                        Text(member.name, fontSize = 17.sp, fontWeight = FontWeight.Medium, color = Cacao.colors.onSurface)
                        if (member.password_hash != null) CuteLock(18.dp, stringResource(R.string.child_password_title))
                    }
                }
            }
            if (row.size == 1) Spacer(Modifier.weight(1f))
        }
    }
}

@Composable
fun PinScreen(vm: AppViewModel, purpose: PinPurpose) {
    var entered by remember(purpose) { mutableStateOf("") }
    var wrong by remember(purpose) { mutableStateOf(false) }
    var now by remember { mutableLongStateOf(System.currentTimeMillis()) }
    val waitSeconds = AttemptLimiter.remainingSeconds(vm.attemptState(purpose), now)
    LaunchedEffect(waitSeconds) {
        if (waitSeconds > 0) {
            delay(1000)
            now = System.currentTimeMillis()
        }
    }
    val title = when (purpose) {
        is PinPurpose.Parent -> stringResource(R.string.pin_title_parent, vm.member(purpose.parentId)?.name.orEmpty())
        PinPurpose.AnyParent -> stringResource(R.string.pin_title_any_parent)
        is PinPurpose.Child -> stringResource(R.string.pin_title_child, vm.member(purpose.childId)?.name.orEmpty())
        is PinPurpose.VerifyTask -> stringResource(R.string.task_verify_title, vm.tasks.task(purpose.taskId)?.title.orEmpty())
    }
    val backTo = if (purpose is PinPurpose.VerifyTask) Screen.ChildTasks(purpose.childId) else Screen.SelectUser
    ScreenColumn {
        TextButton(onClick = { vm.go(backTo) }, modifier = Modifier.testTag("pin_back")) {
            Text(stringResource(R.string.back), color = Cacao.colors.primary)
        }
        Spacer(Modifier.weight(1f))
        Title(title, Modifier.align(Alignment.CenterHorizontally))
        when {
            waitSeconds > 0 -> ErrorText(stringResource(R.string.pin_wait_seconds, waitSeconds))
            wrong -> ErrorText(stringResource(R.string.pin_wrong))
        }
        PinPad(
            entered = entered,
            enabled = waitSeconds == 0,
            deleteLabel = stringResource(R.string.back),
            onChange = { entered = it; wrong = false },
            onComplete = { pin ->
                val ok = vm.submitPin(purpose, pin)
                now = System.currentTimeMillis()
                if (!ok) { wrong = true; entered = "" }
            },
        )
        if (purpose is PinPurpose.Parent) {
            TextButton(onClick = { vm.go(Screen.ForgotPin(purpose.parentId)) }, modifier = Modifier.align(Alignment.CenterHorizontally).testTag("pin_forgot")) {
                Text(stringResource(R.string.pin_forgot), color = Cacao.colors.primary)
            }
        }
        Spacer(Modifier.weight(1f))
    }
}

private enum class ForgotStep { CHOOSE, HELPER_PIN, RECOVERY, NEW_PIN, NEW_CODE }

@Composable
fun ForgotPinScreen(vm: AppViewModel, parentId: String) {
    val otherParents = vm.memberList.filter { it.roleEnum == Role.PARENT && it.id != parentId }
    var step by remember { mutableStateOf(if (otherParents.isEmpty()) ForgotStep.RECOVERY else ForgotStep.CHOOSE) }
    var helperPin by remember { mutableStateOf("") }
    var recoveryInput by remember { mutableStateOf("") }
    var newCode by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<Int?>(null) }

    when (step) {
        ForgotStep.CHOOSE -> ScreenColumn {
            Title(stringResource(R.string.forgot_title))
            PrimaryButton(stringResource(R.string.forgot_by_parent), "forgot_by_parent") { step = ForgotStep.HELPER_PIN }
            PrimaryButton(stringResource(R.string.forgot_by_recovery), "forgot_by_recovery") { step = ForgotStep.RECOVERY }
            TextButton(onClick = { vm.go(Screen.SelectUser) }) { Text(stringResource(R.string.action_cancel), color = Cacao.colors.primary) }
        }

        ForgotStep.HELPER_PIN -> ScreenColumn {
            Title(stringResource(R.string.forgot_by_parent))
            ErrorText(error?.let { stringResource(it) })
            PinPad(helperPin, true, stringResource(R.string.back), { helperPin = it; error = null }) { pin ->
                if (otherParents.any { vm.members.verifyPin(it.id, pin) }) step = ForgotStep.NEW_PIN
                else { error = R.string.pin_wrong; helperPin = "" }
            }
        }

        ForgotStep.RECOVERY -> ScreenColumn {
            Title(stringResource(R.string.forgot_by_recovery))
            PlainField(stringResource(R.string.recovery_input_label), recoveryInput, "recovery_input") { recoveryInput = it; error = null }
            ErrorText(error?.let { stringResource(it) })
            Spacer(Modifier.weight(1f))
            PrimaryButton(stringResource(R.string.action_next), "recovery_input_next") { step = ForgotStep.NEW_PIN }
            TextButton(onClick = { vm.go(Screen.SelectUser) }) { Text(stringResource(R.string.action_cancel), color = Cacao.colors.primary) }
        }

        ForgotStep.NEW_PIN -> ParentForm(stringResource(R.string.new_pin_title), showName = false) { _, pin ->
            val code = if (helperPin.isNotEmpty()) {
                val helper = otherParents.first { vm.members.verifyPin(it.id, helperPin) }
                vm.members.resetPinByOtherParent(parentId, helper.id, helperPin, pin)
            } else {
                vm.members.resetPinByRecovery(parentId, recoveryInput, pin)
            }
            if (code == null) {
                error = R.string.recovery_wrong
                step = ForgotStep.RECOVERY
            } else {
                newCode = code
                step = ForgotStep.NEW_CODE
            }
        }

        ForgotStep.NEW_CODE -> RecoveryCodeView(
            title = stringResource(R.string.new_recovery_title),
            body = stringResource(R.string.new_recovery_body),
            code = newCode,
        ) { vm.go(Screen.SelectUser) }
    }
}

@Composable
fun ChildPasswordScreen(vm: AppViewModel, childId: String) {
    var pin by remember { mutableStateOf("") }
    var confirm by remember { mutableStateOf("") }
    var error by remember { mutableStateOf<Int?>(null) }
    ScreenColumn {
        Title(stringResource(R.string.child_password_title))
        SecretField(stringResource(R.string.pin_label), pin, "child_pw_new") { pin = it }
        SecretField(stringResource(R.string.pin_confirm_label), confirm, "child_pw_confirm") { confirm = it }
        ErrorText(error?.let { stringResource(it) })
        Spacer(Modifier.weight(1f))
        PrimaryButton(stringResource(R.string.action_done), "child_pw_done") {
            error = when {
                pin.length != 4 -> R.string.pin_need_4_digits
                pin != confirm -> R.string.pin_mismatch
                else -> null
            }
            if (error == null) {
                vm.members.setChildPassword(childId, pin)
                vm.go(Screen.ChildHome(childId))
            }
        }
        TextButton(onClick = { vm.go(Screen.ChildHome(childId)) }, modifier = Modifier.fillMaxWidth()) {
            Text(stringResource(R.string.action_cancel), color = Cacao.colors.primary)
        }
    }
}
