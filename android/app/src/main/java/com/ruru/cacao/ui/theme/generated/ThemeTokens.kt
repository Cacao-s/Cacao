// 自動產生，請勿手改。來源：shared/tokens/themes.json；產生：python tools/codegen/tokens.py
package com.ruru.cacao.ui.theme.generated

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.ruru.cacao.R
import com.ruru.cacao.ui.theme.ThemeColors
import com.ruru.cacao.ui.theme.ThemeShape
import com.ruru.cacao.ui.theme.ThemeTokens

object GeneratedThemes {
    val cocoaMilk = ThemeTokens(
        id = "cocoa_milk",
        nameZh = "可可牛奶",
        nameEn = "Cocoa Milk",
        dark = false,
        colors = ThemeColors(
            background = Color(0xFFFBF6EF),
            surface = Color(0xFFFFFFFF),
            onSurface = Color(0xFF2A1A12),
            muted = Color(0xFF6E5A4C),
            outline = Color(0xFFD9C8B6),
            primary = Color(0xFF6B3B22),
            onPrimary = Color(0xFFFFFFFF),
            secondary = Color(0xFF2F7D6D),
            onSecondary = Color(0xFFFFFFFF),
            locked = Color(0xFFE9B44C),
            onLocked = Color(0xFF2A1A12),
            danger = Color(0xFFB3261E),
        ),
        numberFont = R.font.fraunces,
        titleSerif = true,
        shape = ThemeShape(radius = 22.dp, border = 0.0.dp, hardShadow = 0.dp),
    )

    val colorBlocks = ThemeTokens(
        id = "color_blocks",
        nameZh = "彩色積木",
        nameEn = "Color Blocks",
        dark = false,
        colors = ThemeColors(
            background = Color(0xFFFFFBF0),
            surface = Color(0xFFFFFFFF),
            onSurface = Color(0xFF161616),
            muted = Color(0xFF4A4A4A),
            outline = Color(0xFF161616),
            primary = Color(0xFF2D5BFF),
            onPrimary = Color(0xFFFFFFFF),
            secondary = Color(0xFF3DDC97),
            onSecondary = Color(0xFF161616),
            locked = Color(0xFFFFD23F),
            onLocked = Color(0xFF161616),
            danger = Color(0xFFD7263D),
        ),
        numberFont = R.font.rubik,
        titleSerif = false,
        shape = ThemeShape(radius = 16.dp, border = 2.5.dp, hardShadow = 4.dp),
    )

    val marshmallow = ThemeTokens(
        id = "marshmallow",
        nameZh = "棉花糖",
        nameEn = "Marshmallow",
        dark = false,
        colors = ThemeColors(
            background = Color(0xFFF6F1FF),
            surface = Color(0xFFFFFFFF),
            onSurface = Color(0xFF2B2350),
            muted = Color(0xFF6C6590),
            outline = Color(0xFFDCD2FF),
            primary = Color(0xFF6A51F0),
            onPrimary = Color(0xFFFFFFFF),
            secondary = Color(0xFFBDEFD8),
            onSecondary = Color(0xFF134A35),
            locked = Color(0xFFFFE59A),
            onLocked = Color(0xFF5E4700),
            danger = Color(0xFFC2410C),
        ),
        numberFont = R.font.nunito,
        titleSerif = false,
        shape = ThemeShape(radius = 28.dp, border = 0.0.dp, hardShadow = 0.dp),
    )

    val starryGold = ThemeTokens(
        id = "starry_gold",
        nameZh = "星夜金幣",
        nameEn = "Starry Gold",
        dark = true,
        colors = ThemeColors(
            background = Color(0xFF12142B),
            surface = Color(0xFF1D2042),
            onSurface = Color(0xFFF4F1E8),
            muted = Color(0xFFA9AACB),
            outline = Color(0xFF34386A),
            primary = Color(0xFFF5C451),
            onPrimary = Color(0xFF2A1E00),
            secondary = Color(0xFF5FE3B5),
            onSecondary = Color(0xFF0A2A20),
            locked = Color(0xFFF5C451),
            onLocked = Color(0xFF2A1E00),
            danger = Color(0xFFFF8A80),
        ),
        numberFont = R.font.sora,
        titleSerif = false,
        shape = ThemeShape(radius = 20.dp, border = 0.0.dp, hardShadow = 0.dp),
    )

    val all = listOf(cocoaMilk, colorBlocks, marshmallow, starryGold)
}
