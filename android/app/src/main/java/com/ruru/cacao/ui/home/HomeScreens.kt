package com.ruru.cacao.ui.home

import androidx.compose.foundation.background
import androidx.compose.foundation.border
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
import com.ruru.cacao.ui.components.ScreenColumn
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
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius))
                    .border(maxOf(Cacao.shape.border, 1.dp), Cacao.colors.outline, RoundedCornerShape(Cacao.shape.radius))
                    .padding(16.dp)
                    .testTag("child_row_${child.name}"),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(14.dp),
            ) {
                Avatar(child.name, child.avatar_color, 52)
                Column {
                    Text(child.name, fontSize = 18.sp, fontWeight = FontWeight.Bold, color = Cacao.colors.onSurface)
                    child.birthday?.let { Body(stringResource(R.string.age_years, ageOf(it))) }
                }
            }
        }
        Spacer(Modifier.weight(1f))
        TextButton(onClick = { vm.go(Screen.SelectUser) }, modifier = Modifier.testTag("parent_logout")) {
            Text(stringResource(R.string.logout_parent), color = Cacao.colors.primary)
        }
    }
}

/** C01 孩子首頁（M1 先做骨架，按鈕與餘額在 M2～M6 接上） */
@Composable
fun ChildHomeScreen(vm: AppViewModel, childId: String) {
    val child = vm.member(childId) ?: return
    ScreenColumn {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Title(stringResource(R.string.child_greeting, child.name), Modifier.weight(1f).testTag("child_home_title"))
            TextButton(onClick = { vm.go(Screen.Pin(PinPurpose.AnyParent)) }, modifier = Modifier.testTag("child_lock")) {
                Text("🔒 " + stringResource(R.string.switch_to_parent), color = Cacao.colors.primary, fontSize = 13.sp)
            }
        }
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

private fun ageOf(birthday: String): Int =
    runCatching { Period.between(LocalDate.parse(birthday), LocalDate.now()).years }.getOrDefault(0)
