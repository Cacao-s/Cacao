import java.util.Properties

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.sqldelight)
    alias(libs.plugins.roborazzi)
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
        versionCode = 3
        versionName = "0.1.2"
    }

    // 上傳金鑰放 repo 外，路徑與密碼寫在本機 keystore.properties（不進 git，見 docs/release-guide.md）
    val keystoreFile = rootProject.file("keystore.properties")
    signingConfigs {
        if (keystoreFile.exists()) {
            val props = Properties().apply { keystoreFile.inputStream().use { load(it) } }
            create("upload") {
                storeFile = file(props.getProperty("storeFile"))
                storePassword = props.getProperty("storePassword")
                keyAlias = props.getProperty("keyAlias")
                keyPassword = props.getProperty("keyPassword")
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            // 還沒有上傳金鑰時用 debug 金鑰簽，只供本機測 R8 版本，不能上架
            signingConfig = signingConfigs.findByName("upload") ?: signingConfigs.getByName("debug")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    buildFeatures {
        compose = true
        buildConfig = true
    }

    packaging {
        resources {
            excludes += "/META-INF/versions/9/OSGI-INF/MANIFEST.MF"
        }
    }

    // 動畫與音效共用 shared/（D-22）
    sourceSets["main"].assets.srcDirs(sharedDir.resolve("animations"), sharedDir.resolve("sounds"))

    testOptions {
        unitTests.isIncludeAndroidResources = true
        unitTests.all {
            it.systemProperty("cacao.sharedDir", sharedDir.absolutePath)
            it.maxHeapSize = "1g"
            it.maxParallelForks = 1
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
    implementation(libs.appcompat)
    implementation(libs.serialization.json)
    implementation(libs.play.services.auth)
    implementation(libs.lottie.compose)
    debugImplementation(libs.compose.ui.tooling)

    testImplementation(libs.junit)
    testImplementation(libs.sqldelight.jvm)
    testImplementation(libs.serialization.json)
    testImplementation(platform(libs.compose.bom))
    testImplementation(libs.compose.ui.test.junit4)
    testImplementation(libs.robolectric)
    testImplementation(libs.roborazzi)
    testImplementation(libs.roborazzi.compose)
    testImplementation(libs.roborazzi.rule)
    debugImplementation(libs.compose.ui.test.manifest)
}
