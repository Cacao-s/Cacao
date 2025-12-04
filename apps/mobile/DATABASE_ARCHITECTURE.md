# Cacao Mobile App - Database & Auth Architecture

## 📦 完整資料模型 (10 Models)

基於 WatermelonDB + SQLite 的本地優先架構

### 核心 Models

1. **User** - 使用者帳號
   - 支援密碼登入 (bcrypt 加密)
   - 支援 Google OAuth (預留 `google_sub` 欄位)
   - 角色: giver (給予者), baby (寶貝), admin

2. **Family** - 家庭群組
   - 多幣別支援 (currency)
   - 時區設定 (timezone)

3. **FamilyMember** - 家庭成員關聯
   - 家庭角色: giver, baby, viewer
   - 邀請機制 (invited_by)
   - 成員狀態: active, pending, removed

### 財務 Models

4. **Wallet** - 錢包
   - 類型: cash, bank, card, virtual
   - 餘額以分為單位 (balance_cents)
   - 預警門檻 (warning_threshold_cents)

5. **Allowance** - 零用錢排程
   - 頻率: daily, weekly, biweekly, monthly, custom
   - 自動執行時間 (next_run_at)

6. **Request** - 請款申請
   - 狀態: draft, pending, approved, rejected, cancelled
   - 附件上傳 (attachment_url)
   - 決策記錄 (decision_by, rejection_reason)

7. **Transaction** - 交易記錄
   - 類型: credit (收入), debit (支出)
   - 來源: allowance, request, manual, adjustment

### 系統 Models

8. **Notification** - 通知
   - 狀態: pending, sent, failed, read
   - JSON payload

9. **SyncQueue** - 離線同步佇列
   - 操作類型: create, update, delete
   - 重試機制 (retries, last_error)
   - 狀態: pending, synced, failed

10. **AuditLog** - 審計日誌
    - 操作記錄 (action, resource_type, resource_id)
    - JSON metadata

## 🔐 本地認證服務

`src/services/authService.ts`

### 功能清單

✅ **註冊** - `registerUser()`
- Email 重複檢查
- 密碼強度驗證 (≥6 字元)
- bcrypt 加密 (10 salt rounds)

✅ **登入** - `loginUser()`
- Email + 密碼驗證
- bcrypt 密碼比對
- 帳號狀態檢查

✅ **個人資料** - `updateUserProfile()`
- 更新顯示名稱、語言、主題

✅ **變更密碼** - `changePassword()`
- 舊密碼驗證
- 新密碼強度檢查

### 安全性

- ✅ 密碼使用 bcrypt hash (10 rounds)
- ✅ 不儲存明文密碼
- ✅ Email 唯一性驗證
- ✅ 使用者狀態控制 (active/disabled)

## 🧪 測試工具

### 1. 資料庫測試 (`src/utils/testDatabase.ts`)

```typescript
testDatabaseConnection()  // 測試基本 CRUD
testAllModels()          // 測試所有 models 查詢
cleanupTestData()        // 清理測試資料
```

### 2. 測試資料初始化 (`src/utils/seedDatabase.ts`)

```typescript
seedDatabase()   // 建立測試資料
clearDatabase()  // 清空所有資料
```

**測試帳號**:
```
giver@example.com  / password123
baby@example.com   / password123
parent@example.com / password123
```

**測試資料包含**:
- 3 個使用者 (giver, baby, parent)
- 1 個家庭 (Demo Family)
- 3 個家庭成員
- 2 個錢包 (現金 $500, 銀行 $10,000)

## 📱 測試介面

`app/index.tsx` 提供完整測試 UI:

### 資料庫測試
- 測試資料庫連接
- 測試所有 Models

### 測試資料管理
- 建立測試資料
- 清空資料庫

### 認證功能測試
- 測試註冊
- 測試登入 (Giver)

## 🚀 快速開始

### 1. 啟動開發伺服器

```bash
npm start --workspace=apps/mobile
```

### 2. 在 Android 模擬器測試

按 `a` 鍵或執行:
```bash
npm run android --workspace=apps/mobile
```

### 3. 初始化測試資料

在 App 中點擊「建立測試資料」按鈕

### 4. 測試登入功能

點擊「測試登入 (Giver)」按鈕驗證認證流程

## 📂 專案結構

```
apps/mobile/
├── src/
│   ├── models/          # 10 個 WatermelonDB models
│   ├── database/        # Schema, adapter, provider
│   ├── services/        # authService.ts
│   └── utils/           # testDatabase, seedDatabase
├── app/
│   ├── _layout.tsx      # Root layout with DatabaseProvider
│   └── index.tsx        # 測試介面
├── babel.config.js      # Decorators support
└── tsconfig.json        # experimentalDecorators: true
```

## ⚙️ 技術規格

- **Database**: WatermelonDB 0.28.0 + SQLite
- **Authentication**: bcryptjs ^2.4.3
- **TypeScript**: 5.9.3 (strict mode + experimentalDecorators)
- **Expo**: SDK 54.0.25
- **React**: 19.1.0

## 🎯 下一步

1. ✅ 資料庫層完成
2. ✅ 認證服務完成
3. ⏳ 實作登入/註冊 UI
4. ⏳ 建立 Auth Context
5. ⏳ 實作受保護路由
6. ⏳ 開發主要功能 (錢包、請款、交易)

## 💡 注意事項

### Windows 開發
- ✅ 使用 npm workspace (非 pnpm)
- ✅ 避免路徑長度問題

### 本地優先策略
- ✅ 所有功能先基於本地 DB
- ✅ SyncQueue 為未來 API 同步預留
- ✅ 無需後端即可完整測試

### 密碼安全
- ✅ bcrypt 10 rounds
- ✅ 預設密碼長度 ≥6
- ✅ 不儲存明文

## 📝 開發日誌

詳見 [docs/agent-app/agent-app-log.md](../../docs/agent-app/agent-app-log.md)
