package com.ruru.cacao.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.ruru.cacao.ui.theme.Cacao

/** 4 位數輸入盤；輸滿 4 位時呼叫 onComplete */
@Composable
fun PinPad(entered: String, enabled: Boolean, deleteLabel: String, onChange: (String) -> Unit, onComplete: (String) -> Unit) {
    Column(horizontalAlignment = Alignment.CenterHorizontally, verticalArrangement = Arrangement.spacedBy(20.dp)) {
        Row(horizontalArrangement = Arrangement.spacedBy(16.dp), modifier = Modifier.testTag("pin_dots")) {
            repeat(4) { i ->
                val filled = i < entered.length
                Box(
                    Modifier
                        .size(16.dp)
                        .background(if (filled) Cacao.colors.primary else Cacao.colors.background, CircleShape)
                        .border(2.dp, Cacao.colors.primary, CircleShape),
                )
            }
        }
        val keys = listOf("1", "2", "3", "4", "5", "6", "7", "8", "9", "", "0", "del")
        keys.chunked(3).forEach { row ->
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp), modifier = Modifier.fillMaxWidth()) {
                row.forEach { key ->
                    Box(
                        modifier = Modifier
                            .weight(1f)
                            .height(60.dp)
                            .background(if (key.isEmpty()) Cacao.colors.background else Cacao.colors.surface, RoundedCornerShape(Cacao.shape.radius / 2))
                            .then(
                                if (key.isEmpty() || !enabled) Modifier else Modifier
                                    .testTag("pin_key_$key")
                                    .semantics { if (key == "del") contentDescription = deleteLabel }
                                    .clickable {
                                        when (key) {
                                            "del" -> onChange(entered.dropLast(1))
                                            else -> if (entered.length < 4) {
                                                val next = entered + key
                                                onChange(next)
                                                if (next.length == 4) onComplete(next)
                                            }
                                        }
                                    },
                            ),
                        contentAlignment = Alignment.Center,
                    ) {
                        Text(
                            if (key == "del") "⌫" else key,
                            fontFamily = Cacao.fonts.number,
                            fontWeight = FontWeight.Medium,
                            fontSize = 26.sp,
                            color = if (enabled) Cacao.colors.onSurface else Cacao.colors.muted,
                        )
                    }
                }
            }
        }
    }
}
