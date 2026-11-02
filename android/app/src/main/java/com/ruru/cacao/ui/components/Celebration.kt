package com.ruru.cacao.ui.components

import android.content.Context
import android.media.AudioAttributes
import android.media.SoundPool
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.airbnb.lottie.compose.LottieAnimation
import com.airbnb.lottie.compose.LottieCompositionSpec
import com.airbnb.lottie.compose.rememberLottieComposition
import com.ruru.cacao.R
import com.ruru.cacao.ui.theme.Cacao

/** 要慶祝什麼：目標確認達成、任務驗收通過（features〈互動〉） */
sealed interface Celebration {
    data class GoalReached(val goalName: String) : Celebration
    data class TaskDone(val taskTitle: String, val reward: String) : Celebration
}

/** 短音效，音效開關在設定裡 */
class CelebrationSound(context: Context) {
    private val pool = SoundPool.Builder()
        .setMaxStreams(1)
        .setAudioAttributes(AudioAttributes.Builder().setUsage(AudioAttributes.USAGE_GAME).setContentType(AudioAttributes.CONTENT_TYPE_SONIFICATION).build())
        .build()
    private val soundId = context.assets.openFd("celebrate.wav").use { pool.load(it, 1) }

    fun play() {
        pool.play(soundId, 0.8f, 0.8f, 1, 0, 1f)
    }

    fun release() = pool.release()
}

/** S04 慶祝：Lottie 動畫＋一句話＋「太棒了」 */
@Composable
fun CelebrationOverlay(celebration: Celebration, onDismiss: () -> Unit) {
    val composition = rememberLottieComposition(LottieCompositionSpec.Asset("celebrate.json"))
    val (title, body) = when (celebration) {
        is Celebration.GoalReached -> stringResource(R.string.celebrate_goal_title, celebration.goalName) to stringResource(R.string.celebrate_goal_body)
        is Celebration.TaskDone -> stringResource(R.string.celebrate_task_title, celebration.taskTitle) to stringResource(R.string.celebrate_task_body, celebration.reward)
    }
    Box(
        Modifier
            .fillMaxSize()
            .background(Color.Black.copy(alpha = 0.45f))
            .clickable(interactionSource = remember { MutableInteractionSource() }, indication = null) {},
        contentAlignment = Alignment.Center,
    ) {
        Column(
            Modifier
                .padding(24.dp)
                .fillMaxWidth()
                .background(Cacao.colors.background, RoundedCornerShape(Cacao.shape.radius * 1.2f))
                .padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            LottieAnimation(composition.value, modifier = Modifier.size(180.dp))
            Title(title)
            Text(body, color = Cacao.colors.muted, fontSize = 16.sp, textAlign = TextAlign.Center)
            PrimaryButton(stringResource(R.string.celebrate_ok), "celebrate_ok", onClick = onDismiss)
        }
    }
}
