# Google OAuth 登入設定指南

## 📋 概要

本文件說明如何為 Cacao App 設定 Google OAuth 登入功能。

## ✅ 已完成項目

### 1. 套件安裝
- ✅ 已安裝 `@react-native-google-signin/google-signin`
- ✅ 已在 `app.json` 新增 plugin 配置

### 2. Android 憑證生成
- ✅ 已生成 `release.keystore` (位置: `apps/mobile/android/app/release.keystore`)
- ✅ 密碼: `cacao2025`
- ✅ 別名: `cacao-release`
- ✅ 套件名稱: `com.cacao.app`

**SHA-1 指紋 (dev、release 共用):**
```
0C:3F:3A:72:15:15:8B:EB:E7:43:BF:A9:CF:A7:CB:D9:4A:33:76:7B:50:38:58:2F:2B:1E:93:0E:C0:AC:1B:E3
```

**重要:** 請將 `release.keystore` 加入 `.gitignore`，不要提交到版本控制！

### 3. 程式碼實作
- ✅ 已在 `authService.ts` 新增:
  - `configureGoogleSignIn()` - 初始化配置
  - `loginWithGoogle()` - Google 登入邏輯
  - `signOutGoogle()` - Google 登出
- ✅ 已在 `login.tsx` 新增 Google 登入按鈕
- ✅ 已更新 `AuthContext.tsx` 支援 Google OAuth

## 🔧 待完成設定步驟

### Step 1: 建立 Google Cloud 專案

1. 前往 [Google Cloud Console](https://console.cloud.google.com/)
2. 建立新專案或選擇現有專案
3. 專案名稱建議: `Cacao App`

### Step 2: 啟用 Google Sign-In API

1. 在 Google Cloud Console 中，前往 **APIs & Services > Library**
2. 搜尋 "Google Sign-In"
3. 點擊啟用

### Step 3: 設定 OAuth 同意畫面

1. 前往 **APIs & Services > OAuth consent screen**
2. 選擇 **External** (外部使用者)
3. 填寫必填資訊:
   - App name: `Cacao`
   - User support email: (你的 email)
   - Developer contact information: (你的 email)
4. 儲存並繼續

### Step 4: 建立 OAuth 2.0 憑證

#### 4.1 Web Client ID (必須)

1. 前往 **APIs & Services > Credentials**
2. 點擊 **Create Credentials > OAuth client ID**
3. Application type: **Web application**
4. Name: `Cacao Web Client`
5. 點擊 **Create**
6. **複製 Client ID** (格式: `xxxxx.apps.googleusercontent.com`)
7. 將此 Client ID 貼到 `apps/mobile/.env` 檔案中

#### 4.2 Android Client

1. 再次點擊 **Create Credentials > OAuth client ID**
2. Application type: **Android**
3. Name: `Cacao Android`
4. Package name: `com.cacao.app`
5. SHA-1 certificate fingerprint (dev、release 共用):
   - `0C:3F:3A:72:15:15:8B:EB:E7:43:BF:A9:CF:A7:CB:D9:4A:33:76:7B:50:38:58:2F:2B:1E:93:0E:C0:AC:1B:E3`
6. 點擊 **Create**

   **重要**: 只需要建立**一個** Android Client，dev、release 共用同一個 SHA-1 指紋

#### 4.3 iOS Client (未來需要)

1. 點擊 **Create Credentials > OAuth client ID**
2. Application type: **iOS**
3. Name: `Cacao iOS`
4. Bundle ID: `com.cacao.app`
5. 點擊 **Create**

### Step 5: 配置環境變數

1. 複製 `.env.example` 為 `.env`:
   ```bash
   cd apps/mobile
   cp .env.example .env
   ```

2. 編輯 `.env` 檔案，填入 Web Client ID:
   ```env
   EXPO_PUBLIC_GOOGLE_WEB_CLIENT_ID=你的-web-client-id.apps.googleusercontent.com
   ```

3. 確保 `.env` 已在 `.gitignore` 中

### Step 6: 重新建置 Android App

```bash
# 清理並重新建置
cd apps/mobile/android
./gradlew clean
./gradlew assembleDebug

# 或使用 npm script
cd apps/mobile
npm run android
```

### Step 7: 測試 Google 登入

1. 啟動 Expo dev server:
   ```bash
   npm run start --workspace=apps/mobile
   ```

2. 按 `a` 開啟 Android 模擬器或連接實體裝置

3. 在登入畫面點擊 "使用 Google 登入" 按鈕

4. 完成 Google OAuth 流程

## 📱 iOS 設定 (未來需要時)

iOS 需要額外配置:

1. 在 `app.json` 新增 `ios.googleServicesFile`:
   ```json
   "ios": {
     "bundleIdentifier": "com.cacao.app",
     "googleServicesFile": "./GoogleService-Info.plist"
   }
   ```

2. 下載 `GoogleService-Info.plist` 從 Firebase Console

3. 放置於 `apps/mobile/` 根目錄

## 🔍 測試檢查清單

- [ ] Google Cloud Console 專案已建立
- [ ] OAuth 同意畫面已設定
- [ ] Web Client ID 已建立
- [ ] Android Client 已建立 (SHA-1 指紋正確)
- [ ] `.env` 檔案已建立並填入 Web Client ID
- [ ] Android App 已重新建置
- [ ] Google 登入按鈕出現在登入畫面
- [ ] 點擊按鈕後可開啟 Google 登入流程
- [ ] 登入成功後可導向首頁
- [ ] 使用者資料正確儲存到本地資料庫

## 🐛 常見問題

### Q1: 點擊 Google 登入後沒有反應
**A:** 檢查:
1. `.env` 檔案是否存在且包含正確的 Web Client ID
2. App 是否已重新建置 (變更 .env 後需重新建置)
3. 查看 console log 是否有錯誤訊息

### Q2: 出現 "DEVELOPER_ERROR"
**A:** 表示 SHA-1 指紋不匹配或 package name 不正確:
1. 確認 Google Cloud Console 中的 Android Client 設定
2. 確認 SHA-1 指紋是否正確
3. 確認 package name 是 `com.cacao.app`

### Q3: 出現 "SIGN_IN_REQUIRED"
**A:** 表示需要使用者互動:
1. 確保在實體裝置或模擬器上測試 (不要在 Web 上)
2. 確保裝置已登入 Google 帳號

### Q4: iOS 如何生成 SHA-1?
**A:** iOS 不需要 SHA-1，改用 Bundle ID:
1. 確認 `app.json` 中的 `ios.bundleIdentifier` 是 `com.cacao.app`
2. 在 Google Cloud Console 建立 iOS Client 時使用此 Bundle ID

## 📚 相關文件

- [Google Sign-In for React Native](https://github.com/react-native-google-signin/google-signin)
- [Google Cloud Console](https://console.cloud.google.com/)
- [Expo Config Plugins](https://docs.expo.dev/config-plugins/introduction/)

## 📝 安全注意事項

1. **絕對不要** 將以下檔案提交到 Git:
   - `.env`
   - `release.keystore`
   - `GoogleService-Info.plist`

2. 在 `.gitignore` 中確保包含:
   ```
   # Environment variables
   .env
   .env.local
   
   # Android keystore
   *.keystore
   *.jks
   
   # iOS
   GoogleService-Info.plist
   ```

3. 團隊成員需要自行從 Google Cloud Console 下載憑證

---

**建立日期:** 2025-12-05  
**維護者:** Cacao Development Team
