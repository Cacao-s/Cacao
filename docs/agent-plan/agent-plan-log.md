## Tasks

### P0001
完成帳號密碼登入功能

### P0002
1. 我需要安裝什麼來達到 Android 測試
2. 我需要做什麼步驟來達到 vs code 可以 Run and Debug
3. pnpm --filter mobile start    啟動
   遇到問題
   Opening on Android... Failed to resolve the Android SDK path. Default install location not
   我需要安裝什麼

P0002 解決方案總結
🎯 立即解決當前錯誤
錯誤訊息：Failed to resolve the Android SDK path

快速修復（PowerShell 管理員權限）：

📋 完整安裝檢查清單
需要安裝的項目：

✅ Android Studio（~1.1 GB）

下載：https://developer.android.com/studio
包含 Android SDK + 模擬器
✅ Android SDK 元件（透過 Android Studio SDK Manager）

Android 13.0 (API 33)
Android 14.0 (API 34)
Android 15.0 (API 35)
SDK Build-Tools
Platform-Tools（包含 adb）
Emulator
✅ Android Virtual Device（AVD）

建議：Pixel 8 with API 35
✅ VS Code 擴充功能

✅ 環境變數

ANDROID_HOME → SDK 路徑
PATH → 包含 platform-tools 和 emulator
🔧 VS Code Run and Debug 設定
建立 .vscode/launch.json：

✅ 驗證指令
🚀 使用方式
方法 1：VS Code 偵錯

按 F5
選擇 "Expo: Run on Android"
方法 2：命令列

完整的詳細步驟說明已經生成，包含所有安裝步驟、疑難排解和最佳實踐。需要我協助建立 launch.json 檔案嗎？

### P0003