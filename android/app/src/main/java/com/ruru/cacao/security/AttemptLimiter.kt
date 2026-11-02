package com.ruru.cacao.security

/** 輸錯 5 次等 30 秒；累計 10 次起每 5 次等 5 分鐘；輸對歸零（auth-design §1） */
data class AttemptState(val failures: Int = 0, val lockedUntilMillis: Long = 0)

object AttemptLimiter {
    private const val SHORT_WAIT_MILLIS = 30_000L
    private const val LONG_WAIT_MILLIS = 300_000L

    fun onFailure(state: AttemptState, nowMillis: Long): AttemptState {
        val failures = state.failures + 1
        val wait = when {
            failures % 5 != 0 -> 0L
            failures == 5 -> SHORT_WAIT_MILLIS
            else -> LONG_WAIT_MILLIS
        }
        return AttemptState(failures, if (wait > 0) nowMillis + wait else state.lockedUntilMillis)
    }

    fun remainingSeconds(state: AttemptState, nowMillis: Long): Int =
        if (state.lockedUntilMillis <= nowMillis) 0 else ((state.lockedUntilMillis - nowMillis + 999) / 1000).toInt()
}
