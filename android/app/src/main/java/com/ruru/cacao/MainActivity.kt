package com.ruru.cacao

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.ruru.cacao.db.CacaoDatabase

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val driver = AndroidSqliteDriver(CacaoDatabase.Schema, applicationContext, "cacao.db")
        val childCount = CacaoDatabase(driver).ledgerQueries.children().executeAsList().size
        setContent { StartScreen(childCount) }
    }
}

@Composable
private fun StartScreen(childCount: Int) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Color(0xFFFBF6F0))
            .padding(24.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp, Alignment.CenterVertically),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text("Cacao", fontSize = 40.sp, color = Color(0xFF6B4226))
        Text("資料庫已就緒・孩子 $childCount 位", fontSize = 16.sp, color = Color(0xFF6E5A4A))
    }
}
