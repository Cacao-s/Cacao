package com.ruru.cacao.ui.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.ruru.cacao.ui.theme.Cacao

/** 圓胖鎖身＋愛心鑰匙孔，顏色跟主題走 */
@Composable
fun CuteLock(size: Dp = 20.dp, description: String? = null) {
    val body = Cacao.colors.locked
    val ink = Cacao.colors.onLocked
    val shackle = Cacao.colors.muted
    Canvas(
        Modifier
            .size(size)
            .then(if (description != null) Modifier.semantics { contentDescription = description } else Modifier),
    ) {
        val w = this.size.width
        val h = this.size.height
        val stroke = w * 0.13f

        // 鎖環：上半圓弧
        drawArc(
            color = shackle,
            startAngle = 180f,
            sweepAngle = 180f,
            useCenter = false,
            topLeft = Offset(w * 0.24f, h * 0.06f),
            size = Size(w * 0.52f, h * 0.56f),
            style = Stroke(width = stroke, cap = StrokeCap.Round),
        )
        drawLine(shackle, Offset(w * 0.24f, h * 0.34f), Offset(w * 0.24f, h * 0.46f), stroke, StrokeCap.Round)
        drawLine(shackle, Offset(w * 0.76f, h * 0.34f), Offset(w * 0.76f, h * 0.46f), stroke, StrokeCap.Round)

        // 鎖身：圓角胖方塊
        val bodyTop = h * 0.42f
        drawRoundRect(
            color = body,
            topLeft = Offset(w * 0.08f, bodyTop),
            size = Size(w * 0.84f, h * 0.56f),
            cornerRadius = CornerRadius(w * 0.2f),
        )

        // 愛心鑰匙孔
        val cx = w * 0.5f
        val cy = bodyTop + h * 0.27f
        val r = w * 0.17f
        val heart = Path().apply {
            moveTo(cx, cy + r * 0.95f)
            cubicTo(cx - r * 1.4f, cy + r * 0.1f, cx - r * 0.9f, cy - r * 1.1f, cx, cy - r * 0.35f)
            cubicTo(cx + r * 0.9f, cy - r * 1.1f, cx + r * 1.4f, cy + r * 0.1f, cx, cy + r * 0.95f)
            close()
        }
        drawPath(heart, ink)
    }
}
