# Google OAuth 快速參考卡

## 🔑 關鍵資訊

### Android 配置
```
Package Name: com.cacao.app

Release Keystore (dev、release 共用):
  Path: apps/mobile/android/app/release.keystore
  Password: cacao2025
  Alias: cacao
  SHA-1: DA:8B:A6:4F:2C:9B:18:17:2A:CF:AF:C6:BE:51:B2:8A:C2:61:12:C8
  SHA-256: 15:F4:1C:2F:F6:BA:54:B1:36:0E:23:06:F9:89:EF:78:6B:FC:12:A9:CE:A2:34:BB:02:9F:1D:21:08:E6:01:98
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
   - ✅ Android Client (貼上 SHA-1，dev、release 共用)
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
