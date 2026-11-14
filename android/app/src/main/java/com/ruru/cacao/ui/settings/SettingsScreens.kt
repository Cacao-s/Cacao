package com.ruru.cacao.ui.settings

import androidx.appcompat.app.AppCompatDelegate
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.core.os.LocaleListCompat
import com.ruru.cacao.domain.BirthdayInput
import com.ruru.cacao.R
import com.ruru.cacao.data.Role
import com.ruru.cacao.data.roleEnum
import com.ruru.cacao.db.Member
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.ErrorText
import com.ruru.cacao.ui.components.PlainField
import com.ruru.cacao.ui.components.PrimaryButton
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.SecretField
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.theme.Cacao
import com.ruru.cacao.ui.theme.generated.GeneratedThemes
import java.time.LocalDate

@Composable
private fun Header(title: String, onBack: () -> Unit) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        TextButton(onClick = onBack, modifier = Modifier.testTag("back")) { Text("‹", color = Cacao.colors.primary, fontSize = 22.sp) }
        Title(title)
    }
}

@Composable
private fun Chip(text: String, selected: Boolean, tag: String, onClick: () -> Unit) {
    Text(
        text,
        modifier = Modifier
            .background(if (selected) Cacao.colors.primary else Cacao.colors.surface, RoundedCornerShape(50))
            .border(1.dp, Cacao.colors.outline, RoundedCornerShape(50))
            .clickable(onClick = onClick)
            .padding(horizontal = 14.dp, vertical = 10.dp)
            .testTag(tag),
        color = if (selected) Cacao.colors.onPrimary else Cacao.colors.onSurface,
        fontSize = 14.sp,
    )
}

@Composable
private fun Section(content: @Composable () -> Unit) {
    Column(
        Modifier
            .fillMaxWidth()
            .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
            .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) { content() }
}

/** T01 設定（只有父母模式能進） */
@Composable
fun SettingsScreen(vm: AppViewModel, parentId: String) {
    val currentLocale = AppCompatDelegate.getApplicationLocales().toLanguageTags()
    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Header(stringResource(R.string.settings_title)) { vm.go(Screen.ParentHome(parentId)) }
        Body(stringResource(R.string.settings_device))
        Section {
            Text(stringResource(R.string.theme_label), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
            GeneratedThemes.all.chunked(2).forEach { row ->
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    row.forEach { t -> Chip(t.nameZh, vm.themeId == t.id, "theme_${t.id}") { vm.setTheme(t.id) } }
                }
            }
            Text(stringResource(R.string.language_label), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                listOf("" to stringResource(R.string.language_system), "zh-Hant" to "繁體中文", "en" to "English").forEach { (tag, label) ->
                    Chip(label, currentLocale == tag, "lang_${tag.ifEmpty { "system" }}") {
                        AppCompatDelegate.setApplicationLocales(
                            if (tag.isEmpty()) LocaleListCompat.getEmptyLocaleList() else LocaleListCompat.forLanguageTags(tag),
                        )
                    }
                }
            }
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(stringResource(R.string.sound_label), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface, modifier = Modifier.weight(1f))
                Switch(checked = vm.soundOn, onCheckedChange = vm::setSound, modifier = Modifier.testTag("sound_switch"))
            }
            Body(stringResource(R.string.settings_device_note))
        }
        Body(stringResource(R.string.settings_family_data))
        Section {
            Row(Modifier.fillMaxWidth().clickable { vm.go(Screen.Family(parentId)) }.testTag("settings_family")) {
                Text(stringResource(R.string.family_title), color = Cacao.colors.onSurface, modifier = Modifier.weight(1f))
                Text("›", color = Cacao.colors.muted)
            }
            Row(Modifier.fillMaxWidth().clickable { vm.go(Screen.Backup(parentId)) }.testTag("settings_backup")) {
                Text(stringResource(R.string.settings_backup), color = Cacao.colors.onSurface, modifier = Modifier.weight(1f))
                Text("›", color = Cacao.colors.muted)
            }
        }
    }
}

private enum class AddKind { NONE, PARENT, ELDER, CHILD }

/** P09 家庭成員 */
@Composable
fun FamilyScreen(vm: AppViewModel, parentId: String) {
    var adding by remember { mutableStateOf(AddKind.NONE) }
    var renaming by remember { mutableStateOf<Member?>(null) }
    var name by remember { mutableStateOf("") }
    var birthday by remember { mutableStateOf("") }
    var pin by remember { mutableStateOf("") }
    var confirm by remember { mutableStateOf("") }
    var shownCode by remember { mutableStateOf<String?>(null) }
    var error by remember { mutableStateOf<Int?>(null) }

    fun reset() { adding = AddKind.NONE; renaming = null; name = ""; birthday = ""; pin = ""; confirm = ""; error = null }

    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        Header(stringResource(R.string.family_title)) { vm.go(Screen.Settings(parentId)) }
        shownCode?.let { code ->
            Section {
                Text(stringResource(R.string.recovery_title), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
                Body(stringResource(R.string.recovery_body))
                Text(code, fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 22.sp, color = Cacao.colors.onSurface, modifier = Modifier.testTag("recovery_code"))
                TextButton(onClick = { shownCode = null }, modifier = Modifier.testTag("recovery_close")) { Text(stringResource(R.string.recovery_confirm), color = Cacao.colors.primary) }
            }
        }
        listOf(Role.PARENT to R.string.role_parent, Role.ELDER to R.string.role_elder, Role.CHILD to R.string.role_child).forEach { (role, label) ->
            Body(stringResource(label))
            Section {
                val people = vm.memberList.filter { it.roleEnum == role }
                val sides = vm.members.elderSides()
                if (people.isEmpty()) Body(stringResource(R.string.family_empty))
                people.forEach { m ->
                    Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.testTag("family_${m.name}")) {
                        Column(Modifier.weight(1f)) {
                            Text(m.name, color = Cacao.colors.onSurface, fontWeight = FontWeight.Medium)
                            if (role == Role.ELDER) {
                                Text(
                                    sides[m.id]?.let { stringResource(R.string.elder_side_of, vm.member(it)?.name.orEmpty()) } ?: stringResource(R.string.elder_side_none),
                                    fontSize = 13.sp,
                                    color = Cacao.colors.muted,
                                    modifier = Modifier.testTag("elder_side_${m.name}"),
                                )
                            }
                            if (role == Role.CHILD) {
                                Text(
                                    listOfNotNull(m.birthday, stringResource(if (m.password_hash != null) R.string.family_has_password else R.string.family_no_password)).joinToString("・"),
                                    fontSize = 13.sp,
                                    color = Cacao.colors.muted,
                                )
                            }
                        }
                        if (role == Role.ELDER && sides[m.id] == null) {
                            TextButton(onClick = { vm.members.setElderSide(m.id, parentId); vm.go(Screen.Family(parentId)) }, modifier = Modifier.testTag("elder_claim_${m.name}")) {
                                Text(stringResource(R.string.elder_claim), color = Cacao.colors.primary, fontSize = 13.sp)
                            }
                        }
                        if (role == Role.CHILD && m.password_hash != null) {
                            TextButton(onClick = { vm.members.setChildPassword(m.id, null); vm.go(Screen.Family(parentId)) }, modifier = Modifier.testTag("family_clear_pw_${m.name}")) {
                                Text(stringResource(R.string.family_clear_child_password), color = Cacao.colors.primary, fontSize = 13.sp)
                            }
                        }
                        TextButton(onClick = { reset(); renaming = m; name = m.name }, modifier = Modifier.testTag("family_rename_${m.name}")) {
                            Text(stringResource(R.string.family_rename), color = Cacao.colors.primary, fontSize = 13.sp)
                        }
                    }
                }
            }
        }

        renaming?.let { m ->
            Section {
                PlainField(stringResource(R.string.child_name_label), name, "rename_input") { name = it }
                PrimaryButton(stringResource(R.string.action_save), "rename_save") {
                    if (vm.members.rename(m.id, name)) { reset(); vm.go(Screen.Family(parentId)) } else error = R.string.name_required
                }
            }
        }

        when (adding) {
            AddKind.NONE -> if (renaming == null) {
                TextButton(onClick = { reset(); adding = AddKind.PARENT }, modifier = Modifier.testTag("family_add_parent")) { Text(stringResource(R.string.family_add_parent), color = Cacao.colors.primary) }
                TextButton(onClick = { reset(); adding = AddKind.ELDER }, modifier = Modifier.testTag("family_add_elder")) { Text(stringResource(R.string.family_add_elder), color = Cacao.colors.primary) }
                TextButton(onClick = { reset(); adding = AddKind.CHILD }, modifier = Modifier.testTag("family_add_child")) { Text(stringResource(R.string.family_add_child), color = Cacao.colors.primary) }
                TextButton(onClick = { shownCode = vm.members.regenerateRecovery(parentId) }, modifier = Modifier.testTag("family_regenerate")) {
                    Text(stringResource(R.string.family_regenerate_recovery), color = Cacao.colors.primary)
                }
            }
            else -> Section {
                PlainField(stringResource(R.string.child_name_label), name, "add_name") { name = it }
                if (adding == AddKind.CHILD) PlainField(stringResource(R.string.child_birthday_label), birthday, "add_birthday", KeyboardType.Number) { birthday = it.trim() }
                if (adding == AddKind.PARENT) {
                    SecretField(stringResource(R.string.pin_label), pin, "add_pin") { pin = it }
                    SecretField(stringResource(R.string.pin_confirm_label), confirm, "add_pin_confirm") { confirm = it }
                }
                ErrorText(error?.let { stringResource(it) })
                PrimaryButton(stringResource(R.string.action_save), "add_save") {
                    val date = BirthdayInput.parse(birthday)
                    error = when {
                        name.isBlank() -> R.string.name_required
                        adding == AddKind.CHILD && date == null -> R.string.birthday_invalid
                        adding == AddKind.PARENT && pin.length != 4 -> R.string.pin_need_4_digits
                        adding == AddKind.PARENT && pin != confirm -> R.string.pin_mismatch
                        else -> null
                    }
                    if (error == null) {
                        when (adding) {
                            AddKind.PARENT -> shownCode = vm.members.createParent(name, pin).second
                            AddKind.ELDER -> vm.members.createElder(name, parentId)
                            AddKind.CHILD -> vm.members.createChild(name, date.toString())
                            AddKind.NONE -> Unit
                        }
                        reset()
                        vm.go(Screen.Family(parentId))
                    }
                }
                TextButton(onClick = { reset() }) { Text(stringResource(R.string.action_cancel), color = Cacao.colors.primary) }
            }
        }
    }
}
