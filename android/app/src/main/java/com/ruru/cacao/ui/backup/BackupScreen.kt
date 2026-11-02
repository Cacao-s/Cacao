package com.ruru.cacao.ui.backup

import android.content.Context
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
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
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.R
import com.ruru.cacao.backup.BackupManager
import com.ruru.cacao.backup.ImportResult
import com.ruru.cacao.ui.AppViewModel
import com.ruru.cacao.ui.Screen
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.ErrorText
import com.ruru.cacao.ui.components.PlainField
import com.ruru.cacao.ui.components.PrimaryButton
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.theme.Cacao
import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter

@Composable
private fun Section(content: @Composable () -> Unit) {
    Column(
        Modifier
            .fillMaxWidth()
            .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
            .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) { content() }
}

/** 匯入流程（T02 與首次設定共用）：選檔 → 需要時輸入密碼 → 確認整個取代 → 取代後重新啟動 */
class ImportState {
    var pending by mutableStateOf<ByteArray?>(null)
    var needsPassword by mutableStateOf(false)
    var password by mutableStateOf("")
    var confirm by mutableStateOf(false)
    var error by mutableStateOf<Int?>(null)
}

private fun ImportResult.message(): Int? = when (this) {
    ImportResult.Ok, ImportResult.NeedsPassword -> null
    ImportResult.WrongPassword -> R.string.import_wrong_password
    ImportResult.TooNew -> R.string.import_too_new
    ImportResult.NotCacao, ImportResult.Corrupted -> R.string.import_not_cacao
}

fun startImport(vm: AppViewModel, state: ImportState, bytes: ByteArray) {
    state.error = null
    state.pending = bytes
    // 先用空密碼試讀，只為了知道要不要問密碼；真正取代在確認後
    val probe = com.ruru.cacao.backup.BackupCodec.unpack(bytes, null, com.ruru.cacao.db.CacaoDatabase.Schema.version)
    when (probe) {
        com.ruru.cacao.backup.UnpackResult.NeedsPassword -> state.needsPassword = true
        is com.ruru.cacao.backup.UnpackResult.Ok -> state.confirm = true
        com.ruru.cacao.backup.UnpackResult.NotCacao -> { state.error = R.string.import_not_cacao; state.pending = null }
        is com.ruru.cacao.backup.UnpackResult.TooNew -> { state.error = R.string.import_too_new; state.pending = null }
        com.ruru.cacao.backup.UnpackResult.WrongPassword -> state.needsPassword = true
    }
}

@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun ImportDialogs(vm: AppViewModel, state: ImportState, context: Context) {
    if (state.needsPassword) {
        AlertDialog(
            onDismissRequest = { state.needsPassword = false; state.pending = null },
            title = { Text(stringResource(R.string.backup_password)) },
            text = {
                Column {
                    PlainField(stringResource(R.string.backup_password), state.password, "import_password") { state.password = it }
                    ErrorText(state.error?.let { stringResource(it) })
                }
            },
            confirmButton = {
                TextButton(onClick = { state.needsPassword = false; state.confirm = true }, modifier = Modifier.semantics { testTagsAsResourceId = true }.testTag("import_password_ok")) {
                    Text(stringResource(R.string.action_next))
                }
            },
            dismissButton = { TextButton(onClick = { state.needsPassword = false; state.pending = null }) { Text(stringResource(R.string.action_cancel)) } },
        )
    }
    if (state.confirm) {
        AlertDialog(
            onDismissRequest = { state.confirm = false; state.pending = null },
            title = { Text(stringResource(R.string.backup_import_confirm_title)) },
            text = { Text(stringResource(R.string.backup_import_confirm_body)) },
            confirmButton = {
                TextButton(
                    onClick = {
                        state.confirm = false
                        val bytes = state.pending ?: return@TextButton
                        val result = vm.backup.import(bytes, state.password.ifEmpty { null })
                        if (result == ImportResult.Ok) {
                            BackupManager.restartApp(context)
                        } else {
                            state.error = result.message()
                            if (result == ImportResult.WrongPassword) state.needsPassword = true
                        }
                    },
                    modifier = Modifier.semantics { testTagsAsResourceId = true }.testTag("import_replace"),
                ) { Text(stringResource(R.string.backup_import_replace), color = Cacao.colors.danger) }
            },
            dismissButton = { TextButton(onClick = { state.confirm = false; state.pending = null }) { Text(stringResource(R.string.action_cancel)) } },
        )
    }
}

/** T02 備份與還原 */
@Composable
fun BackupScreen(vm: AppViewModel, parentId: String) {
    val context = LocalContext.current
    var exportPassword by remember { mutableStateOf("") }
    var exported by remember { mutableStateOf(false) }
    var showCloudList by remember { mutableStateOf(false) }
    val importState = remember { ImportState() }

    val exportLauncher = rememberLauncherForActivityResult(ActivityResultContracts.CreateDocument("application/octet-stream")) { uri ->
        if (uri != null) {
            vm.backup.exportTo(uri, exportPassword.ifEmpty { null })
            vm.markExported()
            exported = true
        }
    }
    val importLauncher = rememberLauncherForActivityResult(ActivityResultContracts.OpenDocument()) { uri ->
        if (uri != null) startImport(vm, importState, vm.backup.read(uri))
    }

    ScreenColumn(Modifier.verticalScroll(rememberScrollState())) {
        TextButton(onClick = { vm.go(Screen.Settings(parentId)) }, modifier = Modifier.testTag("back")) {
            Text("‹", color = Cacao.colors.primary, fontSize = 22.sp)
        }
        Title(stringResource(R.string.settings_backup))
        if (vm.lastBackupMillis == 0L) {
            Column(Modifier.fillMaxWidth().background(Cacao.colors.locked.copy(alpha = 0.25f), RoundedCornerShape(Cacao.shape.radius)).padding(16.dp)) {
                Text(stringResource(R.string.backup_none), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface, modifier = Modifier.testTag("backup_status_none"))
                Body(stringResource(R.string.backup_none_body))
            }
        } else {
            val time = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm").withZone(ZoneId.systemDefault()).format(Instant.ofEpochMilli(vm.lastBackupMillis))
            Body(stringResource(R.string.backup_last, time))
        }

        Section {
            Text(stringResource(R.string.backup_google), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
            Body(stringResource(R.string.backup_google_body))
            vm.cloudStatus?.let { ErrorText(stringResource(R.string.backup_error, it)) }
            if (!vm.cloudEnabled) {
                PrimaryButton(stringResource(R.string.backup_google_enable), "cloud_enable") { vm.enableCloud() }
            } else {
                PrimaryButton(stringResource(R.string.backup_now), "cloud_backup_now") { vm.backupNow() }
                TextButton(onClick = { showCloudList = true; vm.loadCloudFiles() }, modifier = Modifier.testTag("cloud_restore")) {
                    Text(stringResource(R.string.backup_google_restore), color = Cacao.colors.primary)
                }
                if (showCloudList) {
                    vm.cloudFiles.forEach { f ->
                        Text(
                            f.createdTime.take(16).replace("T", " "),
                            modifier = Modifier.fillMaxWidth().clickable { vm.downloadCloud(f) { startImport(vm, importState, it) } }.padding(vertical = 8.dp),
                            color = Cacao.colors.onSurface,
                        )
                    }
                }
                TextButton(onClick = { vm.disableCloud() }, modifier = Modifier.testTag("cloud_disable")) {
                    Text(stringResource(R.string.backup_google_disable), color = Cacao.colors.danger)
                }
            }
        }

        Section {
            Text(stringResource(R.string.backup_export), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
            PlainField(stringResource(R.string.backup_password_optional), exportPassword, "export_password") { exportPassword = it }
            if (exportPassword.isNotEmpty()) Body(stringResource(R.string.backup_password_warning), Cacao.colors.danger)
            PrimaryButton(stringResource(R.string.backup_export), "backup_export") {
                exported = false
                exportLauncher.launch("cacao-${java.time.LocalDate.now()}.cacao")
            }
            if (exported) Body(stringResource(R.string.backup_exported))
        }

        Section {
            Text(stringResource(R.string.backup_import), fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
            ErrorText(importState.error?.let { stringResource(it) })
            PrimaryButton(stringResource(R.string.backup_import), "backup_import") { importLauncher.launch(arrayOf("*/*")) }
            if (vm.backup.hasPreImportBackup()) {
                TextButton(onClick = { if (vm.backup.restorePreImport() == ImportResult.Ok) BackupManager.restartApp(context) }, modifier = Modifier.testTag("backup_restore_previous")) {
                    Text(stringResource(R.string.backup_restore_previous), color = Cacao.colors.primary)
                }
            }
        }
    }
    ImportDialogs(vm, importState, context)
}
