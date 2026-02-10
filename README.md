# Cacao

家庭零用錢管理 App。Giver（家長）和 Baby（孩子）各自在自己的裝置上安裝，透過 WiFi P2P 同步資料，不需要雲端伺服器。

## Tech Stack

| 層級 | 技術 |
|------|------|
| 框架 | [Tauri v2](https://v2.tauri.app/) — Rust 後端 + WebView 前端 |
| 前端 | React 19 + TypeScript 5.9 + Vite 7 |
| 樣式 | Tailwind CSS 4（CSS-first `@theme {}`）+ shadcn/ui |
| 狀態 | Zustand（client state）+ TanStack Query（server state）|
| 路由 | React Router v7（`createBrowserRouter`）|
| i18n | i18next（zh-TW / en）|
| 資料庫 | SQLite（WAL mode）via sqlx 0.8 |
| 同步 | mDNS 發現 + 嵌入式 Axum HTTP server（Giver 為 server）|
| 認證 | Argon2id PIN hashing |
| 測試 | Vitest 4（前端）+ cargo test（後端）|

## 先決條件

- Node.js 20+
- pnpm 9+
- Rust stable（edition 2024）
- Tauri CLI：`cargo install tauri-cli`
- （iOS）Xcode + CocoaPods
- （Android）Android Studio + NDK

## 快速開始

```bash
# 安裝依賴
make setup        # pnpm install + rustup targets

# 啟動開發環境（Tauri + Vite HMR）
make dev

# 僅啟動前端（不需 Rust 編譯，適合純 UI 開發）
make dev-web
```

## Make 指令

Build pipeline 有依賴鏈：`fmt → check → test → build`

```bash
make dev          # Tauri 開發模式（Rust + WebView）
make dev-web      # 僅前端開發（Vite dev server, port 1420）
make fmt          # 格式化（cargo fmt + prettier）
make check        # Lint（clippy -D warnings + eslint + tsc --noEmit）
make test         # 跑全部測試（fmt + check + cargo test + vitest）
make build        # 完整建置（fmt + check + test + cargo build + vite build）
make build-android  # Android APK
make build-ios      # iOS IPA
make clean        # 清除建置產物
make db-reset     # 刪除本地 SQLite 資料庫
```

## 資料夾結構

```
Cacao/
├── src/                          # 前端（React + TypeScript）
│   ├── main.tsx                  #   React 進入點
│   ├── App.tsx                   #   路由定義 + Provider 層
│   ├── app/                      #   頁面（依路由結構組織）
│   │   ├── dashboard.tsx         #     / — 首頁儀表板
│   │   ├── layout.tsx            #     App shell（header + tab bar）
│   │   ├── setup.tsx             #     /setup — 裝置初始設定
│   │   ├── pairing.tsx           #     /pairing — 家庭配對
│   │   ├── family.tsx            #     /family — 家庭成員管理
│   │   ├── lock-screen.tsx       #     PIN 鎖定畫面
│   │   ├── wallets/              #     /wallets, /wallets/:id
│   │   ├── requests/             #     /requests, /requests/new, /requests/:id
│   │   ├── allowances/           #     /allowances
│   │   ├── transactions/         #     /transactions
│   │   ├── notifications/        #     /notifications
│   │   └── settings/             #     /settings, /settings/audit-log
│   ├── components/               #   可重用元件
│   │   ├── ui/                   #     shadcn/ui 基礎元件（button, card, dialog...）
│   │   ├── setup-guard.tsx       #     路由守衛：未設定 → 導向 /setup
│   │   ├── lock-guard.tsx        #     App resume 時 PIN 鎖定
│   │   ├── sync-indicator.tsx    #     同步狀態指示器
│   │   ├── wallet-card.tsx       #     錢包卡片
│   │   ├── request-card.tsx      #     請款卡片
│   │   ├── allowance-card.tsx    #     津貼卡片
│   │   ├── transaction-item.tsx  #     交易列表項目
│   │   └── ...                   #     amount-display, monthly-summary, etc.
│   ├── hooks/                    #   TanStack Query hooks（呼叫 Tauri invoke）
│   │   ├── use-wallets.ts        #     錢包 CRUD
│   │   ├── use-requests.ts       #     請款流程
│   │   ├── use-allowances.ts     #     津貼管理
│   │   ├── use-transactions.ts   #     交易查詢 + 月報
│   │   ├── use-notifications.ts  #     通知 + 未讀數
│   │   ├── use-auth.ts           #     PIN 認證
│   │   ├── use-dashboard.ts      #     儀表板聚合資料
│   │   ├── use-audit.ts          #     審計日誌
│   │   └── use-export.ts         #     CSV 匯出
│   ├── stores/                   #   Zustand stores（純 client 狀態）
│   │   ├── profile-store.ts      #     使用者 profile + family
│   │   ├── ui-store.ts           #     locale + theme（操作 DOM class）
│   │   └── sync-store.ts         #     同步狀態（呼叫 Tauri invoke）
│   ├── lib/                      #   工具函式
│   │   ├── format.ts             #     金額格式化、日期格式化
│   │   ├── types.ts              #     TypeScript 型別定義
│   │   └── utils.ts              #     cn()（tailwind-merge + clsx）
│   ├── i18n/                     #   國際化
│   │   ├── index.ts              #     i18next 初始化
│   │   ├── zh-TW.json            #     繁體中文翻譯
│   │   └── en.json               #     英文翻譯
│   └── styles/
│       └── globals.css           #   Tailwind 4 @theme + dark/high-contrast 主題
│
├── src-tauri/                    # 後端（Rust）
│   ├── src/
│   │   ├── main.rs               #   桌面平台進入點
│   │   ├── lib.rs                #   Tauri Builder setup + command 註冊
│   │   ├── error.rs              #   AppError 定義（code + message）
│   │   ├── commands/             #   Tauri command handlers（thin layer）
│   │   │   ├── setup.rs          #     裝置設定 + 家庭 + 配對
│   │   │   ├── wallet.rs         #     錢包 CRUD
│   │   │   ├── request.rs        #     請款流程
│   │   │   ├── allowance.rs      #     津貼管理
│   │   │   ├── transaction.rs    #     交易查詢 + 手動交易
│   │   │   ├── notification.rs   #     通知
│   │   │   ├── auth.rs           #     PIN 認證
│   │   │   ├── sync.rs           #     同步控制
│   │   │   ├── dashboard.rs      #     儀表板聚合
│   │   │   ├── audit.rs          #     審計日誌
│   │   │   └── export.rs         #     CSV 匯出
│   │   ├── services/             #   業務邏輯層（核心）
│   │   │   ├── setup_service.rs  #     裝置初始設定 + 重置
│   │   │   ├── family_service.rs #     家庭 + 成員管理
│   │   │   ├── wallet_service.rs #     錢包 CRUD + 封存
│   │   │   ├── request_service.rs#     請款狀態機（draft→pending→approved/rejected）
│   │   │   ├── allowance_service.rs #  津貼排程 + execute + calculate_next_run
│   │   │   ├── transaction_service.rs # 交易記錄 + 月報統計
│   │   │   ├── notification_service.rs # 通知 CRUD + 未讀計數
│   │   │   ├── pairing_service.rs #    6 位配對碼生成 + 驗證
│   │   │   ├── auth_service.rs   #     PIN hash（Argon2id）+ 驗證
│   │   │   ├── audit_service.rs  #     審計日誌查詢
│   │   │   └── export_service.rs #     CSV 匯出 + 欄位跳脫
│   │   ├── models/               #   資料模型（sqlx::FromRow）
│   │   │   ├── params.rs         #     Command 參數 structs
│   │   │   ├── profile.rs        #     Profile
│   │   │   ├── family.rs         #     Family + FamilyMember
│   │   │   ├── wallet.rs         #     Wallet
│   │   │   ├── request.rs        #     Request
│   │   │   ├── allowance.rs      #     Allowance
│   │   │   ├── transaction.rs    #     Transaction
│   │   │   ├── notification.rs   #     Notification
│   │   │   ├── audit_log.rs      #     AuditLog
│   │   │   ├── paired_device.rs  #     PairedDevice
│   │   │   └── sync_state.rs     #     SyncState
│   │   ├── sync/                 #   WiFi P2P 同步引擎
│   │   │   ├── coordinator.rs    #     同步協調器（管理生命週期）
│   │   │   ├── discovery.rs      #     mDNS 服務發現（_cacao._tcp.local.）
│   │   │   ├── server.rs         #     嵌入式 Axum HTTP server（Giver 端）
│   │   │   ├── client.rs         #     HTTP client（Baby 端）
│   │   │   ├── merge.rs          #     衝突解決（sync_version 比對）
│   │   │   ├── apply.rs          #     資料套用
│   │   │   └── protocol.rs       #     同步協議資料結構
│   │   ├── scheduler/
│   │   │   └── allowance_runner.rs #   津貼自動發放排程（僅 Giver 端）
│   │   └── db/
│   │       └── pool.rs           #   SQLite pool 初始化（WAL + foreign_keys）
│   ├── migrations/
│   │   └── 001_init.sql          #   資料庫 schema（11 張表）
│   ├── tests/
│   │   └── integration_tests.rs  #   整合測試（178 tests）
│   ├── Cargo.toml
│   └── tauri.conf.json           #   Tauri 設定（window 390x844）
│
├── docs/                         # 規格文件
│   ├── spec.md                   #   原始規格
│   ├── spec.claude.md            #   Claude 產生的改進版規格
│   ├── spec.claude.improve.md    #   最終改進版規格
│   └── plan.md                   #   分階段執行計畫（Phase 0-10）
│
├── Makefile                      # 建置指令（fmt → check → test → build）
├── index.html                    # Vite HTML 進入點
├── vite.config.ts                # Vite 設定（@ alias, port 1420）
├── vitest.config.ts              # Vitest 設定
├── tsconfig.json                 # TypeScript 設定
├── eslint.config.js              # ESLint flat config
├── .prettierrc                   # Prettier 設定
└── components.json               # shadcn/ui 設定
```

## 程式碼架構

### 後端三層架構

```
前端 invoke() ──→ commands/ ──→ services/ ──→ SQLite (sqlx)
                 (thin layer)   (業務邏輯)
```

- **commands/** — Tauri command handlers，負責從 `AppHandle` 取得 `SqlitePool` state，呼叫 service，回傳結果。不含邏輯。
- **services/** — 核心業務邏輯。所有 DB 操作、驗證、狀態轉換都在這層。可以用 in-memory SQLite 獨立測試。
- **models/** — 資料結構（`sqlx::FromRow` + `serde::Serialize`），對應資料庫表。

### 前端資料流

```
Page ──→ Hook (TanStack Query) ──→ invoke() ──→ Rust command
  │
  └──→ Zustand Store（純 client 狀態：profile, theme, sync status）
```

- **hooks/** — 每個 hook 封裝一個 Tauri `invoke()` 呼叫，回傳 `useQuery` 或 `useMutation`。處理 cache key 和 invalidation。
- **stores/** — Zustand stores 用於不需要從後端取得的 client-only 狀態。
- **components/** — 純展示元件，接收 props 渲染 UI。不直接呼叫 invoke。

### 請款狀態機

```
draft ──→ pending ──→ approved（扣款 + 建立 transaction + notification）
  │          │
  │          └──→ rejected（需附原因 + notification）
  │          │
  └──→ cancelled ←─┘
```

### 同步機制

Giver 裝置啟動 Axum HTTP server，Baby 裝置透過 mDNS（`_cacao._tcp.local.`）發現後用 HTTP 同步。衝突解決用 `sync_version` 欄位：Giver 端版本 >= Baby 時拒絕 Baby 的變更。

### 資料庫

11 張表，所有表都有 `uuid`、`sync_version`、`last_synced_at`、`is_deleted` 欄位支援同步。金額統一用 `cents (INTEGER)`，貨幣預設 TWD。

## 測試

```bash
make test         # 跑全部測試（含 fmt + lint）
make test-rust    # 僅 Rust 測試
make test-web     # 僅前端測試
make test-watch   # Vitest watch mode
```

後端測試使用 in-memory SQLite，不需要外部 DB。前端測試使用 jsdom + `@testing-library/react`，mock Tauri `invoke`。

目前測試數量：178 backend + 201 frontend = **379 tests**。

## 開發注意事項

- Tailwind CSS 4 使用 CSS-first 設定，主題變數定義在 `src/styles/globals.css` 的 `@theme {}` 區塊，不需要 `tailwind.config.ts`
- `src/` 下的 `@` alias 對應 `src/`（在 `vite.config.ts` 和 `tsconfig.json` 中設定）
- Tauri 開發視窗預設 390x844（iPhone 尺寸），在 `tauri.conf.json` 中設定
- SQLite 資料庫檔案在 `src-tauri/target/debug/cacao.db`，可用 `make db-reset` 清除
- 津貼自動發放排程僅在 Giver 裝置的 App 前台時執行
