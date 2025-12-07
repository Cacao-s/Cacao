# Cacao App 開發指南

## 專案結構

Cacao 使用 **npm workspace** 管理 monorepo 結構：

```
Cacao/ (root)
├── package.json         # Root workspace scripts
├── package-lock.json    # 統一 lock file
├── node_modules/        # 共享依賴
├── apps/
│   ├── mobile/          # Expo React Native App
│   └── web-admin/       # React Web 管理後台
├── shared/
│   ├── api-schema/      # API 型別定義
│   ├── client-sdk/      # 共用 API 客戶端
│   └── ui-kit/          # 共用 UI 組件
└── server/              # Go backend API
```

## 套件管理規範

### 為什麼使用 npm workspace？

1. **統一依賴管理**：使用單一 lock file 管理所有依賴
2. **簡化安裝流程**：一次 `npm install` 安裝所有 workspace 依賴
3. **內建支援**：npm 7+ 內建 workspace 功能，無需額外工具
4. **CI 友善**：lock file 穩定，減少 diff 衝突

### 常用指令

### 快速開始

1. Clone 專案
2. 安裝 nvm (Node Version Manager)
   - Windows: <https://github.com/coreybutler/nvm-windows/releases>
3. 安裝 Node.js

   ```bash
   nvm install 24.11.0
   nvm use 24.11.0
   ```

4. 安裝專案依賴

   ```bash
   npm install
   ```

5. 安裝 Java JDK 17 (Android 開發必需)
   - 下載: <https://adoptium.net/temurin/releases/?version=17>
   - winget install Microsoft.OpenJDK.17
   - 選擇 JDK 17 LTS,作業系統選 Windows x64
   - 安裝時**勾選** "Set JAVA_HOME variable" 和 "Add to PATH"
   - 安裝後重啟 VS Code 使環境變數生效
   - 驗證安裝:

     ```bash
     java -version  # 應顯示 openjdk version "17.x.x"
     echo %JAVA_HOME%  # 應顯示 JDK 安裝路徑
     $env:JAVA_HOME # 應顯示 JDK 安裝路徑
     ```

6. 下載 Android Studio (包含 Android SDK + 模擬器)
   - 下載: <https://developer.android.com/studio>
   - 安裝 Android Studio
   - 開啟後安裝 Android SDK (跟隨安裝精靈)
   - 建議安裝 Android 13 (API 33) 或更新版本
7. Android SDK 並設定 ANDROID_HOME 環境變數
  - winget install Google.AndroidStudio
  - set env
  ```bash
  $androidSdkPath = "C:\Users\$env:USERNAME\AppData\Local\Android\Sdk"; [System.Environment]::SetEnvironmentVariable('ANDROID_HOME', $androidSdkPath, 'User'); $env:ANDROID_HOME = $androidSdkPath; Write-Host "ANDROID_HOME set to: $androidSdkPath"
  ```
8. 啟動 Android 模擬器
   - 開啟 Android Studio > Device Manager
   - 建立一個虛擬裝置 (推薦: Pixel 5 + Android 13)
   - 啟動模擬器
9. VS Code 執行除錯: **Expo: Android**
   - 按 F5 或點選左側除錯面板選擇 "Expo: Android"

#### 根目錄操作(推薦)

```bash
# 安裝所有 workspace 依賴
npm install

# 在特定 workspace 執行指令
npm start --workspace=apps/mobile              # 啟動 Expo
npm install expo-camera --workspace=apps/mobile # 在 mobile 安裝套件

# 在所有 workspace 執行指令
npm run typecheck --workspaces  # 全部 TypeScript 型別檢查
npm run lint --workspaces       # 全部 ESLint 檢查
npm run test --workspaces       # 執行所有測試

# Root scripts（已配置在 package.json）
npm run dev:mobile   # 啟動 mobile app
npm run typecheck    # 執行型別檢查
npm run lint         # 執行 lint

# Android Debug
# 啟動模擬器
npm start --workspace=apps/mobile
cd apps\mobile\android                                                        
.\gradlew.bat assembleDebug

# 查看詳細執行錯誤原因
cd apps\mobile\android
.\gradlew.bat app:assembleDebug --stacktrace
```

#### Workspace 內操作（不推薦）

```bash
# ❌ 避免在子目錄直接 npm install
cd apps/mobile
npm install  # 會破壞 workspace 結構!

# ✅ 應該在根目錄使用 --workspace
cd ../..  # 回到根目錄
npm install --workspace=apps/mobile
```

### Launch.json 設定優化

Debug 配置已優化為 4 個核心選項:

- **`Expo: Web`** - 啟動開發伺服器(自動清除快取)
  - 使用 `npm start --workspace=apps/mobile -- --clear`
  - 清除 Metro bundler 快取,確保乾淨啟動
- **`Expo: Web (No Cache Clear)`** - 快速啟動(不清快取)
  - 使用 `npm start --workspace=apps/mobile`
  - 適合程式碼小改動時使用
- **`Expo: Android`** - 啟動 Android 平台
  - 使用 `npm run android --workspace=apps/mobile`
- **`Expo: iOS`** - 啟動 iOS 平台
  - 使用 `npm run ios --workspace=apps/mobile`

### 安裝新套件規則

1. **在根目錄操作**（強制）:
   ```bash
   # ✅ 正確
   npm install @nozbe/watermelondb --workspace=apps/mobile
   
   # ❌ 錯誤
   cd apps/mobile
   npm install @nozbe/watermelondb  # 會產生獨立 lock file!
   ```

2. **devDependencies 安裝在哪？**
   - 專案專屬的放子 workspace（如 `eslint-config-expo` → mobile）
   - 通用工具放 root（如 `prettier`, `husky`）

3. **安裝開發依賴套件**：
   ```bash
   # 安裝 devDependencies
   npm install -D typescript --workspace=apps/mobile
   
   # 安裝一般依賴
   npm install react-native-svg --workspace=apps/mobile
   ```

## Expo 開發流程

### 啟動開發伺服器

```bash
# 根目錄執行（推薦）
npm run dev:mobile

# 或直接使用 workspace
npm start --workspace=apps/mobile

# 清除快取啟動
npm start --workspace=apps/mobile -- --clear
```

### 在實體裝置測試

1. 安裝 Expo Go app（iOS/Android）
2. 確保手機與電腦在同一 Wi-Fi
3. 掃描終端機顯示的 QR code

### 在模擬器測試

```bash
# iOS 模擬器（需要 Xcode）
npm run ios --workspace=apps/mobile

# Android 模擬器（需要 Android Studio）
npm run android --workspace=apps/mobile
```

### TypeScript 型別檢查

```bash
# 檢查 mobile app
npm run typecheck --workspace=apps/mobile

# 檢查所有 workspace
npm run typecheck --workspaces
```

### ESLint 檢查

```bash
# Lint mobile app
npm run lint --workspace=apps/mobile

# Lint 所有 workspace
npm run lint --workspaces
```

## 常見問題

### Q1: node_modules 要加入 .gitignore 嗎？

**A**: 是的，但只需在 `.gitignore` 加 root 的 `node_modules/`：

```gitignore
# Root
node_modules/
package-lock.json  # ❌ 錯誤！lock file 要提交
```

正確版本：
```gitignore
node_modules/
# package-lock.json 不應該被忽略!
```

### Q2: 為什麼不能在子目錄執行 npm install？

**A**: npm workspace 使用根目錄的 `package-lock.json` 管理所有依賴。在子目錄執行會產生獨立的 lock file，破壞 workspace 結構。

### Q3: 如何清理並重新安裝？

```bash
# 清理所有 node_modules
Remove-Item -Recurse -Force node_modules, apps\mobile\node_modules, apps\web-admin\node_modules

# 重新安裝（只在根目錄）
npm install
```

### Q4: Windows 長路徑問題怎麼辦？

如果遇到 "找不到路徑" 錯誤（通常是 React Native 深層目錄）：

```powershell
# 使用 robocopy 強制清理
$emptyDir = New-Item -Type Directory -Force ".\empty_temp"
robocopy $emptyDir node_modules /MIR /R:0 /W:0 | Out-Null
Remove-Item node_modules -Force
Remove-Item empty_temp -Force
```

### Q5: Expo 警告 peer dependency 不符怎麼辦？

```bash
# 範例：react-dom 要求 react@^19.2.1 但目前是 19.1.0
npm install react@^19.2.1 --workspace=apps/mobile

# 若 Expo SDK 54 不支援，可暫時忽略（Expo 會處理）
```

## 相關資源

- [npm workspace 文件](https://docs.npmjs.com/cli/v8/using-npm/workspaces)
- [Expo 官方文件](https://docs.expo.dev/)
- [Expo Router 文件](https://docs.expo.dev/router/introduction/)
- [專案 Commander 規則](../commander.md)
- [F 系列任務日誌](./agent-app-log.md)
