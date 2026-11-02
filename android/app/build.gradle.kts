plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.sqldelight)
}

val sharedDir = rootProject.projectDir.resolve("../shared")

// shared/db/001_*.sql 是初始 schema，之後的 NNN_*.sql 是 migration；轉成 SQLDelight 要的檔名
val syncSharedSchema by tasks.registering(Sync::class) {
    from(sharedDir.resolve("db")) {
        include("*.sql")
        rename { name ->
            val number = name.substringBefore("_").toInt()
            if (number == 1) "Schema.sq" else "${number - 1}.sqm"
        }
    }
    into(layout.buildDirectory.dir("generated/sharedSchema/com/ruru/cacao/db"))
}

android {
    namespace = "com.ruru.cacao"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.ruru.cacao"
        minSdk = 26
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    buildFeatures {
        compose = true
    }

    packaging {
        resources {
            excludes += "/META-INF/versions/9/OSGI-INF/MANIFEST.MF"
        }
    }

    testOptions {
        unitTests.all {
            it.systemProperty("cacao.sharedDir", sharedDir.absolutePath)
        }
    }
}

kotlin {
    jvmToolchain(17)
}

sqldelight {
    databases {
        create("CacaoDatabase") {
            packageName.set("com.ruru.cacao.db")
            srcDirs.setFrom(
                "src/main/sqldelight",
                layout.buildDirectory.dir("generated/sharedSchema"),
            )
        }
    }
}

tasks.matching { it.name.startsWith("generate") && it.name.contains("CacaoDatabase") }.configureEach {
    dependsOn(syncSharedSchema)
}
tasks.matching { it.name.startsWith("verify") && it.name.contains("CacaoDatabase") }.configureEach {
    dependsOn(syncSharedSchema)
}

dependencies {
    implementation(platform(libs.compose.bom))
    implementation(libs.compose.ui)
    implementation(libs.compose.material3)
    implementation(libs.compose.ui.tooling.preview)
    implementation(libs.activity.compose)
    implementation(libs.lifecycle.runtime)
    implementation(libs.lifecycle.viewmodel.compose)
    implementation(libs.sqldelight.android)
    implementation(libs.bouncycastle)
    debugImplementation(libs.compose.ui.tooling)

    testImplementation(libs.junit)
    testImplementation(libs.sqldelight.jvm)
    testImplementation(libs.serialization.json)
}
