package com.ruru.cacao.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
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
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.ui.theme.Cacao

@Composable
fun ScreenColumn(modifier: Modifier = Modifier, content: @Composable ColumnScope.() -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Cacao.colors.background)
            .then(modifier)
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

class TileItem(val title: String, val sub: String, val tint: Color, val tag: String, val onClick: (() -> Unit)?)

/** 彩色正方形按鈕，一列 3 個（UQ-09） */
@Composable
fun TileGrid(items: List<TileItem>, titleSize: TextUnit = 19.sp) {
    items.chunked(3).forEach { row ->
        Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
            row.forEach { Tile(it, titleSize, Modifier.weight(1f)) }
            repeat(3 - row.size) { Spacer(Modifier.weight(1f)) }
        }
    }
}

@Composable
private fun Tile(item: TileItem, titleSize: TextUnit, modifier: Modifier) {
    Column(
        modifier
            .aspectRatio(1f)
            .background(item.tint.copy(alpha = if (item.onClick == null) 0.15f else 0.35f), RoundedCornerShape(Cacao.shape.radius * 0.8f))
            .then(if (item.onClick != null) Modifier.clickable(onClick = item.onClick) else Modifier)
            .padding(horizontal = 12.dp, vertical = 10.dp)
            .testTag(item.tag),
        verticalArrangement = Arrangement.Bottom,
    ) {
        Text(item.title, fontWeight = FontWeight.Bold, fontSize = titleSize, color = Cacao.colors.onSurface)
        if (item.sub.isNotEmpty()) Text(item.sub, fontSize = 12.sp, color = Cacao.colors.muted, maxLines = 1)
    }
}
