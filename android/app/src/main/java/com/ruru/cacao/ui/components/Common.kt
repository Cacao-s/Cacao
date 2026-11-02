package com.ruru.cacao.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.ui.theme.Cacao

@Composable
fun ScreenColumn(modifier: Modifier = Modifier, content: @Composable ColumnScope.() -> Unit) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .background(Cacao.colors.background)
            .padding(horizontal = 24.dp, vertical = 32.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
        content = content,
    )
}

@Composable
fun Title(text: String, modifier: Modifier = Modifier) {
    Text(text, modifier = modifier, fontFamily = Cacao.fonts.title, fontWeight = FontWeight.Bold, fontSize = 26.sp, color = Cacao.colors.onSurface)
}

@Composable
fun Body(text: String, color: Color = Cacao.colors.muted) {
    Text(text, fontSize = 15.sp, lineHeight = 22.sp, color = color)
}

@Composable
fun PrimaryButton(text: String, tag: String, enabled: Boolean = true, onClick: () -> Unit) {
    Button(
        onClick = onClick,
        enabled = enabled,
        modifier = Modifier.fillMaxWidth().height(52.dp).testTag(tag),
        shape = RoundedCornerShape(14.dp),
        colors = ButtonDefaults.buttonColors(containerColor = Cacao.colors.primary, contentColor = Cacao.colors.onPrimary),
    ) { Text(text, fontSize = 17.sp, fontWeight = FontWeight.Bold) }
}

@Composable
fun SecretField(label: String, value: String, tag: String, onChange: (String) -> Unit) {
    OutlinedTextField(
        value = value,
        onValueChange = { onChange(it.filter(Char::isDigit).take(4)) },
        label = { Text(label) },
        singleLine = true,
        visualTransformation = PasswordVisualTransformation(),
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.NumberPassword),
        modifier = Modifier.fillMaxWidth().testTag(tag),
    )
}

@Composable
fun PlainField(label: String, value: String, tag: String, keyboard: KeyboardType = KeyboardType.Text, onChange: (String) -> Unit) {
    OutlinedTextField(
        value = value,
        onValueChange = onChange,
        label = { Text(label) },
        singleLine = true,
        keyboardOptions = KeyboardOptions(keyboardType = keyboard),
        modifier = Modifier.fillMaxWidth().testTag(tag),
    )
}

@Composable
fun ErrorText(text: String?) {
    if (text != null) Text(text, color = Cacao.colors.danger, fontSize = 14.sp)
}

private val avatarColors = mapOf(
    "peach" to Color(0xFFF2C6A0), "mint" to Color(0xFFBFD8C2), "lilac" to Color(0xFFD9CCF2),
    "sky" to Color(0xFFBFDDF2), "butter" to Color(0xFFF2E3A0), "rose" to Color(0xFFF2BFC8),
)

@Composable
fun Avatar(name: String, color: String, size: Int = 48) {
    Box(
        modifier = Modifier
            .size(size.dp)
            .background(avatarColors[color] ?: Color(0xFFD9C8B6), CircleShape)
            .border(Cacao.shape.border, Cacao.colors.outline, CircleShape),
        contentAlignment = Alignment.Center,
    ) {
        Text(name.take(1), fontWeight = FontWeight.Bold, fontSize = (size / 2.4).sp, color = Color(0xFF2A1A12))
    }
}
