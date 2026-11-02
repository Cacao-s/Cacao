package com.ruru.cacao.ui

import androidx.compose.runtime.Composable
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onRoot
import com.github.takahirom.roborazzi.captureRoboImage
import com.ruru.cacao.ui.components.Body
import com.ruru.cacao.ui.components.Celebration
import com.ruru.cacao.ui.components.CelebrationOverlay
import com.ruru.cacao.ui.components.CuteLock
import com.ruru.cacao.ui.components.PinPad
import com.ruru.cacao.ui.components.PrimaryButton
import com.ruru.cacao.ui.components.ScreenColumn
import com.ruru.cacao.ui.components.Title
import com.ruru.cacao.ui.theme.CacaoTheme
import com.ruru.cacao.ui.theme.ThemeTokens
import com.ruru.cacao.ui.theme.generated.GeneratedThemes
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.ParameterizedRobolectricTestRunner
import org.robolectric.annotation.Config
import org.robolectric.annotation.GraphicsMode

/**
 * 截圖測試（D-23）：共用元件 × 4 個主題，防止主題跑版。
 * 記錄基準：gradlew recordRoborazziDebug；比對：gradlew verifyRoborazziDebug
 */
@RunWith(ParameterizedRobolectricTestRunner::class)
@GraphicsMode(GraphicsMode.Mode.NATIVE)
@Config(sdk = [35], qualifiers = "zh-rTW-w393dp-h851dp-xxhdpi")
class ThemeSnapshotTest(private val tokens: ThemeTokens) {

    companion object {
        @JvmStatic
        @ParameterizedRobolectricTestRunner.Parameters(name = "{index}")
        fun themes(): List<Array<Any>> = GeneratedThemes.all.map { arrayOf(it) }
    }

    @get:Rule
    val compose = createComposeRule()

    private fun capture(name: String, content: @Composable () -> Unit) {
        compose.setContent { CacaoTheme(tokens) { content() } }
        compose.onRoot().captureRoboImage("src/test/snapshots/${name}_${tokens.id}.png")
    }

    @Test
    fun pinScreen() = capture("pin") {
        ScreenColumn {
            Title("媽媽，請輸入 PIN")
            CuteLock()
            PinPad(entered = "12", enabled = true, deleteLabel = "刪除", onChange = {}, onComplete = {})
        }
    }

    @Test
    fun balanceTexts() = capture("texts") {
        ScreenColumn {
            Title("存款 $3,450")
            Body("可出金額 $2,250・錢包 $120")
            PrimaryButton("記收入", "x") {}
        }
    }

    @Test
    fun celebration() = capture("celebrate") {
        CelebrationOverlay(Celebration.TaskDone("整理房間", "$50")) {}
    }
}
