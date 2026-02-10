# Cacao 分階段執行計畫

> **專案**：Cacao — 家庭零用金管理 App（Tauri v2 Mobile + React + Rust）
> **基於**：`docs/spec.claude.improve.md` v2（無雲端、P2P WiFi 同步、裝置角色）
> **日期**：2026-02-10

---

## 全域依賴圖

```
Phase 0 ─ 專案骨架搭建
  │
  └─▶ Phase 1 ─ 資料庫 + 核心模型
        │
        ├─▶ Phase 2 ─ P0: 裝置設定 + 家庭配對
        │     │
        │     ├─▶ Phase 3 ─ P0: 錢包管理
        │     │     │
        │     │     └─▶ Phase 4 ─ P0: 請款流程
        │     │           │
        │     │           ├─▶ Phase 5 ─ P0: WiFi P2P 同步引擎
        │     │           │
        │     │           └─▶ Phase 6 ─ P1: 津貼 + 交易 + 通知
        │     │                 │
        │     │                 └─▶ Phase 7 ─ P1: Dashboard 整合
        │     │                       │
        │     │                       └─▶ Phase 8 ─ P2: 生物辨識 + 審計 + CSV
        │     │                             │
        │     │                             └─▶ Phase 9 ─ P3: i18n + 主題 + 無障礙
        │     │                                   │
        │     │                                   └─▶ Phase 10 ─ CI/CD + Release
        │     │
        │     └─▶ Phase 5（可與 Phase 3/4 並行原型驗證）
        │
        └─▶ Phase 7（需 Phase 3, 4, 5, 6 全部完成）
```

> **提示**：Phase 5（同步引擎）是最大技術風險點。建議在 Phase 2 完成後即啟動原型驗證，與 Phase 3/4 並行開發。

---

## Phase 總覽

| Phase | 目標 | 優先級 | 關鍵交付物 |
|-------|------|--------|-----------|
| 0 | 專案骨架搭建 | — | Tauri v2 + React + Vite + Tailwind 4 + shadcn/ui + Rust 模組骨架 |
| 1 | 資料庫 + 核心模型 | — | SQLite pool + 11 張表 migration + 所有 Rust model structs |
| 2 | 裝置設定 + 家庭配對 | P0 | setup/family services + commands + Setup/Pairing 頁面 + SetupGuard |
| 3 | 錢包管理 | P0 | wallet service + commands + Wallet 列表/詳情頁 + 金額格式化 |
| 4 | 請款流程 | P0 | request service（狀態機 + 扣款）+ Request 列表/建立/詳情頁 |
| 5 | WiFi P2P 同步引擎 | P0 | mDNS + Axum server + client + merge engine + 同步 UI |
| 6 | 津貼 + 交易 + 通知 | P1 | allowance runner + transaction service + notification + 頁面 |
| 7 | Dashboard 整合 | P1 | Giver/Baby Dashboard + DashboardData 聚合 command |
| 8 | 生物辨識 + 審計 + CSV | P2 | PIN/biometric + audit log UI + CSV 匯出 |
| 9 | i18n + 主題 + 無障礙 | P3 | zh-TW/en 翻譯 + high contrast + WCAG AA |
| 10 | CI/CD + Release | — | GitHub Actions + E2E 檢查清單 + APK/IPA 建置 |

---

# Phase 0 — 專案骨架搭建

**目標**：從零建立 Tauri v2 Mobile + React + Vite + Tailwind CSS 4 + Rust 專案結構，所有工具鏈可正常編譯與執行。

**前置條件**：無

---

### 0.1 初始化 Tauri v2 專案

```bash
pnpm create tauri-app cacao --template react-ts --manager pnpm
cd cacao
cargo tauri android init
cargo tauri ios init
```

**產出**：專案根目錄結構 + `src-tauri/gen/android/` + `src-tauri/gen/apple/`

---

### 0.2 配置 `src-tauri/Cargo.toml`

新增所有 Rust 依賴：

```toml
[dependencies]
tauri = { version = "2", features = ["mobile"] }
tauri-plugin-store = "2"
tauri-plugin-biometric = "2"
tauri-specta = { version = "2", features = ["derive"] }
specta = { version = "2", features = ["derive"] }
specta-typescript = "0.0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
argon2 = "0.5"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
axum = "0.8"
mdns-sd = "0.11"
rand = "0.8"
thiserror = "2"
```

---

### 0.3 配置 `src-tauri/tauri.conf.json`

- `identifier`: `"com.cacao.app"`
- `app.name`: `"Cacao"`
- `plugins`: 啟用 `store`, `biometric`
- `security.capabilities`: 設定 IPC 存取權限

相關檔案：`src-tauri/tauri.conf.json`, `src-tauri/capabilities/default.json`

---

### 0.4 安裝前端依賴

```bash
# 運行時依賴
pnpm add react@19 react-dom@19 react-router@7 \
  @tanstack/react-query@5 zustand \
  i18next react-i18next zod lucide-react date-fns \
  @tauri-apps/api@2 @tauri-apps/plugin-store@2 @tauri-apps/plugin-biometric@2

# 開發依賴
pnpm add -D typescript @types/react @types/react-dom \
  vite @vitejs/plugin-react \
  tailwindcss@4 @tailwindcss/vite \
  vitest @testing-library/react @testing-library/jest-dom jsdom
```

---

### 0.5 配置 `vite.config.ts`

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: { "@": "/src" },
  },
  clearScreen: false,
  server: {
    strictPort: true,
  },
});
```

---

### 0.6 配置 Tailwind CSS 4（CSS-first）

**`src/styles/globals.css`**：

```css
@import "tailwindcss";

@theme {
  --color-primary: oklch(0.65 0.15 55);
  --color-primary-foreground: oklch(0.98 0 0);
  --color-secondary: oklch(0.75 0.08 80);
  --color-secondary-foreground: oklch(0.25 0.02 80);
  --color-destructive: oklch(0.55 0.2 25);
  --color-muted: oklch(0.92 0.01 80);
  --color-muted-foreground: oklch(0.55 0.02 80);
  --color-accent: oklch(0.88 0.04 80);
  --color-border: oklch(0.85 0.02 80);
  --color-ring: oklch(0.65 0.15 55);
  --radius-lg: 0.75rem;
  --radius-md: 0.5rem;
  --radius-sm: 0.25rem;
}
```

> **注意**：無 `tailwind.config.ts`、無 `autoprefixer`、無 `postcss.config.js`。Tailwind CSS 4 完全使用 CSS-first 設定。

---

### 0.7 初始化 shadcn/ui

```bash
pnpm dlx shadcn@latest init
pnpm dlx shadcn@latest add button card input label dialog select tabs badge toast alert-dialog separator
```

**產出**：`src/components/ui/*.tsx`、`components.json`

---

### 0.8 建立 Rust 目錄骨架

建立所有空的 `mod.rs` 與基礎結構：

```
src-tauri/src/
├── lib.rs              # Tauri Builder 入口
├── error.rs            # AppError struct
├── specta.rs           # tauri-specta 初始化
├── commands/
│   └── mod.rs
├── db/
│   ├── mod.rs
│   └── pool.rs
├── models/
│   └── mod.rs
├── services/
│   └── mod.rs
├── sync/
│   └── mod.rs
└── scheduler/
    └── mod.rs
```

**`src-tauri/src/error.rs`**：

```rust
use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type, Clone)]
pub struct AppError {
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into() }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}
```

---

### 0.9 初始化 tauri-specta

**`src-tauri/src/specta.rs`**：

- 建立 `specta::Builder`，設定 TypeScript binding 輸出至 `../src/lib/bindings.ts`
- 後續每個 Phase 新增的 command 都在此註冊

---

### 0.10 Tauri 入口 `src-tauri/src/lib.rs`

```rust
mod commands;
mod db;
mod error;
mod models;
mod scheduler;
mod services;
mod specta;
mod sync;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta::create_specta_builder();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            // DB pool 會在 Phase 1 加入
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### 0.11 前端路由骨架

**`src/App.tsx`**：

```tsx
import { RouterProvider, createBrowserRouter } from "react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient();

const router = createBrowserRouter([
  { path: "/setup", element: <div>Setup</div> },
  { path: "/pairing", element: <div>Pairing</div> },
  { path: "/", element: <div>Dashboard</div> },
]);

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
}
```

---

### 驗證步驟

- [ ] `cargo build`（在 `src-tauri/` 下）— Rust 編譯通過
- [ ] `pnpm dev` — Vite 開發伺服器啟動
- [ ] `cargo tauri dev` — 桌面預覽可顯示 "Dashboard"
- [ ] `cargo tauri android dev` — Android 模擬器可啟動（需先設定 Android SDK）
- [ ] `src/lib/bindings.ts` 已產生（內容可為空）
- [ ] `src/components/ui/button.tsx` 存在（shadcn/ui 安裝成功）
- [ ] Tailwind CSS 4 樣式正確套用（測試 `className="bg-primary text-primary-foreground"`）

---

# Phase 1 — 資料庫與核心資料模型

**目標**：完成 SQLite 初始化、所有 migration、所有 Rust model structs，讓後續 Phase 可直接使用資料層。

**前置條件**：Phase 0

---

### 1.1 SQLite 連線池

**`src-tauri/src/db/pool.rs`**：

```rust
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub async fn init_pool(app_data_dir: &Path) -> Result<SqlitePool, sqlx::Error> {
    let db_path = app_data_dir.join("cacao.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::query("PRAGMA journal_mode = WAL;").execute(&pool).await?;
    sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
```

整合至 `src-tauri/src/lib.rs` 的 `setup`：

```rust
.setup(move |app| {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    let pool = tauri::async_runtime::block_on(db::pool::init_pool(&app_data_dir))?;
    app.manage(pool);
    Ok(())
})
```

---

### 1.2 Migration 檔案

**`src-tauri/migrations/001_init.sql`**：

包含所有 11 張表的 DDL（直接取自 spec Section 5）：

| # | 表名 | 同步欄位 | 備註 |
|---|------|---------|------|
| 1 | `profiles` | uuid, sync_version, last_synced_at, is_deleted | 裝置 profile，無 email/password |
| 2 | `families` | uuid, sync_version, last_synced_at, is_deleted | `created_by_device TEXT` |
| 3 | `family_members` | uuid, sync_version, last_synced_at, is_deleted | UNIQUE(family_id, profile_uuid) |
| 4 | `paired_devices` | — | 配對裝置記錄 |
| 5 | `wallets` | uuid, sync_version, last_synced_at, is_deleted | UNIQUE(family_id, name) |
| 6 | `allowances` | uuid, sync_version, last_synced_at, is_deleted | frequency CHECK 約束 |
| 7 | `requests` | uuid, sync_version, last_synced_at, is_deleted | status CHECK 約束 + idx_requests_family_status |
| 8 | `transactions` | uuid, sync_version, last_synced_at, is_deleted | append-only + idx_transactions_wallet_date + idx_transactions_family_date |
| 9 | `notifications` | — | 純本地，無同步欄位 |
| 10 | `audit_logs` | — | 純本地 |
| 11 | `sync_state` | — | key-value store |

所有 `CREATE TABLE` 及 `CREATE INDEX` SQL 直接照搬 spec Section 5.2–5.12。

---

### 1.3 Rust Models

每個 model 使用 `#[derive(Debug, Serialize, Deserialize, FromRow, Type, Clone)]`。

| 檔案 | Struct(s) | Enum(s) |
|------|-----------|---------|
| `src-tauri/src/models/profile.rs` | `Profile` | `Role { Giver, Baby }` |
| `src-tauri/src/models/family.rs` | `Family`, `FamilyMember` | `FamilyRole { Giver, Baby, Viewer }`, `MemberStatus { Active, Removed }` |
| `src-tauri/src/models/paired_device.rs` | `PairedDevice` | — |
| `src-tauri/src/models/wallet.rs` | `Wallet` | `WalletType { Cash, Bank, Card, Virtual }`, `WalletStatus { Active, Archived }` |
| `src-tauri/src/models/allowance.rs` | `Allowance` | `Frequency { Daily, Weekly, Biweekly, Monthly, Custom }`, `AllowanceStatus { Active, Paused, Archived }` |
| `src-tauri/src/models/request.rs` | `Request` | `RequestStatus { Draft, Pending, Approved, Rejected, Cancelled }`, `RequestCategory { Food, Transport, Education, Entertainment, Clothing, Health, Other }` |
| `src-tauri/src/models/transaction.rs` | `Transaction` | `TransactionType { Credit, Debit }`, `SourceType { Allowance, Request, Manual, Adjustment }` |
| `src-tauri/src/models/notification.rs` | `Notification` | `EventType { RequestSubmitted, RequestApproved, RequestRejected, AllowanceDisbursed, LowBalance, MemberJoined }` |
| `src-tauri/src/models/audit_log.rs` | `AuditLog` | — |
| `src-tauri/src/models/sync_state.rs` | `SyncState` | — |
| `src-tauri/src/models/mod.rs` | — | re-export all |

---

### 1.4 共用參數型別

**`src-tauri/src/models/params.rs`**：

```rust
#[derive(Debug, Deserialize, Type)]
pub struct CreateWalletParams {
    pub family_id: i64,
    pub name: String,
    pub wallet_type: String,
    pub initial_balance_cents: Option<i64>,
}

#[derive(Debug, Deserialize, Type)]
pub struct CreateRequestParams {
    pub family_id: i64,
    pub requester_member_id: i64,
    pub wallet_id: i64,
    pub amount_cents: i64,
    pub category: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Type)]
pub struct CreateAllowanceParams {
    pub family_id: i64,
    pub giver_member_id: i64,
    pub receiver_member_id: i64,
    pub wallet_id: i64,
    pub amount_cents: i64,
    pub frequency: String,
    pub interval_count: Option<i64>,
    pub notes: Option<String>,
}
```

---

### 驗證步驟

- [ ] `cargo build` — 所有 model 編譯通過
- [ ] `cargo test` — migration 測試通過（in-memory SQLite 執行 migration 確認 schema 正確）
- [ ] 所有 11 張表的 DDL 在 migration 中完整無遺漏
- [ ] `bindings.ts` 產生了所有 TypeScript 型別（`Profile`, `Wallet`, `Request` 等）
- [ ] 每個 enum 的 `#[serde(rename_all = "snake_case")]` 確保 JSON 序列化與 DB CHECK 約束一致

---

# Phase 2 — P0: 裝置設定與家庭配對

**目標**：實作首次使用流程 — 設定裝置、建立家庭（Giver）、配對加入家庭（Baby）。

**前置條件**：Phase 0, Phase 1

---

### 2.1 Backend: Setup Service

**`src-tauri/src/services/setup_service.rs`**：

| 方法 | 說明 |
|------|------|
| `setup_device(pool, display_name, role) -> Result<Profile>` | 建立 profile，產生 UUID，寫入 `profiles` 表 |
| `get_profile(pool) -> Result<Option<Profile>>` | 取得裝置 profile（profiles 表只有一筆） |
| `is_setup(pool) -> Result<bool>` | 是否已設定 |
| `reset_device(pool) -> Result<()>` | 清除所有資料（重設裝置） |

---

### 2.2 Backend: Family Service

**`src-tauri/src/services/family_service.rs`**：

| 方法 | 說明 |
|------|------|
| `create_family(pool, device_uuid, name) -> Result<Family>` | 建立家庭 + 自動建立 `family_members` (giver) |
| `get_family(pool) -> Result<Option<Family>>` | 取得家庭 |
| `get_members(pool, family_id) -> Result<Vec<FamilyMember>>` | 列出成員 |
| `add_member(pool, family_id, profile_uuid, role) -> Result<FamilyMember>` | 新增成員 |
| `remove_member(pool, member_id) -> Result<()>` | 移除成員（soft delete） |

---

### 2.3 Backend: Pairing Service

**`src-tauri/src/services/pairing_service.rs`**：

| 方法 | 說明 |
|------|------|
| `generate_pairing_code(pool, family_id) -> Result<String>` | 產生 6 位數碼，存入 `sync_state`（key: `pairing_code` + `pairing_expires_at`） |
| `validate_pairing_code(pool, code) -> Result<PairingInfo>` | 驗證碼是否有效且未過期 |
| `complete_pairing(pool, baby_device_uuid, baby_profile, family_id) -> Result<()>` | 寫入 `paired_devices` + `family_members` |

配對碼規則：
- 6 位數字（`rand::Rng::gen_range(100000..999999)`）
- 有效期 5 分鐘
- 一次性使用（驗證成功後清除）

---

### 2.4 Backend: Setup Commands

**`src-tauri/src/commands/setup.rs`**：

```rust
#[tauri::command]
#[specta::specta]
pub async fn setup_device(display_name: String, role: String, pool: State<'_, SqlitePool>) -> Result<Profile, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn get_profile(pool: State<'_, SqlitePool>) -> Result<Option<Profile>, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn is_setup(pool: State<'_, SqlitePool>) -> Result<bool, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn create_family(name: String, pool: State<'_, SqlitePool>) -> Result<Family, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn get_family(pool: State<'_, SqlitePool>) -> Result<Option<Family>, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn get_family_members(family_id: i64, pool: State<'_, SqlitePool>) -> Result<Vec<FamilyMember>, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn generate_pairing_code(family_id: i64, pool: State<'_, SqlitePool>) -> Result<String, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn join_family_with_code(code: String, pool: State<'_, SqlitePool>) -> Result<Family, AppError>;

#[tauri::command]
#[specta::specta]
pub async fn remove_family_member(member_id: i64, pool: State<'_, SqlitePool>) -> Result<(), AppError>;

#[tauri::command]
#[specta::specta]
pub async fn reset_device(pool: State<'_, SqlitePool>) -> Result<(), AppError>;
```

在 `specta.rs` 中註冊以上所有 commands。

---

### 2.5 Frontend: Zustand Store

**`src/stores/profile-store.ts`**：

```typescript
interface ProfileState {
  profile: Profile | null;
  family: Family | null;
  isSetup: boolean;
  setProfile: (p: Profile) => void;
  setFamily: (f: Family) => void;
  clear: () => void;
}
```

---

### 2.6 Frontend: SetupGuard

**`src/components/setup-guard.tsx`**：

```tsx
function SetupGuard() {
  const { data: isSetup, isLoading } = useQuery({
    queryKey: ["is-setup"],
    queryFn: () => commands.isSetup(),
  });
  if (isLoading) return <LoadingScreen />;
  if (!isSetup) return <Navigate to="/setup" />;
  return <Outlet />;
}
```

---

### 2.7 Frontend: Setup Page

**`src/app/setup.tsx`**：

- Step 1：顯示名稱輸入（zod 驗證 1~50 字）
- Step 2：角色選擇 — 兩張大卡片（Giver / Baby），lucide-react 圖示
- Step 3：
  - Giver → 輸入家庭名稱 → `createFamily()` → 產生配對碼 → 顯示或跳過
  - Baby → 導向 `/pairing`
- 進度指示器（Giver: 1/4 ~ 4/4，Baby: 1/3 ~ 3/3）

元件：`Card`, `Input`, `Label`, `Button`

---

### 2.8 Frontend: Pairing Page

**`src/app/pairing.tsx`**：

**Giver 模式**（產生配對碼）：
- 顯示 6 位數配對碼（`font-mono text-4xl tracking-widest`）
- 倒計時 5 分鐘（過期自動產生新碼）
- 狀態指示：等待中 / 連接中 / 配對成功
- 呼叫 `generatePairingCode(familyId)`

**Baby 模式**（輸入配對碼）：
- 6 個獨立數字輸入框（auto-focus 下一格）
- 「加入家庭」按鈕 → `joinFamilyWithCode(code)`
- 錯誤提示：`PAIRING_CODE_INVALID` / `PAIRING_CODE_EXPIRED` / `PEER_NOT_FOUND`
- 配對成功 → 同步家庭資料 → 導向 `/`

---

### 2.9 Frontend: AppLayout（Tab Bar）

**`src/app/layout.tsx`**：

底部 5 個 Tab：

| Tab | 圖示 | 路徑 |
|-----|------|------|
| 首頁 | `Home` | `/` |
| 錢包 | `Wallet` | `/wallets` |
| 請款 | `Receipt` | `/requests` |
| 通知 | `Bell` | `/notifications` |
| 設定 | `Settings` | `/settings` |

- lucide-react 圖示
- 通知 Tab 顯示未讀數量 `Badge`
- `<Outlet />` 渲染子路由

---

### 2.10 Frontend: 更新路由

**`src/App.tsx`** 更新為完整路由結構：

```tsx
const router = createBrowserRouter([
  { path: "/setup", element: <SetupPage /> },
  { path: "/pairing", element: <PairingPage /> },
  {
    element: <SetupGuard />,
    children: [{
      element: <AppLayout />,
      children: [
        { index: true, element: <DashboardPage /> },
        { path: "wallets", element: <WalletsPage /> },
        { path: "wallets/:id", element: <WalletDetailPage /> },
        { path: "allowances", element: <AllowancesPage /> },
        { path: "requests", element: <RequestsPage /> },
        { path: "requests/new", element: <NewRequestPage /> },
        { path: "requests/:id", element: <RequestDetailPage /> },
        { path: "transactions", element: <TransactionsPage /> },
        { path: "notifications", element: <NotificationsPage /> },
        { path: "settings", element: <SettingsPage /> },
        { path: "family", element: <FamilyPage /> },
      ],
    }],
  },
]);
```

此階段大部分頁面先用 placeholder `<div>Coming Soon</div>`，後續 Phase 逐步替換。

---

### 2.11 Backend 測試

**`src-tauri/tests/setup_test.rs`**：

- `setup_device` → profile 寫入成功 + UUID 自動產生
- 重複 `setup_device` → `DEVICE_ALREADY_SETUP` error
- `create_family` → family + family_member 同時建立
- `generate_pairing_code` → 6 位數 + 5 分鐘有效
- 過期碼 → `PAIRING_CODE_EXPIRED`
- 錯誤碼 → `PAIRING_CODE_INVALID`
- `complete_pairing` → `paired_devices` + `family_members` 寫入

---

### 驗證步驟

- [ ] `cargo test` — setup/family/pairing service 測試通過
- [ ] 桌面預覽：首次啟動導向 `/setup`，完成設定後導向 `/`
- [ ] Giver 流程：名稱 → Giver → 建立家庭 → 顯示配對碼
- [ ] Baby 流程：名稱 → Baby → 輸入碼 → 加入家庭（本地邏輯）
- [ ] 重設裝置 → 回到 `/setup`
- [ ] `bindings.ts` 含 `setupDevice`, `getProfile`, `createFamily`, `generatePairingCode`, `joinFamilyWithCode`
- [ ] Tab Bar 正確顯示，所有路由可導航

---

# Phase 3 — P0: 錢包管理

**目標**：Giver 可建立/管理錢包，雙方可查看錢包列表與餘額。

**前置條件**：Phase 2（需要家庭已建立）

---

### 3.1 Backend: Wallet Service

**`src-tauri/src/services/wallet_service.rs`**：

| 方法 | 說明 |
|------|------|
| `create_wallet(pool, params) -> Result<Wallet>` | 驗證同名不存在 + 初始餘額建立 manual credit transaction |
| `list_wallets(pool, family_id) -> Result<Vec<Wallet>>` | 僅列 `is_deleted = 0` |
| `get_wallet(pool, id) -> Result<Wallet>` | 單筆查詢 |
| `update_wallet(pool, id, name, warning_threshold) -> Result<Wallet>` | 更新名稱/閾值 |
| `archive_wallet(pool, id) -> Result<()>` | status → `archived` + 遞增 `sync_version` |

初始餘額處理：

```rust
// 在 SQLite transaction 中
if let Some(initial) = params.initial_balance_cents {
    if initial > 0 {
        // 1. INSERT INTO transactions (type='credit', source_type='manual', amount_cents=initial)
        // 2. UPDATE wallets SET balance_cents = initial
    }
}
```

---

### 3.2 Backend: Wallet Commands

**`src-tauri/src/commands/wallet.rs`**：

```rust
pub async fn create_wallet(params: CreateWalletParams, pool: State<'_, SqlitePool>) -> Result<Wallet, AppError>;
pub async fn list_wallets(family_id: i64, pool: State<'_, SqlitePool>) -> Result<Vec<Wallet>, AppError>;
pub async fn get_wallet(id: i64, pool: State<'_, SqlitePool>) -> Result<Wallet, AppError>;
pub async fn update_wallet(id: i64, name: String, warning_threshold_cents: i64, pool: State<'_, SqlitePool>) -> Result<Wallet, AppError>;
pub async fn archive_wallet(id: i64, pool: State<'_, SqlitePool>) -> Result<(), AppError>;
```

---

### 3.3 Frontend: Utility Functions

**`src/lib/format.ts`**：

```typescript
export function formatAmount(cents: number, currency = "TWD"): string {
  return new Intl.NumberFormat("zh-TW", {
    style: "currency",
    currency,
    minimumFractionDigits: 0,
  }).format(cents / 100);
}

export function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString("zh-TW");
}

export function formatDateTime(iso: string): string {
  return new Date(iso).toLocaleString("zh-TW");
}
```

---

### 3.4 Frontend: TanStack Query Hooks

**`src/hooks/use-wallets.ts`**：

```typescript
export function useWallets(familyId: number) {
  return useQuery({
    queryKey: ["wallets", familyId],
    queryFn: () => commands.listWallets(familyId),
  });
}

export function useWallet(id: number) {
  return useQuery({
    queryKey: ["wallets", id],
    queryFn: () => commands.getWallet(id),
  });
}

export function useCreateWallet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: commands.createWallet,
    onSuccess: () => qc.invalidateQueries({ queryKey: ["wallets"] }),
  });
}

export function useArchiveWallet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => commands.archiveWallet(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["wallets"] }),
  });
}
```

---

### 3.5 Frontend: Components

| 檔案 | 說明 |
|------|------|
| `src/components/wallet-card.tsx` | 錢包卡片：名稱、類型圖示、餘額、低餘額警告 |
| `src/components/amount-display.tsx` | cents → `NT$ X,XXX` 格式化顯示元件 |
| `src/components/status-badge.tsx` | 通用狀態標籤（active/archived/pending 等） |
| `src/components/empty-state.tsx` | 通用 Empty 狀態（圖示 + 文字 + CTA） |
| `src/components/loading-skeleton.tsx` | 卡片骨架屏 |
| `src/components/error-state.tsx` | 錯誤狀態 + 重試按鈕 |

---

### 3.6 Frontend: Wallet Pages

**`src/app/wallets/index.tsx`**：

- 錢包卡片網格列表
- Giver：右下 FAB 按鈕 → Dialog 建立錢包（名稱、類型、初始餘額）
- zod 表單驗證
- Loading / Error / Empty 三態

**`src/app/wallets/[id].tsx`**：

- 錢包詳情（名稱、類型、餘額大字、低餘額警告閾值）
- 交易紀錄列表（Phase 4/6 完善後才有資料）
- Giver：封存按鈕 + `AlertDialog` 確認

---

### 3.7 Backend 測試

**`src-tauri/tests/wallet_test.rs`**：

- 建立錢包（含初始餘額）→ wallet + transaction 正確
- 同名重複 → `WALLET_NAME_EXISTS`
- 封存 → status == archived + sync_version 遞增
- 列表僅回傳 `is_deleted = 0`
- 初始餘額 = 0 → 不建立 transaction

---

### 驗證步驟

- [ ] `cargo test` — wallet service 測試通過
- [ ] 桌面預覽：`/wallets` 頁建立錢包 → 卡片出現
- [ ] 金額顯示格式正確（`NT$1,000`）
- [ ] 封存 → 確認對話框 → 消失
- [ ] Empty 狀態（無錢包時）正確顯示
- [ ] TypeScript 型別安全：`bindings.ts` 中有 `Wallet` 型別

---

# Phase 4 — P0: 請款流程

**目標**：Baby 建立/送出請款，Giver 核准/駁回，自動扣款與記帳。

**前置條件**：Phase 3（需要錢包存在才能發起請款）

---

### 4.1 Backend: Request Service

**`src-tauri/src/services/request_service.rs`**：

此為業務邏輯最複雜的 service。

| 方法 | 說明 |
|------|------|
| `create_request(pool, params) -> Result<Request>` | 建立草稿（status = draft） |
| `update_request(pool, id, params) -> Result<Request>` | 編輯草稿（僅 draft 可編輯） |
| `submit_request(pool, id) -> Result<Request>` | draft → pending，遞增 sync_version |
| `approve_request(pool, id, approver_id) -> Result<Request>` | **核心**，見下方 |
| `reject_request(pool, id, reason, rejector_id) -> Result<Request>` | pending → rejected |
| `cancel_request(pool, id) -> Result<Request>` | draft/pending → cancelled |
| `list_requests(pool, family_id, status?) -> Result<Vec<Request>>` | 列表（可按狀態篩選） |
| `get_request(pool, id) -> Result<Request>` | 單筆查詢 |

**`approve_request` 核心流程**（必須在 SQLite Transaction 中）：

```
1. 驗證 request.status == "pending"
2. 查詢 wallet → 檢查 balance_cents >= request.amount_cents
3. 餘額不足 → return Err(INSUFFICIENT_BALANCE)
4. UPDATE wallets SET balance_cents = balance_cents - amount_cents
5. INSERT INTO transactions (type='debit', source_type='request', source_id=request.id)
6. UPDATE requests SET status='approved', decision_by_member_id, decision_at
7. 遞增 request.sync_version + wallet.sync_version
8. INSERT INTO audit_logs
9. INSERT INTO notifications (event_type='request_approved')
10. COMMIT
```

**`reject_request`**：

- 驗證 `rejection_reason` 非空（`REJECTION_REASON_REQUIRED`）
- pending → rejected，記錄 `decision_by_member_id` + `decision_at`
- 不動錢包餘額

---

### 4.2 Backend: Transaction Service（基礎）

**`src-tauri/src/services/transaction_service.rs`**：

Phase 4 先建立基礎方法，Phase 6 擴充完整功能。

| 方法 | 說明 |
|------|------|
| `create_transaction(pool, params) -> Result<Transaction>` | 建立交易（內部使用） |
| `list_wallet_transactions(pool, wallet_id, limit) -> Result<Vec<Transaction>>` | 錢包交易紀錄 |

---

### 4.3 Backend: Request Commands

**`src-tauri/src/commands/request.rs`**：

```rust
pub async fn create_request(params: CreateRequestParams, pool: ...) -> Result<Request, AppError>;
pub async fn update_request(id: i64, params: UpdateRequestParams, pool: ...) -> Result<Request, AppError>;
pub async fn submit_request(id: i64, pool: ...) -> Result<Request, AppError>;
pub async fn approve_request(id: i64, approver_member_id: i64, pool: ...) -> Result<Request, AppError>;
pub async fn reject_request(id: i64, reason: String, rejector_member_id: i64, pool: ...) -> Result<Request, AppError>;
pub async fn cancel_request(id: i64, pool: ...) -> Result<Request, AppError>;
pub async fn list_requests(family_id: i64, status: Option<String>, pool: ...) -> Result<Vec<Request>, AppError>;
pub async fn get_request(id: i64, pool: ...) -> Result<Request, AppError>;
```

---

### 4.4 Frontend: TanStack Query Hooks

**`src/hooks/use-requests.ts`**：

```typescript
export function useRequests(familyId: number, status?: string);
export function useRequest(id: number);
export function useCreateRequest();    // invalidate ['requests'] + ['wallets']
export function useSubmitRequest();     // invalidate ['requests']
export function useApproveRequest();    // invalidate ['requests'] + ['wallets']
export function useRejectRequest();     // invalidate ['requests']
export function useCancelRequest();     // invalidate ['requests']
```

所有 mutation 成功後 invalidate `['requests']`；涉及金額的額外 invalidate `['wallets']`。

---

### 4.5 Frontend: Components

| 檔案 | 說明 |
|------|------|
| `src/components/request-card.tsx` | 請款卡片：金額、分類 icon、狀態 Badge、請款者、時間 |
| `src/components/request-status-badge.tsx` | 請款專用狀態 Badge（色彩分明） |

---

### 4.6 Frontend: Request Pages

**`src/app/requests/index.tsx`**：

- Tabs：全部 / 待審核 / 已核准 / 已駁回（shadcn/ui `Tabs`）
- 請款卡片列表（時間倒序）
- Baby：FAB「建立請款」按鈕
- Giver：待審核 tab 有 Badge 數量

**`src/app/requests/new.tsx`**（Baby 專屬）：

表單欄位：
- 錢包選擇（`Select`）
- 金額輸入（正整數，以元為單位，前端 × 100 轉 cents）
- 分類選擇（7 種：食物/交通/教育/娛樂/服飾/醫療/其他）
- 備註（≤ 500 字）
- 兩個按鈕：「儲存草稿」+「送出請款」

zod schema 驗證。

**`src/app/requests/[id].tsx`**：

- 請款詳情：金額、分類、備註、狀態、時間軸
- **Giver**（pending）：「核准」+「駁回」按鈕
  - 駁回 → 展開原因輸入（必填）→ `AlertDialog` 確認
  - 核准 → `AlertDialog` 確認
- **Baby**（draft）：編輯 + 送出 + 取消
- **Baby**（pending）：僅能取消
- 已決定的請款：只讀顯示結果

---

### 4.7 Frontend: 錢包詳情頁更新

更新 `src/app/wallets/[id].tsx`：現在可以顯示交易紀錄列表（來自請款核准 + 手動記帳）。

---

### 4.8 Backend 測試（重點）

**`src-tauri/tests/request_test.rs`**：

- [ ] 建立草稿 → status == draft
- [ ] 編輯草稿 → 成功；編輯 pending → `REQUEST_INVALID_STATUS`
- [ ] draft → pending（submit）→ sync_version 遞增
- [ ] pending → approved → 餘額扣除 + transaction 建立 + notification
- [ ] pending → approved（餘額不足）→ `INSUFFICIENT_BALANCE`
- [ ] pending → rejected（無原因）→ `REJECTION_REASON_REQUIRED`
- [ ] pending → rejected（有原因）→ 成功 + notification
- [ ] approved → approve 再次 → `REQUEST_ALREADY_DECIDED`
- [ ] draft/pending → cancelled → 成功
- [ ] approved → cancelled → `REQUEST_INVALID_STATUS`
- [ ] sync_version 在每次狀態轉換時正確遞增

---

### 驗證步驟

- [ ] `cargo test` — 所有請款狀態機測試通過
- [ ] 桌面預覽（Baby）：建立請款 → 草稿 → 送出 → pending
- [ ] 桌面預覽（Giver）：pending 請款 → 核准 → 錢包餘額減少
- [ ] 駁回流程：不填原因 → 錯誤；填原因 → 成功
- [ ] 取消流程：draft/pending 可取消
- [ ] 錢包詳情頁交易紀錄正確顯示
- [ ] `transactions` 表記錄正確（type, source_type, source_id, amount_cents）

---

# Phase 5 — P0: WiFi P2P 同步引擎

**目標**：實作同 WiFi 區域網路下 Giver–Baby 雙向同步。**整個專案中最複雜的模組。**

**前置條件**：Phase 2（配對）, Phase 3（錢包）, Phase 4（請款）

> **建議**：Phase 2 完成後即可啟動 Phase 5 的 mDNS 原型驗證，與 Phase 3/4 並行。

---

### 5.1 mDNS Discovery

**`src-tauri/src/sync/discovery.rs`**：

**Giver 端 — 廣播**：

```rust
pub struct MdnsBroadcaster { /* daemon handle */ }

impl MdnsBroadcaster {
    /// Register "_cacao._tcp.local." service
    /// TXT records: { "family_uuid": "...", "device_name": "..." }
    pub fn start(family_uuid: &str, device_name: &str, port: u16) -> Result<Self>;
    pub fn stop(&self);
}
```

**Baby 端 — 搜尋**：

```rust
pub struct PeerInfo {
    pub ip: String,
    pub port: u16,
    pub device_name: String,
    pub family_uuid: String,
}

pub struct MdnsDiscovery { /* daemon handle */ }

impl MdnsDiscovery {
    /// Browse "_cacao._tcp.local.", filter by family_uuid
    pub fn search(family_uuid: &str, timeout_secs: u64) -> Result<Vec<PeerInfo>>;
    pub fn stop(&self);
}
```

---

### 5.2 Sync Protocol 資料結構

**`src-tauri/src/sync/protocol.rs`**：

```rust
#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SyncHandshakeRequest {
    pub device_uuid: String,
    pub family_uuid: String,
    pub pairing_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SyncHandshakeResponse {
    pub success: bool,
    pub family_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SyncPushRequest {
    pub device_uuid: String,
    pub changes: Vec<SyncChange>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SyncPushResponse {
    pub accepted: Vec<String>,  // uuid list
    pub rejected: Vec<String>,  // uuid list
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SyncChange {
    pub table: String,       // "wallets", "requests", etc.
    pub uuid: String,        // record uuid
    pub sync_version: i64,
    pub data: serde_json::Value,
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct SyncPullResponse {
    pub changes: Vec<SyncChange>,
    pub server_timestamp: String,
}
```

---

### 5.3 Embedded Axum HTTP Server（Giver 端）

**`src-tauri/src/sync/server.rs`**：

```rust
pub struct SyncServer {
    shutdown_tx: tokio::sync::oneshot::Sender<()>,
    port: u16,
}

impl SyncServer {
    pub async fn start(pool: SqlitePool) -> Result<Self> {
        let port = find_available_port();  // 隨機可用 port

        let app = Router::new()
            .route("/sync/handshake", post(handshake_handler))
            .route("/sync/push", post(push_handler))
            .route("/sync/pull", get(pull_handler))
            .route("/sync/attachments/{uuid}", get(download_attachment))
            .route("/sync/attachments", post(upload_attachment))
            .with_state(AppState { pool });

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

        tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async { shutdown_rx.await.ok(); })
                .await
                .ok();
        });

        Ok(Self { shutdown_tx, port })
    }

    pub fn port(&self) -> u16 { self.port }

    pub fn stop(self) {
        self.shutdown_tx.send(()).ok();
    }
}
```

**Handler 實作**：

| Endpoint | Handler | 說明 |
|----------|---------|------|
| `POST /sync/handshake` | `handshake_handler` | 驗證 device_uuid + family_uuid（或首次配對碼），回傳 success |
| `POST /sync/push` | `push_handler` | 接收 Baby 的 `SyncChange[]` → 呼叫 merge engine → 回傳 accepted/rejected |
| `GET /sync/pull?since=` | `pull_handler` | 回傳 Giver 端 since 之後所有表的變更 |
| `GET /sync/attachments/{uuid}` | `download_attachment` | 讀取本地附件檔案回傳 |
| `POST /sync/attachments` | `upload_attachment` | 接收附件存入本地 |

---

### 5.4 Sync Client（Baby 端）

**`src-tauri/src/sync/client.rs`**：

```rust
pub struct SyncClient {
    base_url: String,  // http://{ip}:{port}
}

impl SyncClient {
    pub fn new(peer: &PeerInfo) -> Self {
        Self { base_url: format!("http://{}:{}", peer.ip, peer.port) }
    }

    pub async fn handshake(&self, req: SyncHandshakeRequest) -> Result<SyncHandshakeResponse>;
    pub async fn push(&self, req: SyncPushRequest) -> Result<SyncPushResponse>;
    pub async fn pull(&self, since: &str) -> Result<SyncPullResponse>;
    pub async fn download_attachment(&self, uuid: &str) -> Result<Vec<u8>>;
    pub async fn upload_attachment(&self, uuid: &str, data: Vec<u8>) -> Result<()>;
}
```

> 使用 `reqwest` crate 作為 HTTP client（需新增至 `Cargo.toml`：`reqwest = { version = "0.12", features = ["json"] }`）。

---

### 5.5 Merge Engine（Giver 端核心）

**`src-tauri/src/sync/merge.rs`**：

```rust
pub struct MergeResult {
    pub accepted: Vec<String>,
    pub rejected: Vec<String>,
}

/// Giver 端合併 Baby push 的變更
pub async fn merge_changes(pool: &SqlitePool, changes: Vec<SyncChange>) -> Result<MergeResult> {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for change in changes {
        match change.table.as_str() {
            "transactions" => {
                // Append-only: 只要 uuid 不重複就接受
                if !record_exists(pool, "transactions", &change.uuid).await? {
                    insert_record(pool, &change).await?;
                    accepted.push(change.uuid);
                }
                // 已存在的 transaction 忽略（不算拒絕）
            }
            table => {
                let local = get_record_version(pool, table, &change.uuid).await?;
                match local {
                    Some(local_version) => {
                        if change.sync_version > local_version {
                            // Baby 版本較新 → 接受
                            upsert_record(pool, &change).await?;
                            accepted.push(change.uuid);
                        } else {
                            // Giver 版本 ≥ Baby → 拒絕（Giver 優先）
                            rejected.push(change.uuid);
                        }
                    }
                    None => {
                        // 新記錄 → 接受
                        insert_record(pool, &change).await?;
                        accepted.push(change.uuid);
                    }
                }
            }
        }
    }

    Ok(MergeResult { accepted, rejected })
}
```

**Pull 邏輯**（回傳 Giver 端的變更給 Baby）：

```rust
/// 查詢所有表中 updated_at > since 的記錄
pub async fn collect_changes_since(pool: &SqlitePool, since: &str) -> Result<Vec<SyncChange>> {
    let mut changes = Vec::new();

    for table in SYNCABLE_TABLES {
        let rows = sqlx::query(&format!(
            "SELECT * FROM {} WHERE updated_at > ? OR created_at > ?", table
        ))
        .bind(since).bind(since)
        .fetch_all(pool).await?;

        for row in rows {
            changes.push(row_to_sync_change(table, row)?);
        }
    }

    Ok(changes)
}

const SYNCABLE_TABLES: &[&str] = &[
    "profiles", "families", "family_members", "wallets",
    "allowances", "requests", "transactions",
];
```

---

### 5.6 Baby 端 Pull 後處理

**`src-tauri/src/sync/apply.rs`**（或在 `client.rs` 中）：

Baby 收到 Giver 的 changes 後：

```
1. 遍歷所有 SyncChange
2. 對每條記錄執行 UPSERT（直接覆蓋本地，Giver 為權威）
3. 對每個受影響的 wallet，重算 balance_cents：
   SELECT SUM(CASE WHEN type='credit' THEN amount_cents ELSE -amount_cents END)
   FROM transactions WHERE wallet_id = ? AND is_deleted = 0
4. 比較同步前後差異，偵測新資料（新的 approved request、新的 allowance_disbursed 等）
5. 產生本地 notifications
6. 更新 sync_state 的 last_sync_timestamp
```

---

### 5.7 Sync Coordinator

**`src-tauri/src/sync/coordinator.rs`**：

整合所有同步子模組的高層控制器：

```rust
pub struct SyncCoordinator { /* handles */ }

impl SyncCoordinator {
    /// Giver: start HTTP server + mDNS broadcast
    pub async fn start_as_giver(pool: SqlitePool, family_uuid: &str, device_name: &str) -> Result<Self>;

    /// Baby: discover + handshake + push + pull
    pub async fn start_as_baby(pool: SqlitePool, family_uuid: &str, device_uuid: &str) -> Result<Self>;

    /// Stop all sync activity
    pub fn stop(&mut self);

    /// Get current sync status
    pub fn status(&self) -> SyncStatus;
}

#[derive(Debug, Serialize, Type, Clone)]
pub struct SyncStatus {
    pub state: String,          // "disconnected" | "discovering" | "connected" | "syncing" | "error"
    pub last_sync_at: Option<String>,
    pub peer_device_name: Option<String>,
    pub error_message: Option<String>,
}
```

---

### 5.8 App 生命週期整合

在 `src-tauri/src/lib.rs` 的 `setup` 中：

```rust
// 判斷角色，啟動對應同步模式
let profile = services::setup_service::get_profile(&pool).await?;
if let Some(profile) = profile {
    let family = services::family_service::get_family(&pool).await?;
    if let Some(family) = family {
        match profile.role.as_str() {
            "giver" => {
                let coordinator = SyncCoordinator::start_as_giver(
                    pool.clone(), &family.uuid, &profile.display_name
                ).await?;
                app.manage(Arc::new(Mutex::new(coordinator)));
            }
            "baby" => {
                let coordinator = SyncCoordinator::start_as_baby(
                    pool.clone(), &family.uuid, &profile.uuid
                ).await?;
                app.manage(Arc::new(Mutex::new(coordinator)));
            }
            _ => {}
        }
    }
}
```

前台/背景切換（透過 Tauri app lifecycle events）：
- `RunEvent::Resumed` → 啟動 sync
- `RunEvent::Suspended` → 停止 sync

---

### 5.9 Sync Commands

**`src-tauri/src/commands/sync.rs`**：

```rust
pub async fn start_sync(pool: State<'_, SqlitePool>, coordinator: State<'_, Arc<Mutex<SyncCoordinator>>>) -> Result<(), AppError>;
pub async fn stop_sync(coordinator: State<'_, Arc<Mutex<SyncCoordinator>>>) -> Result<(), AppError>;
pub async fn get_sync_status(coordinator: State<'_, Arc<Mutex<SyncCoordinator>>>) -> Result<SyncStatus, AppError>;
pub async fn trigger_sync(coordinator: State<'_, Arc<Mutex<SyncCoordinator>>>) -> Result<(), AppError>;
pub async fn get_paired_devices(pool: State<'_, SqlitePool>) -> Result<Vec<PairedDevice>, AppError>;
```

---

### 5.10 Frontend: Sync Store + UI

**`src/stores/sync-store.ts`**：

```typescript
interface SyncState {
  status: "disconnected" | "discovering" | "connected" | "syncing" | "error";
  lastSyncAt: string | null;
  peerDeviceName: string | null;
  errorMessage: string | null;
  fetchStatus: () => Promise<void>;
}
```

**`src/components/sync-indicator.tsx`**：

同步狀態指示器：
- 🟢 `connected` — 已連線
- 🔄 `syncing` — 同步中
- 🟡 `discovering` — 搜尋中
- ⚪ `disconnected` — 未連線
- 🔴 `error` — 錯誤

放在 Dashboard header + AppLayout header。

---

### 5.11 Pairing Page 整合（更新 Phase 2）

更新 `src/app/pairing.tsx`：

Baby 配對流程整合真正的 mDNS + HTTP handshake：

```
1. Baby 輸入配對碼
2. mDNS 搜尋同 WiFi 的 Giver
3. 找到 → HTTP handshake（帶 pairing_code）
4. 驗證成功 → 同步家庭資料 → 導向 Dashboard
```

Giver 配對頁整合：

```
1. 產生配對碼
2. 啟動 mDNS 廣播 + HTTP server
3. 等待 Baby handshake
4. 驗證配對碼 → 完成配對
```

---

### 5.12 Backend 測試（重點中的重點）

**`src-tauri/tests/sync_merge_test.rs`**：

- [ ] 兩個 in-memory SQLite instance 模擬 Giver/Baby
- [ ] Baby 建立 request → serialize → merge into Giver → 成功
- [ ] 衝突：同時修改同一 wallet 名稱 → Giver 版本優先
- [ ] transactions append-only → uuid 不重複即接受
- [ ] balance 重算：sync 後 `SUM(transactions)` == `wallet.balance_cents`
- [ ] soft delete 傳播：Giver 刪除 → Baby pull → is_deleted = 1

**`src-tauri/tests/sync_protocol_test.rs`**：

- [ ] SyncChange 序列化/反序列化正確
- [ ] handshake 驗證成功/失敗
- [ ] pull since 篩選正確（只回傳 since 之後的變更）
- [ ] push + pull round-trip 資料一致

---

### 驗證步驟

- [ ] `cargo test` — 所有 sync 測試通過
- [ ] 兩台裝置（或模擬器）同 WiFi：
  - Giver 建立錢包 → Baby pull → 錢包出現
  - Baby 建立請款 → push → Giver 看到 pending 請款
  - Giver 核准 → Baby pull → 餘額更新 + notification
- [ ] 離線操作 → 重新連線 → 資料正確同步
- [ ] mDNS 發現 < 5 秒
- [ ] 同步完成 < 3 秒（少量資料）
- [ ] 同步指示器 UI 正確反映連線狀態
- [ ] App 背景 → 前台 → 自動恢復同步

---

# Phase 6 — P1: 定期津貼 + 交易紀錄 + 通知系統

**目標**：實作定期零用金自動發放、完整交易帳本查詢、App 內通知。

**前置條件**：Phase 4（請款核准已建立 transaction 機制）, Phase 5（同步引擎運作）

---

### 6.1 Backend: Allowance Service

**`src-tauri/src/services/allowance_service.rs`**：

| 方法 | 說明 |
|------|------|
| `create_allowance(pool, params) -> Result<Allowance>` | 計算 `next_run_at` |
| `update_allowance(pool, id, params) -> Result<Allowance>` | 更新金額/頻率 |
| `pause_allowance(pool, id) -> Result<()>` | status → paused |
| `resume_allowance(pool, id) -> Result<()>` | status → active，重算 `next_run_at` |
| `archive_allowance(pool, id) -> Result<()>` | status → archived |
| `list_allowances(pool, family_id) -> Result<Vec<Allowance>>` | 列表 |
| `execute_allowance(pool, allowance) -> Result<()>` | **核心**，見下方 |

**`execute_allowance` 核心流程**（SQLite Transaction）：

```
1. INSERT INTO transactions (type='credit', source_type='allowance', source_id=allowance.id)
2. UPDATE wallets SET balance_cents += amount_cents
3. UPDATE allowances SET last_run_at = now(), next_run_at = calculate_next()
4. 遞增 wallet.sync_version + allowance.sync_version
5. INSERT INTO notifications (event_type='allowance_disbursed')
6. INSERT INTO audit_logs
7. 如 wallet.balance_cents < warning_threshold_cents → INSERT notification (low_balance)
8. COMMIT
```

**`next_run_at` 計算**：

```rust
fn calculate_next_run(frequency: &str, interval: i64, from: NaiveDateTime) -> NaiveDateTime {
    match frequency {
        "daily"    => from + Duration::days(interval),
        "weekly"   => from + Duration::weeks(interval),
        "biweekly" => from + Duration::weeks(2 * interval),
        "monthly"  => from + Months::new(interval as u32),
        "custom"   => from + Duration::days(interval),
        _ => from + Duration::days(interval),
    }
}
```

---

### 6.2 Backend: Allowance Runner

**`src-tauri/src/scheduler/allowance_runner.rs`**：

```rust
/// 僅在 Giver 裝置執行
/// App 前台時每 60 秒檢查一次到期津貼
pub async fn start(pool: SqlitePool, cancel: CancellationToken) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            _ = tokio::time::sleep(Duration::from_secs(60)) => {
                let due = sqlx::query_as::<_, Allowance>(
                    "SELECT * FROM allowances \
                     WHERE status = 'active' AND next_run_at <= datetime('now') AND is_deleted = 0"
                ).fetch_all(&pool).await;

                if let Ok(allowances) = due {
                    for a in allowances {
                        if let Err(e) = allowance_service::execute_allowance(&pool, &a).await {
                            tracing::error!("allowance {} execute failed: {}", a.id, e);
                        }
                    }
                }
            }
        }
    }
}
```

在 `lib.rs` 中，判斷角色為 Giver 時啟動：

```rust
if profile.role == "giver" {
    let cancel = CancellationToken::new();
    tokio::spawn(scheduler::allowance_runner::start(pool.clone(), cancel.clone()));
    app.manage(cancel);
}
```

> 需新增依賴：`tokio-util = { version = "0.7", features = ["rt"] }`（CancellationToken）

---

### 6.3 Backend: Transaction Service（完整）

**`src-tauri/src/services/transaction_service.rs`**（擴充 Phase 4 基礎）：

| 方法 | 說明 |
|------|------|
| `list_transactions(pool, family_id, filters) -> Result<Vec<Transaction>>` | 支援日期範圍、錢包、類型篩選 |
| `list_wallet_transactions(pool, wallet_id, filters) -> Result<Vec<Transaction>>` | 錢包交易紀錄 |
| `create_manual_transaction(pool, params) -> Result<Transaction>` | Giver 手動記帳 |
| `get_monthly_summary(pool, family_id, year, month) -> Result<MonthlySummary>` | 月度統計 |

```rust
#[derive(Debug, Serialize, Type)]
pub struct MonthlySummary {
    pub total_credit_cents: i64,
    pub total_debit_cents: i64,
    pub net_change_cents: i64,
    pub transaction_count: i64,
}

#[derive(Debug, Deserialize, Type)]
pub struct TransactionFilters {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub wallet_id: Option<i64>,
    pub transaction_type: Option<String>,
    pub source_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
```

---

### 6.4 Backend: Notification Service

**`src-tauri/src/services/notification_service.rs`**：

| 方法 | 說明 |
|------|------|
| `create_notification(pool, event_type, payload) -> Result<Notification>` | 建立通知 |
| `list_notifications(pool, limit, offset) -> Result<Vec<Notification>>` | 列表 |
| `mark_read(pool, id) -> Result<()>` | 標記已讀 |
| `mark_all_read(pool) -> Result<()>` | 全部已讀 |
| `unread_count(pool) -> Result<i64>` | 未讀數量 |

---

### 6.5 Backend: Commands

**`src-tauri/src/commands/allowance.rs`**：

```rust
pub async fn create_allowance(params: CreateAllowanceParams, pool: ...) -> Result<Allowance, AppError>;
pub async fn update_allowance(id: i64, params: UpdateAllowanceParams, pool: ...) -> Result<Allowance, AppError>;
pub async fn pause_allowance(id: i64, pool: ...) -> Result<(), AppError>;
pub async fn resume_allowance(id: i64, pool: ...) -> Result<(), AppError>;
pub async fn list_allowances(family_id: i64, pool: ...) -> Result<Vec<Allowance>, AppError>;
```

**`src-tauri/src/commands/transaction.rs`**：

```rust
pub async fn list_transactions(family_id: i64, filters: TransactionFilters, pool: ...) -> Result<Vec<Transaction>, AppError>;
pub async fn create_manual_transaction(params: ManualTransactionParams, pool: ...) -> Result<Transaction, AppError>;
pub async fn get_monthly_summary(family_id: i64, year: i32, month: u32, pool: ...) -> Result<MonthlySummary, AppError>;
```

**`src-tauri/src/commands/notification.rs`**：

```rust
pub async fn list_notifications(limit: i64, offset: i64, pool: ...) -> Result<Vec<Notification>, AppError>;
pub async fn mark_notification_read(id: i64, pool: ...) -> Result<(), AppError>;
pub async fn mark_all_notifications_read(pool: ...) -> Result<(), AppError>;
pub async fn unread_notification_count(pool: ...) -> Result<i64, AppError>;
```

---

### 6.6 Frontend: Hooks

**`src/hooks/use-allowances.ts`**：

```typescript
export function useAllowances(familyId: number);
export function useCreateAllowance();   // invalidate ['allowances']
export function usePauseAllowance();    // invalidate ['allowances']
export function useResumeAllowance();   // invalidate ['allowances']
```

**`src/hooks/use-transactions.ts`**：

```typescript
export function useTransactions(familyId: number, filters?: TransactionFilters);
export function useWalletTransactions(walletId: number, limit?: number);
export function useMonthlySummary(familyId: number, year: number, month: number);
export function useCreateManualTransaction();  // invalidate ['transactions'] + ['wallets']
```

**`src/hooks/use-notifications.ts`**：

```typescript
export function useNotifications(limit?: number);
export function useUnreadCount();
export function useMarkRead();
export function useMarkAllRead();
```

---

### 6.7 Frontend: Components

| 檔案 | 說明 |
|------|------|
| `src/components/allowance-card.tsx` | 津貼卡片：發放者 → 接收者、金額、頻率、狀態、下次發放 |
| `src/components/transaction-item.tsx` | 交易列表項：金額（綠色 credit / 紅色 debit）、來源圖示、分類、時間 |
| `src/components/notification-item.tsx` | 通知列表項：事件圖示、摘要文字、時間、已讀/未讀狀態 |
| `src/components/monthly-summary.tsx` | 月度統計卡片：總入帳 / 總出帳 / 淨變動 |
| `src/components/transaction-filters.tsx` | 篩選器：日期範圍 + 錢包 + 類型 |

---

### 6.8 Frontend: Pages

**`src/app/allowances/index.tsx`**：

- 津貼卡片列表
- Giver：新增津貼按鈕 → Dialog（接收者、錢包、金額、頻率）
- 暫停/恢復操作
- Baby：唯讀，查看津貼規則

**`src/app/transactions/index.tsx`**：

- 交易列表（時間倒序）
- 篩選器（日期、錢包、類型、來源）
- 月度摘要卡片（置頂）
- Giver：FAB「手動記帳」→ Dialog（錢包、類型 credit/debit、金額、分類、備註）

**`src/app/notifications/index.tsx`**：

- 通知列表（時間倒序）
- 已讀/未讀視覺區分
- 點擊標記已讀 + 導航到相關頁面（如 `request_approved` → `/requests/:id`）
- 頂部「全部已讀」按鈕
- Tab Bar 通知 Tab Badge 顯示 `unreadCount`

---

### 6.9 Backend 測試

**`src-tauri/tests/allowance_test.rs`**：

- [ ] 建立津貼 → `next_run_at` 正確（daily/weekly/monthly/custom）
- [ ] `execute_allowance` → wallet 餘額增加 + transaction 建立 + notification
- [ ] 低餘額警告 → 額外 notification
- [ ] pause → resume → `next_run_at` 從 now() 重算
- [ ] sync_version 遞增

**`src-tauri/tests/transaction_test.rs`**：

- [ ] 列表篩選（日期、錢包、類型）
- [ ] 手動記帳（credit + debit）→ 餘額正確
- [ ] 月度統計計算正確

---

### 驗證步驟

- [ ] `cargo test` — allowance + transaction + notification 測試通過
- [ ] Giver 建立津貼（每日 $100）→ runner 執行 → 餘額增加 + notification
- [ ] 交易紀錄頁顯示所有來源（allowance + request + manual）
- [ ] 通知中心顯示事件，點擊可標記已讀
- [ ] 津貼發放結果正確同步到 Baby
- [ ] 通知 Badge 數字正確

---

# Phase 7 — P1: Dashboard 整合

**目標**：完成 Dashboard 頁面，整合所有已完成的功能模組。

**前置條件**：Phase 3, 4, 5, 6 全部完成

---

### 7.1 Backend: Dashboard Command

**`src-tauri/src/commands/dashboard.rs`**：

一次 IPC 呼叫取得所有 Dashboard 資料（避免 N+1）：

```rust
#[derive(Debug, Serialize, Type)]
pub struct DashboardData {
    pub profile: Profile,
    pub family: Option<Family>,
    pub pending_requests_count: i64,
    pub wallets: Vec<Wallet>,
    pub recent_transactions: Vec<Transaction>,   // 最近 5 筆
    pub request_summary: RequestSummary,
    pub sync_status: SyncStatus,
    pub unread_notification_count: i64,
}

#[derive(Debug, Serialize, Type)]
pub struct RequestSummary {
    pub pending: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[tauri::command]
#[specta::specta]
pub async fn get_dashboard_data(pool: State<'_, SqlitePool>, coordinator: ...) -> Result<DashboardData, AppError>;
```

---

### 7.2 Frontend: Dashboard Page

**`src/app/dashboard.tsx`**：

依角色條件渲染：

**Giver 版**：

```
┌─────────────────────────────────────────────┐
│ 👋 Hello, {name}              🟢 已連線     │
├─────────────────────────────────────────────┤
│ 🔴 {n} 筆待審核請款                  → 查看 │
├─────────────────────────────────────────────┤
│ 錢包餘額          (水平滾動卡片)             │
│ ┌──────┐ ┌──────┐ ┌──────┐                  │
│ │現金  │ │銀行  │ │虛擬  │                  │
│ │$3000 │ │$1500 │ │$200  │                  │
│ └──────┘ └──────┘ └──────┘                  │
├─────────────────────────────────────────────┤
│ 最近交易                                     │
│ • 津貼發放  +$100  今天                      │
│ • 請款核准  -$50   昨天                      │
│ • ...                                        │
├─────────────────────────────────────────────┤
│ [審核請款]          [手動記帳]               │
└─────────────────────────────────────────────┘
```

**Baby 版**：

```
┌─────────────────────────────────────────────┐
│ 👋 Hello, {name}              🟢 已連線     │
├─────────────────────────────────────────────┤
│          NT$ 3,000                           │
│          主要錢包餘額                        │
├─────────────────────────────────────────────┤
│ 最近津貼：+$100 (每日)                      │
├─────────────────────────────────────────────┤
│ 我的請款     待審核: 2  核准: 5  駁回: 1    │
├─────────────────────────────────────────────┤
│         [建立請款]                           │
└─────────────────────────────────────────────┘
```

---

### 7.3 Frontend: Hook

**`src/hooks/use-dashboard.ts`**：

```typescript
export function useDashboard() {
  return useQuery({
    queryKey: ["dashboard"],
    queryFn: () => commands.getDashboardData(),
    refetchInterval: 30_000,  // 每 30 秒自動刷新
  });
}
```

---

### 驗證步驟

- [ ] Giver Dashboard 顯示：待審核數量、錢包餘額、最近交易、快捷操作
- [ ] Baby Dashboard 顯示：主錢包餘額、津貼、請款摘要、建立請款按鈕
- [ ] 點擊快捷操作正確導航
- [ ] 同步指示器反映真實狀態
- [ ] 資料即時更新（操作後 invalidate dashboard）

---

# Phase 8 — P2: 生物辨識鎖定 + 審計日誌 + CSV 匯出

**目標**：加入安全鎖定、操作追蹤、資料匯出功能。

**前置條件**：Phase 7（所有核心功能完成）

---

### 8.1 PIN + 生物辨識鎖定

**Backend**：

**`src-tauri/src/services/auth_service.rs`**：

| 方法 | 說明 |
|------|------|
| `set_pin(pool, pin) -> Result<()>` | Argon2id hash → 存入 `profiles.pin_hash` |
| `verify_pin(pool, pin) -> Result<bool>` | 驗證 PIN |
| `remove_pin(pool) -> Result<()>` | 清除 PIN |
| `has_pin(pool) -> Result<bool>` | 是否已設定 PIN |

**`src-tauri/src/commands/auth.rs`**：

```rust
pub async fn set_pin(pin: String, pool: ...) -> Result<(), AppError>;
pub async fn verify_pin(pin: String, pool: ...) -> Result<bool, AppError>;
pub async fn remove_pin(pool: ...) -> Result<(), AppError>;
pub async fn has_pin(pool: ...) -> Result<bool, AppError>;
pub async fn enable_biometric() -> Result<(), AppError>;
pub async fn verify_biometric() -> Result<bool, AppError>;
```

**Frontend**：

| 檔案 | 說明 |
|------|------|
| `src/app/lock-screen.tsx` | PIN 4~6 位數字輸入 + 生物辨識按鈕 |
| `src/components/lock-guard.tsx` | App 回到前台時檢查是否需要解鎖 |

**`src/components/lock-guard.tsx`**：

```tsx
function LockGuard({ children }) {
  const [isLocked, setIsLocked] = useState(false);

  useEffect(() => {
    // 監聽 Tauri app lifecycle: resume → check if PIN is set → lock
    const unlisten = listen("tauri://resumed", async () => {
      const hasPin = await commands.hasPin();
      if (hasPin) setIsLocked(true);
    });
    return () => { unlisten.then(fn => fn()); };
  }, []);

  if (isLocked) return <LockScreen onUnlock={() => setIsLocked(false)} />;
  return children;
}
```

Settings 頁面：
- PIN 設定/變更/移除
- 生物辨識開關（`tauri-plugin-biometric`）

---

### 8.2 審計日誌

**Backend**：

**`src-tauri/src/services/audit_service.rs`**：

| 方法 | 說明 |
|------|------|
| `log_action(pool, actor_device, family_id, action, resource_type, resource_id, metadata)` | 寫入 audit_logs |
| `list_audit_logs(pool, family_id, limit, offset) -> Result<Vec<AuditLog>>` | 列表 |

> 此 service 已在 Phase 4/6 的各 service 方法中呼叫。Phase 8 主要完成前端展示。

**`src-tauri/src/commands/audit.rs`**：

```rust
pub async fn list_audit_logs(family_id: i64, limit: i64, offset: i64, pool: ...) -> Result<Vec<AuditLog>, AppError>;
```

**Frontend**：

**`src/app/settings/audit-log.tsx`**（Giver 專屬）：

- 審計日誌列表（時間倒序）
- 顯示：操作者裝置、行為、資源類型、時間
- 從 Settings 頁面導入

---

### 8.3 CSV 匯出

**Backend**：

**`src-tauri/src/services/export_service.rs`**：

```rust
pub async fn export_transactions_csv(
    pool: &SqlitePool,
    family_id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
    wallet_id: Option<i64>,
) -> Result<String> {
    // CSV header: Date,Wallet,Type,Amount,Source,Category,Notes
    // 查詢 transactions + JOIN wallets
    // 格式化為 CSV 字串
}
```

**`src-tauri/src/commands/export.rs`**：

```rust
pub async fn export_csv(
    family_id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
    wallet_id: Option<i64>,
    pool: ...,
) -> Result<String, AppError>;  // 回傳 CSV 字串
```

**Frontend**：

Settings 頁面新增「匯出交易紀錄」：
- Dialog：選擇日期範圍 + 錢包（可選全部）
- 呼叫 `exportCsv()` 取得 CSV 字串
- 使用系統分享功能（或 `tauri::api::dialog::save_file`）儲存

---

### 8.4 Frontend: Settings Page 完善

**`src/app/settings/index.tsx`**：

- 裝置資訊（顯示名稱、角色、裝置 UUID）
- 主題切換（Light / Dark / High Contrast）— Phase 9 完善
- 語言切換 — Phase 9 完善
- **App 鎖定**（PIN 設定 / 生物辨識開關）— 此 Phase
- **匯出交易紀錄** — 此 Phase
- 同步狀態（最後同步時間、已配對裝置列表）
- **審計日誌**（Giver）→ 導向 `audit-log.tsx` — 此 Phase
- 重設裝置（`AlertDialog` 確認）

---

### 8.5 Frontend: Family Page

**`src/app/family.tsx`**（Giver 專屬）：

- 家庭名稱
- 已配對成員列表（名稱、角色、最後同步時間）
- 「邀請成員」按鈕 → 產生配對碼 Dialog
- 「移除成員」→ `AlertDialog` 確認 → `removeFamilyMember()`

---

### 驗證步驟

- [ ] 設定 4 位 PIN → App 背景 → 回前台 → PIN 輸入畫面出現
- [ ] 正確 PIN → 解鎖；錯誤 PIN → 錯誤提示
- [ ] 生物辨識（實機測試 — 模擬器可能不支援）
- [ ] 審計日誌頁面顯示所有操作紀錄
- [ ] CSV 匯出 → 內容正確（欄位、金額、日期）→ 系統分享
- [ ] 家庭管理頁面正確顯示成員
- [ ] Settings 所有功能入口正確

---

# Phase 9 — P3: 多語言 + 高對比主題 + 無障礙

**目標**：i18n 英文支援、High Contrast 主題、WCAG AA 無障礙合規。

**前置條件**：Phase 8（所有功能已完成，此 Phase 為 polish）

---

### 9.1 i18n 完善

**`src/i18n/index.ts`**：

```typescript
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import zhTW from "./zh-TW.json";
import en from "./en.json";

i18n.use(initReactI18next).init({
  resources: {
    "zh-TW": { translation: zhTW },
    en: { translation: en },
  },
  lng: "zh-TW",
  fallbackLng: "zh-TW",
  interpolation: { escapeValue: false },
});

export default i18n;
```

**`src/i18n/zh-TW.json`** + **`src/i18n/en.json`**：

完整翻譯檔案。所有頁面的硬編碼中文字串替換為 `t('key')`。

結構範例：

```json
{
  "common": {
    "save": "儲存",
    "cancel": "取消",
    "confirm": "確認",
    "delete": "刪除",
    "edit": "編輯",
    "loading": "載入中...",
    "retry": "重試",
    "empty": "暫無資料"
  },
  "setup": {
    "title": "歡迎使用 Cacao",
    "displayName": "顯示名稱",
    "selectRole": "選擇角色",
    "giver": "家長",
    "baby": "子女"
  },
  "wallet": { ... },
  "request": { ... },
  "allowance": { ... },
  "notification": { ... },
  "settings": { ... }
}
```

**`src/stores/ui-store.ts`**：

```typescript
interface UiState {
  locale: "zh-TW" | "en";
  theme: "light" | "dark" | "high-contrast";
  setLocale: (locale: string) => void;
  setTheme: (theme: string) => void;
}
```

`setLocale` → 呼叫 `i18n.changeLanguage()` + 存入 `tauri-plugin-store`。

---

### 9.2 主題系統

**`src/styles/globals.css`** 擴充：

```css
@import "tailwindcss";

@theme {
  /* Light theme (default) */
  --color-background: oklch(0.99 0 0);
  --color-foreground: oklch(0.15 0 0);
  --color-primary: oklch(0.65 0.15 55);
  --color-primary-foreground: oklch(0.98 0 0);
  /* ... */
}

.dark {
  --color-background: oklch(0.15 0 0);
  --color-foreground: oklch(0.95 0 0);
  --color-primary: oklch(0.70 0.15 55);
  /* ... */
}

.high-contrast {
  --color-background: oklch(0 0 0);
  --color-foreground: oklch(1 0 0);
  --color-primary: oklch(0.80 0.20 55);
  /* WCAG AAA contrast ratios (7:1+) */
}
```

`setTheme` → 在 `<html>` 上切換 class：`""` / `"dark"` / `"high-contrast"`。

---

### 9.3 無障礙（Accessibility）

全域改善：

- [ ] 所有互動元素加入 `aria-label`（特別是 icon-only 按鈕）
- [ ] 同步狀態指示器使用 `aria-live="polite"` 廣播狀態變化
- [ ] 觸控目標 ≥ 48×48 dp（Tailwind: `min-h-12 min-w-12`）
- [ ] 所有字型使用 `rem`，支援 200% 縮放
- [ ] 動畫遵循 `prefers-reduced-motion`：

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}
```

- [ ] `@media (prefers-contrast: more)` 自動切換高對比
- [ ] 表單元素正確關聯 `<label>` 與 `<input>`

---

### 驗證步驟

- [ ] 切換語言至 English → 所有文字正確切換（無殘留中文）
- [ ] 切回繁體中文 → 正確
- [ ] Dark mode 切換 → 所有頁面背景/文字正確
- [ ] High contrast → 所有文字/背景對比度 ≥ 4.5:1（WCAG AA）
- [ ] VoiceOver (iOS) / TalkBack (Android) 可正確朗讀所有元素
- [ ] 字型放大至 200% → 版面不破碎
- [ ] 所有按鈕觸控面積 ≥ 48×48 dp

---

# Phase 10 — CI/CD + 品質收尾 + Release

**目標**：自動化測試管線、最終品質檢查、首次 Release 建置。

**前置條件**：Phase 9

---

### 10.1 CI Pipeline

**`.github/workflows/ci.yml`**：

```yaml
name: CI
on: [push, pull_request]

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo clippy --all-targets --manifest-path src-tauri/Cargo.toml -- -D warnings
      - run: cargo fmt --check --manifest-path src-tauri/Cargo.toml
      - run: cargo test --manifest-path src-tauri/Cargo.toml

  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with: { node-version: 22 }
      - run: pnpm install --frozen-lockfile
      - run: pnpm lint
      - run: pnpm test
```

---

### 10.2 Release Pipeline

**`.github/workflows/release.yml`**：

```yaml
name: Release
on:
  push:
    tags: ["v*.*.*"]

jobs:
  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - # Setup JDK, Android SDK, Rust, Node, pnpm
      - run: pnpm install --frozen-lockfile
      - run: cargo tauri android build --release
      - uses: actions/upload-artifact@v4
        with:
          name: android-apk
          path: src-tauri/gen/android/app/build/outputs/apk/release/

  build-ios:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - # Setup Xcode, Rust, Node, pnpm
      - run: pnpm install --frozen-lockfile
      - run: cargo tauri ios build --release
      - uses: actions/upload-artifact@v4
        with:
          name: ios-ipa
          path: src-tauri/gen/apple/build/
```

---

### 10.3 E2E 測試檢查清單（手動 / 半自動）

| # | 場景 | 驗證項目 |
|---|------|---------|
| 1 | 裝置設定（Giver） | 名稱 + 角色 → profile 建立 |
| 2 | 建立家庭 | 家庭名稱 → family 建立 |
| 3 | 建立錢包 | 名稱 + 類型 + 初始餘額 → 錢包建立 |
| 4 | 裝置設定（Baby） | 名稱 + 角色 → profile 建立 |
| 5 | 配對 | 同 WiFi + 配對碼 → 加入家庭 |
| 6 | 同步 | Baby 看到 Giver 的錢包 |
| 7 | Baby 建立請款 | 金額 + 分類 → 草稿 → 送出 |
| 8 | 同步 | Giver 看到 pending 請款 |
| 9 | Giver 核准 | 餘額扣除 + transaction |
| 10 | 同步 | Baby 看到 approved + 餘額更新 + notification |
| 11 | 津貼發放 | Giver 建立每日津貼 → runner 執行 → 餘額增加 |
| 12 | 同步 | Baby 看到津貼 + 新交易 |
| 13 | PIN 鎖定 | 設定 PIN → 背景 → 前台 → 需解鎖 |
| 14 | CSV 匯出 | 選日期 + 錢包 → CSV 正確 → 系統分享 |
| 15 | 語言切換 | zh-TW → en → zh-TW |
| 16 | 主題切換 | Light → Dark → High Contrast |
| 17 | 離線操作 | 斷 WiFi → 各自操作 → 重連 → 同步正確 |

---

### 10.4 Lint + Format 配置

**`package.json`** scripts：

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "lint": "eslint src/",
    "test": "vitest run",
    "test:watch": "vitest"
  }
}
```

**`eslint.config.js`**：配置 React + TypeScript rules。

**`vitest.config.ts`**：

```typescript
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "jsdom",
    setupFiles: ["./src/test/setup.ts"],
    globals: true,
  },
});
```

---

### 驗證步驟

- [ ] CI 全綠（push 一個 commit 觸發）
- [ ] Release tag `v0.1.0` → APK / IPA artifact 產生
- [ ] E2E 檢查清單 17 項全部通過
- [ ] `cargo clippy` 0 warnings
- [ ] `cargo fmt --check` 通過
- [ ] `pnpm lint` 0 errors
- [ ] `pnpm test` 全綠

---

# 附錄 A：完整檔案清單

## Rust Backend (`src-tauri/src/`)

| 檔案 | Phase | 說明 |
|------|-------|------|
| `lib.rs` | 0 | Tauri 入口 |
| `error.rs` | 0 | AppError struct |
| `specta.rs` | 0 | tauri-specta 初始化 + command 註冊 |
| `db/mod.rs` | 1 | DB module |
| `db/pool.rs` | 1 | SQLite 連線池 + migration |
| `models/mod.rs` | 1 | Models re-export |
| `models/profile.rs` | 1 | Profile + Role enum |
| `models/family.rs` | 1 | Family + FamilyMember + enums |
| `models/paired_device.rs` | 1 | PairedDevice |
| `models/wallet.rs` | 1 | Wallet + WalletType/Status enums |
| `models/allowance.rs` | 1 | Allowance + Frequency/Status enums |
| `models/request.rs` | 1 | Request + RequestStatus/Category enums |
| `models/transaction.rs` | 1 | Transaction + TransactionType/SourceType enums |
| `models/notification.rs` | 1 | Notification + EventType enum |
| `models/audit_log.rs` | 1 | AuditLog |
| `models/sync_state.rs` | 1 | SyncState |
| `models/params.rs` | 1 | 共用參數 structs |
| `services/mod.rs` | 2 | Services re-export |
| `services/setup_service.rs` | 2 | 裝置設定 |
| `services/family_service.rs` | 2 | 家庭管理 |
| `services/pairing_service.rs` | 2 | 配對邏輯 |
| `commands/mod.rs` | 2 | Commands re-export |
| `commands/setup.rs` | 2 | 設定 + 配對 commands |
| `services/wallet_service.rs` | 3 | 錢包管理 |
| `commands/wallet.rs` | 3 | 錢包 commands |
| `services/request_service.rs` | 4 | 請款流程（狀態機 + 扣款） |
| `services/transaction_service.rs` | 4+6 | 交易紀錄（Phase 4 基礎, Phase 6 完整） |
| `commands/request.rs` | 4 | 請款 commands |
| `sync/mod.rs` | 5 | Sync module + coordinator |
| `sync/discovery.rs` | 5 | mDNS 廣播與搜尋 |
| `sync/protocol.rs` | 5 | 同步協議資料結構 |
| `sync/server.rs` | 5 | 嵌入式 Axum HTTP server |
| `sync/client.rs` | 5 | HTTP sync client |
| `sync/merge.rs` | 5 | 衝突解決引擎 |
| `sync/apply.rs` | 5 | Baby 端 pull 後處理 |
| `services/allowance_service.rs` | 6 | 津貼管理 + 發放 |
| `services/notification_service.rs` | 6 | 通知 CRUD |
| `scheduler/mod.rs` | 6 | Scheduler module |
| `scheduler/allowance_runner.rs` | 6 | 津貼定時排程 |
| `commands/allowance.rs` | 6 | 津貼 commands |
| `commands/transaction.rs` | 6 | 交易 commands |
| `commands/notification.rs` | 6 | 通知 commands |
| `commands/dashboard.rs` | 7 | Dashboard 聚合 command |
| `services/auth_service.rs` | 8 | PIN + 生物辨識 |
| `services/audit_service.rs` | 8 | 審計日誌 |
| `services/export_service.rs` | 8 | CSV 匯出 |
| `commands/auth.rs` | 8 | PIN/biometric commands |
| `commands/audit.rs` | 8 | 審計日誌 commands |
| `commands/export.rs` | 8 | CSV 匯出 commands |
| `commands/sync.rs` | 5 | 同步 commands |

## Migrations (`src-tauri/migrations/`)

| 檔案 | Phase |
|------|-------|
| `001_init.sql` | 1 |

## React Frontend (`src/`)

| 檔案 | Phase | 說明 |
|------|-------|------|
| `App.tsx` | 0+2 | 根元件（Phase 0 骨架, Phase 2 完整路由） |
| `main.tsx` | 0 | React 入口 |
| `styles/globals.css` | 0+9 | Tailwind CSS 4 主題（Phase 9 擴充 dark/high-contrast） |
| `components/ui/*.tsx` | 0 | shadcn/ui 元件 |
| `lib/bindings.ts` | 0 | tauri-specta 自動產生 |
| `lib/format.ts` | 3 | 金額/日期格式化 |
| `stores/profile-store.ts` | 2 | Profile + Family 狀態 |
| `stores/ui-store.ts` | 9 | 主題 + 語言 |
| `stores/sync-store.ts` | 5 | 同步狀態 |
| `components/setup-guard.tsx` | 2 | 路由守衛 |
| `components/lock-guard.tsx` | 8 | 鎖屏守衛 |
| `components/sync-indicator.tsx` | 5 | 同步狀態指示器 |
| `components/wallet-card.tsx` | 3 | 錢包卡片 |
| `components/amount-display.tsx` | 3 | 金額顯示 |
| `components/status-badge.tsx` | 3 | 通用狀態標籤 |
| `components/empty-state.tsx` | 3 | Empty 狀態 |
| `components/loading-skeleton.tsx` | 3 | 骨架屏 |
| `components/error-state.tsx` | 3 | 錯誤狀態 |
| `components/request-card.tsx` | 4 | 請款卡片 |
| `components/request-status-badge.tsx` | 4 | 請款狀態 Badge |
| `components/allowance-card.tsx` | 6 | 津貼卡片 |
| `components/transaction-item.tsx` | 6 | 交易列表項 |
| `components/notification-item.tsx` | 6 | 通知列表項 |
| `components/monthly-summary.tsx` | 6 | 月度統計卡片 |
| `components/transaction-filters.tsx` | 6 | 交易篩選器 |
| `app/setup.tsx` | 2 | 裝置設定頁 |
| `app/pairing.tsx` | 2+5 | 配對頁（Phase 5 整合真正同步） |
| `app/layout.tsx` | 2 | AppLayout + Tab Bar |
| `app/dashboard.tsx` | 7 | Dashboard |
| `app/wallets/index.tsx` | 3 | 錢包列表 |
| `app/wallets/[id].tsx` | 3+4 | 錢包詳情（Phase 4 加入交易紀錄） |
| `app/requests/index.tsx` | 4 | 請款列表 |
| `app/requests/new.tsx` | 4 | 建立請款（Baby） |
| `app/requests/[id].tsx` | 4 | 請款詳情 + 審核 |
| `app/allowances/index.tsx` | 6 | 津貼管理 |
| `app/transactions/index.tsx` | 6 | 交易紀錄 |
| `app/notifications/index.tsx` | 6 | 通知中心 |
| `app/settings/index.tsx` | 8 | 設定頁 |
| `app/settings/audit-log.tsx` | 8 | 審計日誌 |
| `app/family.tsx` | 8 | 家庭管理（Giver） |
| `app/lock-screen.tsx` | 8 | PIN 解鎖畫面 |
| `hooks/use-wallets.ts` | 3 | 錢包 hooks |
| `hooks/use-requests.ts` | 4 | 請款 hooks |
| `hooks/use-allowances.ts` | 6 | 津貼 hooks |
| `hooks/use-transactions.ts` | 6 | 交易 hooks |
| `hooks/use-notifications.ts` | 6 | 通知 hooks |
| `hooks/use-dashboard.ts` | 7 | Dashboard hook |
| `i18n/index.ts` | 9 | i18next 初始化 |
| `i18n/zh-TW.json` | 9 | 繁體中文翻譯 |
| `i18n/en.json` | 9 | 英文翻譯 |

## CI/CD (`.github/workflows/`)

| 檔案 | Phase |
|------|-------|
| `ci.yml` | 10 |
| `release.yml` | 10 |

## 測試 (`src-tauri/tests/`)

| 檔案 | Phase | 說明 |
|------|-------|------|
| `migration_test.rs` | 1 | Migration schema 驗證 |
| `setup_test.rs` | 2 | 裝置設定 + 配對 |
| `wallet_test.rs` | 3 | 錢包 CRUD + 封存 |
| `request_test.rs` | 4 | 請款狀態機（重點） |
| `sync_merge_test.rs` | 5 | 衝突解決（重點中的重點） |
| `sync_protocol_test.rs` | 5 | 同步協議序列化 |
| `allowance_test.rs` | 6 | 津貼排程 + 發放 |
| `transaction_test.rs` | 6 | 交易篩選 + 統計 |

---

# 附錄 B：關鍵風險與緩解

| 風險 | 影響 | 緩解策略 |
|------|------|---------|
| mDNS 在某些 WiFi AP 被阻擋 | 裝置無法發現 | 提供手動輸入 IP:port 的 fallback |
| Tauri v2 Mobile 尚不穩定 | build/runtime 問題 | 密切追蹤 Tauri GitHub issues，預留 debug 時間 |
| SQLite 並發寫入 | 同步時讀寫衝突 | WAL mode + 短事務 + 重試機制 |
| 同步引擎複雜度 | 資料不一致 | 大量 integration tests + balance 重算校驗 |
| iOS mDNS 權限 | Bonjour 需要特別設定 | `Info.plist` 加入 `NSBonjourServices` + `NSLocalNetworkUsageDescription` |
| Android 背景限制 | sync/scheduler 被殺 | 僅前台同步 + resume 時重啟 |
