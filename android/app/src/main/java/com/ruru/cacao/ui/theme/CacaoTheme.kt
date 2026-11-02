package com.ruru.cacao.ui.theme

import androidx.annotation.FontRes
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.Immutable
import androidx.compose.runtime.remember
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.ExperimentalTextApi
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontVariation
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.Dp
import com.ruru.cacao.ui.theme.generated.GeneratedThemes

@Immutable
data class ThemeColors(
    val background: Color,
    val surface: Color,
    val onSurface: Color,
    val muted: Color,
    val outline: Color,
    val primary: Color,
    val onPrimary: Color,
    val secondary: Color,
    val onSecondary: Color,
    val locked: Color,
    val onLocked: Color,
    val danger: Color,
)

@Immutable
data class ThemeShape(val radius: Dp, val border: Dp, val hardShadow: Dp)

@Immutable
data class ThemeTokens(
    val id: String,
    val nameZh: String,
    val nameEn: String,
    val dark: Boolean,
    val colors: ThemeColors,
    @FontRes val numberFont: Int,
    val titleSerif: Boolean,
    val shape: ThemeShape,
)

@Immutable
data class ThemeFonts(val number: FontFamily, val title: FontFamily)

private val LocalTokens = staticCompositionLocalOf { GeneratedThemes.cocoaMilk }
private val LocalFonts = staticCompositionLocalOf { ThemeFonts(FontFamily.Default, FontFamily.Default) }

object Cacao {
    val tokens: ThemeTokens @Composable get() = LocalTokens.current
    val colors: ThemeColors @Composable get() = LocalTokens.current.colors
    val shape: ThemeShape @Composable get() = LocalTokens.current.shape
    val fonts: ThemeFonts @Composable get() = LocalFonts.current
}

@OptIn(ExperimentalTextApi::class)
private fun variableFont(@FontRes res: Int): FontFamily = FontFamily(
    listOf(400, 500, 700, 800).map { weight ->
        Font(res, FontWeight(weight), variationSettings = FontVariation.Settings(FontVariation.weight(weight)))
    },
)

@Composable
fun CacaoTheme(tokens: ThemeTokens = GeneratedThemes.cocoaMilk, content: @Composable () -> Unit) {
    val fonts = remember(tokens.id) {
        ThemeFonts(
            number = variableFont(tokens.numberFont),
            title = if (tokens.titleSerif) FontFamily.Serif else FontFamily.Default,
        )
    }
    val c = tokens.colors
    val scheme = if (tokens.dark) {
        darkColorScheme(
            primary = c.primary, onPrimary = c.onPrimary, secondary = c.secondary, onSecondary = c.onSecondary,
            background = c.background, onBackground = c.onSurface, surface = c.surface, onSurface = c.onSurface,
            outline = c.outline, error = c.danger,
        )
    } else {
        lightColorScheme(
            primary = c.primary, onPrimary = c.onPrimary, secondary = c.secondary, onSecondary = c.onSecondary,
            background = c.background, onBackground = c.onSurface, surface = c.surface, onSurface = c.onSurface,
            outline = c.outline, error = c.danger,
        )
    }
    CompositionLocalProvider(LocalTokens provides tokens, LocalFonts provides fonts) {
        MaterialTheme(colorScheme = scheme, content = content)
    }
}
