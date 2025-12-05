# Google OAuth 快速參考卡

## 🔑 關鍵資訊

### Android 配置
```
Package Name: com.cacao.app

Debug Keystore (開發用):
  Path: apps/mobile/android/app/debug.keystore
  Password: android
  Alias: androiddebugkey
  SHA-1: FA:C6:17:45:DC:09:03:78:6F:B9:ED:E6:2A:96:2B:39:9F:73:48:F0:BB:6F:89:9B:83:32:66:75:91:03:3B:9C

Release Keystore (發布用):
  Path: apps/mobile/android/app/release.keystore
  Password: cacao2025
  Alias: cacao-release
  SHA-1: 0C:3F:3A:72:15:15:8B:EB:E7:43:BF:A9:CF:A7:CB:D9:4A:33:76:7B:50:38:58:2F:2B:1E:93:0E:C0:AC:1B:E3
```

### iOS 配置
```
Bundle ID: com.cacao.app
```

## 📝 快速設定步驟

1. **Google Cloud Console** → 建立專案
2. **啟用 Google Sign-In API**
3. **建立 OAuth Clients**:
   - ✅ Web Client (複製 Client ID)
   - ✅ Android Client - Debug (貼上 Debug SHA-1)
   - ✅ Android Client - Release (貼上 Release SHA-1)
   - ⏳ iOS Client (貼上 Bundle ID)
4. **建立 .env 檔案**:
   ```bash
   cd apps/mobile
   cp .env.example .env
   # 編輯 .env，填入 Web Client ID
   ```
5. **重建 App**:
   ```bash
   npm run android --workspace=apps/mobile
   ```

## 🧪 測試步驟

1. 啟動 Expo dev server: `npm run start --workspace=apps/mobile`
2. 按 `a` 開啟 Android
3. 點擊 "使用 Google 登入"
4. 完成 OAuth 流程
5. 確認導向首頁

## 🐛 常見錯誤

| 錯誤碼 | 原因 | 解決方法 |
|--------|------|----------|
| `DEVELOPER_ERROR` | SHA-1 不匹配 | 檢查 Google Cloud Console 設定 |
| `SIGN_IN_CANCELLED` | 使用者取消 | 正常行為 |
| `PLAY_SERVICES_NOT_AVAILABLE` | 缺少 Google Play | 使用有 Google Play 的裝置 |

## 📚 相關文件

- 完整設定: `GOOGLE_OAUTH_SETUP.md`
- 開發日誌: `docs/agent-app-log.md`
- Package: [@react-native-google-signin/google-signin](https://github.com/react-native-google-signin/google-signin)

---
**重要提醒**: 不要將 `.env`, `*.keystore`, `GoogleService-Info.plist` 提交到 Git!
