package com.ruru.cacao

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.ui.ThemePreviewScreen

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val driver = AndroidSqliteDriver(CacaoDatabase.Schema, applicationContext, "cacao.db")
        CacaoDatabase(driver)
        setContent { ThemePreviewScreen() }
    }
}
