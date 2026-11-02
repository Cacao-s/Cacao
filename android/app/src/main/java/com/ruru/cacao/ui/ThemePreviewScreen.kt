package com.ruru.cacao.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.ui.theme.Cacao
import com.ruru.cacao.ui.theme.CacaoTheme
import com.ruru.cacao.ui.theme.generated.GeneratedThemes

@Composable
fun ThemePreviewScreen() {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Color(0xFFEDEDED))
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        GeneratedThemes.all.forEach { tokens ->
            CacaoTheme(tokens) { ThemeSample() }
        }
    }
}

@Composable
private fun ThemeSample() {
    val c = Cacao.colors
    val shape = RoundedCornerShape(Cacao.shape.radius)
    val shadow = Cacao.shape.hardShadow
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .background(c.background, shape)
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        Text(Cacao.tokens.nameZh, fontFamily = Cacao.fonts.title, fontWeight = FontWeight.Bold, fontSize = 22.sp, color = c.onSurface)
        Box {
            if (shadow.value > 0f) {
                Box(Modifier.matchParentSize().offset(shadow, shadow).background(c.onSurface, shape))
            }
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(c.primary, shape)
                    .border(Cacao.shape.border, c.outline, shape)
                    .padding(16.dp),
            ) {
                Text("存款", fontSize = 14.sp, color = c.onPrimary)
                Text("$3,450", fontFamily = Cacao.fonts.number, fontWeight = FontWeight.Bold, fontSize = 34.sp, color = c.onPrimary)
            }
        }
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp), verticalAlignment = Alignment.CenterVertically) {
            Chip("已達成・鎖住", c.locked, c.onLocked)
            Chip("可出金額 $2,250", c.secondary, c.onSecondary)
        }
        Text("錢包 $120・小字顏色", fontSize = 14.sp, color = c.muted, fontFamily = Cacao.fonts.number)
    }
}

@Composable
private fun Chip(text: String, background: Color, content: Color) {
    Text(
        text,
        modifier = Modifier
            .background(background, RoundedCornerShape(50))
            .border(Cacao.shape.border, Cacao.colors.outline, RoundedCornerShape(50))
            .padding(horizontal = 12.dp, vertical = 6.dp),
        fontSize = 13.sp,
        fontWeight = FontWeight.Bold,
        fontFamily = Cacao.fonts.number,
        color = content,
    )
}
