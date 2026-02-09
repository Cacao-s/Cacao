# Cacao 專案規格文件

> 本文件記錄 Cacao 專案截至 2026-02-09 的完整規格，作為後續改寫的參考基礎。

---

## 目錄

1. [專案概述](#1-專案概述)
2. [核心功能規格](#2-核心功能規格)
3. [資料庫結構](#3-資料庫結構)
4. [API 設計規格](#4-api-設計規格)
5. [後端架構](#5-後端架構)
6. [前端架構](#6-前端架構)
7. [列舉值與常數](#7-列舉值與常數)
8. [業務規則](#8-業務規則)
9. [基礎設施](#9-基礎設施)

---

## 1. 專案概述

### 1.1 產品名稱

**Cacao** — 家庭零用金管理平台

### 1.2 產品目標

提供家庭成員（家長與子女）一個管理零用金的數位工具，涵蓋：

- 定期自動發放零用金
- 子女提出請款、家長審核
- 錢包餘額管理與交易紀錄
- 離線使用與同步

### 1.3 核心價值

- 讓家長輕鬆管理子女的零用金發放
- 讓子女學習理財、記帳
- 透明的交易紀錄與審計追蹤

### 1.4 使用者角色

| 角色 | 英文代號 | 說明 |
|------|---------|------|
| 家長/給予者 | `giver` | 設定津貼、審核請款、管理錢包 |
| 子女/寶寶 | `baby` | 提出請款、查看津貼、學習理財 |
| 管理員 | `admin` | 系統管理（預留角色） |

### 1.5 家庭角色（Family Role）

在家庭中，成員的角色可以進一步區分：

| 家庭角色 | 說明 |
|---------|------|
| `giver` | 可發放津貼、建立錢包、審核請款 |
| `baby` | 可接收津貼、提出請款 |
| `viewer` | 唯讀檢視家庭資料 |

### 1.6 目標平台

- iOS
- Android

### 1.7 預設語系與幣別

- 語系：`zh-TW`（繁體中文）
- 幣別：`TWD`（新台幣）
- 時區：`Asia/Taipei`

---

## 2. 核心功能規格

### 2.1 身份認證

#### 2.1.1 Email + 密碼登入

- 使用者以 email 註冊帳號
- 密碼使用 bcrypt 雜湊儲存
- 登入成功後回傳 session token
- Token 以 `SessionSecret` 簽署

#### 2.1.2 Google OAuth 登入

- 支援 Google 帳號登入
- 儲存 `google_sub`（Google 使用者唯一識別碼）
- 首次登入自動建立帳號

#### 2.1.3 認證規則

- 除了 `/health` 和 `/auth/login` 以外，所有 API 端點都需要驗證 token
- Token 驗證失敗回傳 `401 Unauthorized`
- 權限不足回傳 `403 Forbidden`

### 2.2 家庭管理

- Giver 可建立家庭（`families`）
- 家庭名稱 + 建立者的組合必須唯一
- 可邀請成員加入家庭（`family_members`）
- 每位成員在家庭中有特定角色（giver / baby / viewer）
- 邀請中的成員狀態為 `pending`，接受後為 `active`
- 成員可被移除（狀態改為 `removed`）

### 2.3 錢包管理

- 每個家庭可建立多個錢包（`wallets`）
- 錢包類型：`cash`（現金）、`bank`（銀行帳戶）、`card`（卡片）、`virtual`（虛擬錢包）
- 餘額以整數（cents）儲存，避免浮點數誤差
- 可設定低餘額警告閾值（`warning_threshold_cents`）
- 錢包可封存（`archived`），封存後不可進行新交易
- 同一家庭內錢包名稱唯一

### 2.4 定期津貼

- Giver 可為 Baby 設定定期津貼（`allowances`）
- 支援頻率：
  - `daily`（每日）
  - `weekly`（每週）
  - `biweekly`（每兩週）
  - `monthly`（每月）
  - `custom`（自訂，搭配 `interval_count` 使用）
- 系統根據 `next_run_at` 自動執行發放
- 發放完成後更新 `last_run_at` 並計算下次 `next_run_at`
- 津貼可暫停（`paused`）或封存（`archived`）
- 每筆津貼關聯一個特定錢包

### 2.5 請款流程

Baby 可向 Giver 提出請款申請（`requests`），狀態流轉如下：

```
draft → pending → approved
                → rejected
                → cancelled
```

- **draft（草稿）**：Baby 尚未送出的請款
- **pending（待審核）**：Baby 送出後等待 Giver 審核
- **approved（核准）**：Giver 核准，自動從錢包扣款並記錄交易
- **rejected（駁回）**：Giver 駁回，必須填寫駁回原因（`rejection_reason`）
- **cancelled（取消）**：Baby 自行取消

請款可附帶：
- 金額（`amount_cents`）
- 分類（`category`）
- 備註（`notes`）
- 附件/收據圖片 URL（`attachment_url`）

### 2.6 交易紀錄

所有金錢異動記錄在 `transactions` 表中，為不可變更的帳本：

- 類型：`credit`（入帳）或 `debit`（出帳）
- 來源類型：
  - `allowance` — 來自定期津貼發放
  - `request` — 來自請款核准
  - `manual` — 手動記帳
  - `adjustment` — 系統調整
- 每筆交易關聯家庭與錢包
- 記錄實際發生時間（`occurred_at`）

### 2.7 通知系統

- 基於事件驅動的通知佇列（`notifications`）
- 通知狀態：
  - `pending` — 尚未發送
  - `sent` — 已成功發送
  - `failed` — 發送失敗
  - `read` — 使用者已讀
- 通知內容以 JSON 格式儲存在 `payload` 欄位
- 失敗的通知會由背景任務重新發送

### 2.8 離線同步機制

- 支援離線操作，操作記錄暫存在本地 `sync_queue`
- 同步佇列記錄：
  - `device_id` — 裝置識別碼
  - `operation_type` — 操作類型
  - `payload` — 操作內容（JSON）
  - `temp_id` — 本地臨時 ID（用於樂觀更新）
- 同步狀態：`pending` → `synced` / `failed`
- 失敗時自動重試，記錄重試次數與錯誤訊息
- 每個使用者 + `temp_id` 組合唯一，防止重複操作

### 2.9 審計日誌

- 所有重要操作記錄在 `audit_logs` 表
- 記錄欄位：
  - `actor_id` — 操作者
  - `family_id` — 所屬家庭
  - `action` — 操作行為
  - `resource_type` — 資源類型
  - `resource_id` — 資源 ID
  - `metadata` — 額外資訊（JSON）

---

## 3. 資料庫結構

### 3.1 設計原則

- 所有金額以整數 cents（int64）儲存
- 時間欄位使用 `TEXT`，格式為 `datetime('now')`
- 軟刪除使用 `status` 欄位
- 外鍵設定 `ON DELETE CASCADE` 或 `ON DELETE SET NULL`
- 唯一約束使用 `UNIQUE INDEX`

### 3.2 users 表 — 使用者帳號

```sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT,
    google_sub TEXT UNIQUE,
    display_name TEXT NOT NULL,
    locale TEXT NOT NULL DEFAULT 'zh-TW',
    theme TEXT NOT NULL DEFAULT 'default',
    role TEXT NOT NULL DEFAULT 'baby' CHECK (role IN ('giver','baby','admin')),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','invited','disabled')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `email` | TEXT UNIQUE | 登入用 email |
| `password_hash` | TEXT | bcrypt 雜湊密碼（Email 登入用） |
| `google_sub` | TEXT UNIQUE | Google OAuth 使用者識別碼 |
| `display_name` | TEXT | 顯示名稱 |
| `locale` | TEXT | 語系，預設 `zh-TW` |
| `theme` | TEXT | 主題，預設 `default` |
| `role` | TEXT | 系統角色：`giver` / `baby` / `admin` |
| `status` | TEXT | 帳號狀態：`active` / `invited` / `disabled` |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

### 3.3 families 表 — 家庭群組

```sql
CREATE TABLE IF NOT EXISTS families (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'TWD',
    timezone TEXT NOT NULL DEFAULT 'Asia/Taipei',
    created_by INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_families_name_owner ON families(name, created_by);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `name` | TEXT | 家庭名稱 |
| `currency` | TEXT | 幣別，預設 `TWD` |
| `timezone` | TEXT | 時區，預設 `Asia/Taipei` |
| `created_by` | INTEGER FK | 建立者（關聯 users.id） |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

**索引**：`(name, created_by)` 唯一索引

### 3.4 family_members 表 — 家庭成員

```sql
CREATE TABLE IF NOT EXISTS family_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    family_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    family_role TEXT NOT NULL CHECK (family_role IN ('giver','baby','viewer')),
    invited_by INTEGER,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','pending','removed')),
    joined_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (family_id, user_id),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (invited_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_family_members_family ON family_members(family_id);
CREATE INDEX IF NOT EXISTS idx_family_members_user ON family_members(user_id);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `family_id` | INTEGER FK | 所屬家庭 |
| `user_id` | INTEGER FK | 使用者 |
| `family_role` | TEXT | 家庭角色：`giver` / `baby` / `viewer` |
| `invited_by` | INTEGER FK | 邀請者 |
| `status` | TEXT | 成員狀態：`active` / `pending` / `removed` |
| `joined_at` | TEXT | 加入時間 |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

**唯一約束**：`(family_id, user_id)`
**索引**：`family_id`、`user_id`

### 3.5 wallets 表 — 錢包

```sql
CREATE TABLE IF NOT EXISTS wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    family_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('cash','bank','card','virtual')),
    balance_cents INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'TWD',
    warning_threshold_cents INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','archived')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    UNIQUE (family_id, name)
);

CREATE INDEX IF NOT EXISTS idx_wallets_family ON wallets(family_id);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `family_id` | INTEGER FK | 所屬家庭 |
| `name` | TEXT | 錢包名稱 |
| `type` | TEXT | 類型：`cash` / `bank` / `card` / `virtual` |
| `balance_cents` | INTEGER | 餘額（單位：分） |
| `currency` | TEXT | 幣別，預設 `TWD` |
| `warning_threshold_cents` | INTEGER | 低餘額警告閾值（分） |
| `status` | TEXT | 狀態：`active` / `archived` |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

**唯一約束**：`(family_id, name)`
**索引**：`family_id`

### 3.6 allowances 表 — 定期津貼

```sql
CREATE TABLE IF NOT EXISTS allowances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    family_id INTEGER NOT NULL,
    giver_member_id INTEGER NOT NULL,
    receiver_member_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    amount_cents INTEGER NOT NULL,
    frequency TEXT NOT NULL CHECK (frequency IN ('daily','weekly','biweekly','monthly','custom')),
    interval_count INTEGER NOT NULL DEFAULT 1,
    next_run_at TEXT,
    last_run_at TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','paused','archived')),
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (giver_member_id) REFERENCES family_members(id) ON DELETE CASCADE,
    FOREIGN KEY (receiver_member_id) REFERENCES family_members(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_allowances_family ON allowances(family_id);
CREATE INDEX IF NOT EXISTS idx_allowances_wallet ON allowances(wallet_id);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `family_id` | INTEGER FK | 所屬家庭 |
| `giver_member_id` | INTEGER FK | 發放者（family_members.id） |
| `receiver_member_id` | INTEGER FK | 接收者（family_members.id） |
| `wallet_id` | INTEGER FK | 關聯錢包 |
| `amount_cents` | INTEGER | 發放金額（分） |
| `frequency` | TEXT | 頻率：`daily` / `weekly` / `biweekly` / `monthly` / `custom` |
| `interval_count` | INTEGER | 間隔數（搭配 `custom` 使用），預設 1 |
| `next_run_at` | TEXT | 下次執行時間 |
| `last_run_at` | TEXT | 上次執行時間 |
| `status` | TEXT | 狀態：`active` / `paused` / `archived` |
| `notes` | TEXT | 備註 |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

**索引**：`family_id`、`wallet_id`

### 3.7 requests 表 — 請款申請

```sql
CREATE TABLE IF NOT EXISTS requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    family_id INTEGER NOT NULL,
    requester_member_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    amount_cents INTEGER NOT NULL,
    category TEXT,
    notes TEXT,
    attachment_url TEXT,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','pending','approved','rejected','cancelled')),
    decision_by_member_id INTEGER,
    decision_at TEXT,
    rejection_reason TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (requester_member_id) REFERENCES family_members(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE,
    FOREIGN KEY (decision_by_member_id) REFERENCES family_members(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_requests_status ON requests(status);
CREATE INDEX IF NOT EXISTS idx_requests_wallet ON requests(wallet_id);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `family_id` | INTEGER FK | 所屬家庭 |
| `requester_member_id` | INTEGER FK | 請款者（family_members.id） |
| `wallet_id` | INTEGER FK | 目標錢包 |
| `amount_cents` | INTEGER | 請款金額（分） |
| `category` | TEXT | 分類標籤 |
| `notes` | TEXT | 備註 |
| `attachment_url` | TEXT | 收據/附件圖片 URL |
| `status` | TEXT | 狀態：`draft` / `pending` / `approved` / `rejected` / `cancelled` |
| `decision_by_member_id` | INTEGER FK | 審核者 |
| `decision_at` | TEXT | 審核時間 |
| `rejection_reason` | TEXT | 駁回原因 |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

**索引**：`status`、`wallet_id`

### 3.8 transactions 表 — 交易紀錄

```sql
CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    family_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('credit','debit')),
    amount_cents INTEGER NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('allowance','request','manual','adjustment')),
    source_id INTEGER,
    category TEXT,
    occurred_at TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_transactions_wallet ON transactions(wallet_id);
CREATE INDEX IF NOT EXISTS idx_transactions_family_date ON transactions(family_id, occurred_at);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `family_id` | INTEGER FK | 所屬家庭 |
| `wallet_id` | INTEGER FK | 關聯錢包 |
| `type` | TEXT | 類型：`credit`（入帳）/ `debit`（出帳） |
| `amount_cents` | INTEGER | 金額（分） |
| `source_type` | TEXT | 來源：`allowance` / `request` / `manual` / `adjustment` |
| `source_id` | INTEGER | 來源資料的 ID |
| `category` | TEXT | 分類標籤 |
| `occurred_at` | TEXT | 實際發生時間 |
| `notes` | TEXT | 備註 |
| `created_at` | TEXT | 建立時間 |

**注意**：此表無 `updated_at`，交易紀錄為不可變更的帳本。
**索引**：`wallet_id`、`(family_id, occurred_at)` 複合索引

### 3.9 notifications 表 — 通知佇列

```sql
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    payload TEXT,
    delivery_status TEXT NOT NULL DEFAULT 'pending' CHECK (delivery_status IN ('pending','sent','failed','read')),
    read_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_notifications_user ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_status ON notifications(delivery_status);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `user_id` | INTEGER FK | 目標使用者 |
| `event_type` | TEXT | 事件類型 |
| `payload` | TEXT | 通知內容（JSON） |
| `delivery_status` | TEXT | 發送狀態：`pending` / `sent` / `failed` / `read` |
| `read_at` | TEXT | 已讀時間 |
| `created_at` | TEXT | 建立時間 |

**索引**：`user_id`、`delivery_status`

### 3.10 sync_queue 表 — 離線同步佇列

```sql
CREATE TABLE IF NOT EXISTS sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    operation_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    temp_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','synced','failed')),
    retries INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (user_id, temp_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `device_id` | TEXT | 裝置識別碼 |
| `user_id` | INTEGER FK | 使用者 |
| `operation_type` | TEXT | 操作類型 |
| `payload` | TEXT | 操作內容（JSON） |
| `temp_id` | TEXT | 本地臨時 ID |
| `status` | TEXT | 狀態：`pending` / `synced` / `failed` |
| `retries` | INTEGER | 重試次數，預設 0 |
| `last_error` | TEXT | 最後一次錯誤訊息 |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

**唯一約束**：`(user_id, temp_id)`

### 3.11 audit_logs 表 — 審計日誌

```sql
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    actor_id INTEGER,
    family_id INTEGER,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (actor_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_logs_family ON audit_logs(family_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
```

| 欄位 | 類型 | 說明 |
|------|------|------|
| `id` | INTEGER PK | 自動遞增主鍵 |
| `actor_id` | INTEGER FK | 操作者（可為 NULL，系統操作時） |
| `family_id` | INTEGER FK | 所屬家庭（可為 NULL） |
| `action` | TEXT | 操作行為 |
| `resource_type` | TEXT | 資源類型 |
| `resource_id` | TEXT | 資源 ID |
| `metadata` | TEXT | 額外資訊（JSON） |
| `created_at` | TEXT | 建立時間 |

**索引**：`family_id`、`action`

### 3.12 種子資料

```sql
-- 預設 Giver 使用者
INSERT OR IGNORE INTO users (email, password_hash, display_name, role)
VALUES ('giver@example.com', 'bcrypt-placeholder', 'Primary Giver', 'giver');

-- 預設 Demo 家庭
INSERT OR IGNORE INTO families (name, created_by)
SELECT 'Demo Family', id FROM users WHERE email = 'giver@example.com';

-- 將 Giver 加入 Demo 家庭
INSERT OR IGNORE INTO family_members (family_id, user_id, family_role, status, joined_at)
SELECT f.id, u.id, 'giver', 'active', datetime('now')
FROM families f JOIN users u ON u.id = f.created_by;
```

### 3.13 SQLite 初始設定

```sql
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
PRAGMA user_version = 1;
```

---

## 4. API 設計規格

### 4.1 路徑規範

```
/api/v1/{resource}/{id?}/{action?}
```

範例：
- `POST /api/v1/auth/login` — 登入
- `GET /api/v1/families/123/wallets` — 取得家庭錢包列表
- `PATCH /api/v1/requests/456/approve` — 核准請款

### 4.2 HTTP 方法

| 方法 | 用途 |
|------|------|
| `GET` | 查詢（無副作用） |
| `POST` | 建立新資源 |
| `PATCH` | 部分更新 |
| `DELETE` | 刪除（軟刪除） |

### 4.3 狀態碼

| 狀態碼 | 說明 |
|--------|------|
| `200` | 成功 |
| `201` | 建立成功 |
| `400` | 客戶端錯誤（輸入驗證失敗） |
| `401` | 未授權（未登入或 token 失效） |
| `403` | 無權限（已登入但無操作權限） |
| `404` | 資源不存在 |
| `500` | 伺服器內部錯誤 |

### 4.4 統一回應格式

```json
{
    "data": { ... },
    "error": null
}
```

錯誤回應：
```json
{
    "data": null,
    "error": {
        "code": "WALLET_NOT_FOUND",
        "message": "找不到指定的錢包"
    }
}
```

### 4.5 錯誤格式

```go
type APIError struct {
    Code    string `json:"code"`
    Message string `json:"message"`
}
```

### 4.6 認證機制

- 登入成功回傳 session token
- 後續請求在 Header 帶入 token
- Token 使用 `CACAO_SESSION_SECRET` 簽署

### 4.7 環境變數

| 變數名稱 | 說明 | 預設值 |
|----------|------|--------|
| `CACAO_ENV` | 環境 | `development` |
| `CACAO_API_PORT` | API 監聽埠 | `8080` |
| `CACAO_DB_HOST` | 資料庫主機 | — |
| `CACAO_DB_NAME` | 資料庫名稱 | — |
| `CACAO_ADMIN_USERNAME` | 預設管理員帳號 | — |
| `CACAO_ADMIN_PASSWORD` | 預設管理員密碼 | — |
| `CACAO_SESSION_SECRET` | Token 簽署密鑰 | — |
| `CACAO_ALLOW_CORS` | 是否啟用 CORS | `false` |

### 4.8 API 端點清單（規劃中）

#### 認證

| 方法 | 路徑 | 說明 |
|------|------|------|
| `GET` | `/health` | 健康檢查（無需認證） |
| `POST` | `/api/v1/auth/login` | Email/密碼登入（無需認證） |
| `POST` | `/api/v1/auth/register` | 註冊帳號 |
| `POST` | `/api/v1/auth/google` | Google OAuth 登入 |

#### 家庭

| 方法 | 路徑 | 說明 |
|------|------|------|
| `POST` | `/api/v1/families` | 建立家庭 |
| `GET` | `/api/v1/families` | 取得使用者的家庭列表 |
| `GET` | `/api/v1/families/:id` | 取得家庭詳情 |
| `PATCH` | `/api/v1/families/:id` | 更新家庭資訊 |
| `POST` | `/api/v1/families/:id/members` | 邀請成員 |
| `DELETE` | `/api/v1/families/:id/members/:memberId` | 移除成員 |

#### 錢包

| 方法 | 路徑 | 說明 |
|------|------|------|
| `POST` | `/api/v1/families/:familyId/wallets` | 建立錢包 |
| `GET` | `/api/v1/families/:familyId/wallets` | 取得錢包列表 |
| `GET` | `/api/v1/wallets/:id` | 取得錢包詳情 |
| `PATCH` | `/api/v1/wallets/:id` | 更新錢包 |
| `PATCH` | `/api/v1/wallets/:id/archive` | 封存錢包 |

#### 津貼

| 方法 | 路徑 | 說明 |
|------|------|------|
| `POST` | `/api/v1/families/:familyId/allowances` | 建立津貼 |
| `GET` | `/api/v1/families/:familyId/allowances` | 取得津貼列表 |
| `PATCH` | `/api/v1/allowances/:id` | 更新津貼 |
| `PATCH` | `/api/v1/allowances/:id/pause` | 暫停津貼 |
| `PATCH` | `/api/v1/allowances/:id/resume` | 恢復津貼 |

#### 請款

| 方法 | 路徑 | 說明 |
|------|------|------|
| `POST` | `/api/v1/families/:familyId/requests` | 建立請款 |
| `GET` | `/api/v1/families/:familyId/requests` | 取得請款列表 |
| `GET` | `/api/v1/requests/:id` | 取得請款詳情 |
| `PATCH` | `/api/v1/requests/:id` | 更新請款（草稿中） |
| `PATCH` | `/api/v1/requests/:id/submit` | 送出請款 |
| `PATCH` | `/api/v1/requests/:id/approve` | 核准請款 |
| `PATCH` | `/api/v1/requests/:id/reject` | 駁回請款 |
| `PATCH` | `/api/v1/requests/:id/cancel` | 取消請款 |

#### 交易

| 方法 | 路徑 | 說明 |
|------|------|------|
| `GET` | `/api/v1/families/:familyId/transactions` | 取得交易紀錄 |
| `GET` | `/api/v1/wallets/:walletId/transactions` | 取得錢包交易紀錄 |
| `POST` | `/api/v1/wallets/:walletId/transactions` | 手動新增交易 |

#### 通知

| 方法 | 路徑 | 說明 |
|------|------|------|
| `GET` | `/api/v1/notifications` | 取得通知列表 |
| `PATCH` | `/api/v1/notifications/:id/read` | 標記已讀 |

---

## 5. 後端架構

### 5.1 技術堆疊

| 項目 | 技術 |
|------|------|
| 語言 | Go 1.24+ |
| HTTP 框架 | Gin |
| ORM | GORM |
| 資料庫（開發） | SQLite |
| 資料庫（生產） | MySQL 8.0 |
| Migration | Goose / golang-migrate |
| 快取（預留） | Redis |
| DI（預留） | Wire |

### 5.2 分層架構（Clean Architecture）

```
server/
├── cmd/
│   ├── api/main.go          # API 服務入口
│   └── jobs/main.go         # 背景任務入口
├── internal/
│   ├── platform/            # 基礎設施層
│   │   ├── config/          # 環境變數與配置
│   │   ├── router/          # Gin router 設定
│   │   ├── server/          # HTTP server 初始化
│   │   └── logger/          # 結構化日誌
│   ├── domain/              # 核心業務邏輯層（無外部依賴）
│   │   ├── auth/            # 認證邏輯
│   │   ├── families/        # 家庭管理
│   │   ├── wallets/         # 錢包管理
│   │   ├── allowances/      # 津貼管理
│   │   ├── requests/        # 請款管理
│   │   ├── transactions/    # 交易紀錄
│   │   ├── notifications/   # 通知系統
│   │   └── sync/            # 離線同步
│   ├── api/                 # HTTP handlers 層
│   │   └── auth/            # 認證 API handler
│   └── jobs/                # 背景任務
│       └── runner.go        # 任務排程器
├── migrations/              # 資料庫 migration 檔案
├── configs/                 # 配置檔案範本
└── go.mod
```

### 5.3 API 服務入口

```go
func main() {
    cfg := config.Load()
    authenticator := domainauth.NewAuthenticator(
        cfg.Auth.AdminUsername,
        cfg.Auth.AdminPassword,
        cfg.Auth.SessionSecret,
        cfg.Auth.DisplayName,
    )
    authHandler := authapi.NewHandler(authenticator)
    eng := router.New(cfg, authHandler)
    srv := server.New(cfg, eng)
    if err := srv.Run(context.Background()); err != nil {
        slog.Error("api server exited", "error", err)
    }
}
```

### 5.4 背景任務入口

```go
func main() {
    ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
    defer stop()
    cfg := config.Load()
    if err := jobs.Run(ctx, cfg); err != nil {
        slog.Error("jobs runner exited", "error", err)
    }
}
```

背景任務職責：
- **Allowance Runner**：根據 `next_run_at` 自動執行津貼發放
- **Notification Resender**：重新發送失敗的通知

### 5.5 命名慣例

| 類型 | 規則 | 範例 |
|------|------|------|
| Package | 小寫單字 | `auth`、`wallets` |
| Interface | `-er` 結尾 | `Authenticator`、`WalletService` |
| 私有函數 | 小寫開頭 | `validatePassword` |
| 公開函數 | 大寫開頭 | `NewHandler` |

### 5.6 效能目標

- API 回應時間 < 200ms（p95）
- 資料庫查詢 < 50ms（p95）
- 背景任務不阻塞主服務

---

## 6. 前端架構

### 6.1 技術堆疊（原架構）

| 項目 | 技術 |
|------|------|
| 框架 | Expo SDK 54 / React Native 0.81.5 |
| 路由 | Expo Router 6.0.15 |
| 語言 | TypeScript 5.9.2 |
| React | 19.1.0 |
| 狀態管理（Server） | React Query |
| 狀態管理（Client） | Zustand |
| 本地資料庫 | WatermelonDB + SQLite |
| 國際化 | i18n（zh-TW、en） |

### 6.2 計畫中的目錄結構

```
apps/mobile/
├── app/                  # Expo Router 路由
│   ├── _layout.tsx       # Root layout
│   ├── index.tsx         # 首頁（重定向邏輯）
│   ├── (tabs)/           # Tab 導航
│   │   ├── index.tsx     # Dashboard
│   │   ├── requests.tsx  # 請款列表
│   │   └── settings.tsx  # 設定頁
│   └── auth/
│       └── login.tsx     # 登入頁
├── features/             # 功能模組
│   ├── auth/             # 認證（AuthProvider）
│   ├── requests/         # 請款功能
│   └── dashboard/        # 儀表板
├── hooks/                # 自定義 hooks
├── services/             # API 服務層
├── stores/               # Zustand stores
├── theme/                # 主題系統
│   ├── index.ts
│   ├── theme-provider.tsx
│   └── themes.ts
├── i18n/                 # 多語言翻譯
│   ├── zh-TW.json
│   └── en.json
└── assets/               # 靜態資源
```

### 6.3 主題系統

支援三種主題：

| 主題 | 說明 |
|------|------|
| Light | 預設淺色主題 |
| Dark | 深色主題 |
| High Contrast | 高對比度（無障礙） |

主題使用方式：
```tsx
const { colors, spacing, typography } = useTheme();
```

### 6.4 多語言支援

- 預設語言：繁體中文（`zh-TW`）
- 支援語言：英文（`en`）
- 翻譯檔案格式：JSON
- 使用 `useTranslation` hook 取用翻譯

### 6.5 APP 設定

| 項目 | 值 |
|------|------|
| App 名稱 | Cacao |
| Bundle ID | `com.cacao.app` |
| URL Scheme | `cacao` |
| 螢幕方向 | portrait（直式） |
| 支援平板 | 是（iOS） |
| UI 風格 | 跟隨系統（automatic） |

### 6.6 效能目標

- App 啟動時間 < 3 秒
- 頁面切換無卡頓
- 列表滾動流暢（60 FPS）

### 6.7 無障礙性要求

- 所有互動元素需有 `accessibilityLabel`
- 支援 VoiceOver（iOS）/ TalkBack（Android）
- 對比度符合 WCAG AA 標準
- 可觸碰區域至少 44x44 pt

---

## 7. 列舉值與常數

### 7.1 使用者角色（User Role）

| 值 | 說明 |
|------|------|
| `giver` | 家長/給予者 |
| `baby` | 子女/接收者 |
| `admin` | 系統管理員 |

### 7.2 家庭角色（Family Role）

| 值 | 說明 |
|------|------|
| `giver` | 可發放津貼、審核請款 |
| `baby` | 可接收津貼、提出請款 |
| `viewer` | 唯讀檢視 |

### 7.3 使用者狀態（User Status）

| 值 | 說明 |
|------|------|
| `active` | 正常使用中 |
| `invited` | 已邀請，尚未接受 |
| `disabled` | 已停用 |

### 7.4 家庭成員狀態（Family Member Status）

| 值 | 說明 |
|------|------|
| `active` | 活躍成員 |
| `pending` | 邀請待接受 |
| `removed` | 已移除 |

### 7.5 錢包類型（Wallet Type）

| 值 | 說明 |
|------|------|
| `cash` | 現金 |
| `bank` | 銀行帳戶 |
| `card` | 信用卡/金融卡 |
| `virtual` | 虛擬錢包 |

### 7.6 錢包狀態（Wallet Status）

| 值 | 說明 |
|------|------|
| `active` | 使用中 |
| `archived` | 已封存 |

### 7.7 津貼頻率（Allowance Frequency）

| 值 | 說明 |
|------|------|
| `daily` | 每日 |
| `weekly` | 每週 |
| `biweekly` | 每兩週 |
| `monthly` | 每月 |
| `custom` | 自訂（搭配 `interval_count`） |

### 7.8 津貼狀態（Allowance Status）

| 值 | 說明 |
|------|------|
| `active` | 執行中 |
| `paused` | 已暫停 |
| `archived` | 已封存 |

### 7.9 請款狀態（Request Status）

| 值 | 說明 |
|------|------|
| `draft` | 草稿 |
| `pending` | 待審核 |
| `approved` | 已核准 |
| `rejected` | 已駁回 |
| `cancelled` | 已取消 |

### 7.10 交易類型（Transaction Type）

| 值 | 說明 |
|------|------|
| `credit` | 入帳（錢包餘額增加） |
| `debit` | 出帳（錢包餘額減少） |

### 7.11 交易來源（Transaction Source Type）

| 值 | 說明 |
|------|------|
| `allowance` | 定期津貼發放 |
| `request` | 請款核准 |
| `manual` | 手動記帳 |
| `adjustment` | 系統調整 |

### 7.12 通知狀態（Notification Delivery Status）

| 值 | 說明 |
|------|------|
| `pending` | 待發送 |
| `sent` | 已發送 |
| `failed` | 發送失敗 |
| `read` | 已讀 |

### 7.13 同步狀態（Sync Queue Status）

| 值 | 說明 |
|------|------|
| `pending` | 待同步 |
| `synced` | 已同步 |
| `failed` | 同步失敗 |

---

## 8. 業務規則

### 8.1 金額處理

- 所有金額以 **cents（分）** 為單位，使用 **整數（int64）** 儲存
- 例如：NT$100 儲存為 `10000`
- 前端顯示時除以 100 並格式化
- 避免浮點數精度問題

### 8.2 錢包餘額變更

- 涉及錢包餘額變更的操作 **必須使用資料庫 transaction**
- 防止並發操作導致餘額不一致
- 餘額變更後同時寫入 `transactions` 表

### 8.3 請款審核流程狀態機

```
                ┌──────────────────────────────────┐
                │                                  │
draft ──→ pending ──→ approved                     │
                │                                  │
                ├──→ rejected                      │
                │                                  │
                └──→ cancelled ←───────────────────┘
                     (baby 可在 draft/pending 時取消)
```

規則：
- 只有 `draft` 狀態可以編輯請款內容
- 只有 `draft` 狀態可以送出（→ `pending`）
- 只有 `pending` 狀態可以被核准或駁回
- `draft` 和 `pending` 狀態可以取消
- 駁回時 **必須** 提供 `rejection_reason`
- 核准時自動：
  1. 從錢包扣除金額（建立 `debit` 交易）
  2. 記錄審核者（`decision_by_member_id`）與時間（`decision_at`）
  3. 發送通知給請款者

### 8.4 津貼自動發放

- 背景任務定期檢查 `allowances` 表
- 條件：`status = 'active'` 且 `next_run_at <= NOW()`
- 執行流程：
  1. 在目標錢包新增 `credit` 交易
  2. 更新錢包餘額
  3. 更新 `last_run_at` 為當前時間
  4. 根據 `frequency` 和 `interval_count` 計算並更新 `next_run_at`
  5. 發送通知給接收者

### 8.5 離線同步

- 離線時的操作存入 `sync_queue`
- 恢復連線後依序同步
- 同步失敗自動重試，記錄 `retries` 次數
- `temp_id` 用於樂觀更新：本地先生成臨時 ID，同步成功後替換為伺服器 ID
- 相同 `(user_id, temp_id)` 不會重複提交

### 8.6 授權規則

- 使用者只能存取自己所屬家庭的資料
- 操作前必須檢查 `family_id` 和 `user_id` 的對應關係
- Giver 可以：建立/管理錢包、設定/管理津貼、審核請款
- Baby 可以：查看餘額、提出請款
- Viewer 只能：檢視家庭資料（唯讀）

### 8.7 輸入驗證

- 所有使用者輸入必須驗證長度、格式、範圍
- Email 格式驗證
- 金額必須為正整數
- 字串長度限制

---

## 9. 基礎設施

### 9.1 Docker Compose 服務

```yaml
services:
  cacao_mysql_3307:
    image: mysql:lts
    ports: ["3307:3306"]
    environment:
      MYSQL_ROOT_PASSWORD: mysettingpassword
      MYSQL_DATABASE: TemplateDB
    volumes: mysql_3307_data:/var/lib/mysql

  cacao_api:
    ports: ["4545:80"]
    env_file: [.env.production, .env]
    environment:
      DB_USER: root
      DB_PASS: mysettingpassword
      DB_PORT: 3307
      DB_HOST: cacao_mysql_3307
    depends_on: [cacao_mysql_3307]
```

### 9.2 Task 自動化腳本

| 指令 | 說明 |
|------|------|
| `task server:test` | 執行 Go 單元測試 |
| `task mobile:lint` | 執行 ESLint |
| `task mobile:typecheck` | 執行 TypeScript 型別檢查 |
| `task compose:config` | 驗證 docker-compose 配置 |
| `task health` | 執行所有檢查（上述四項） |

### 9.3 MySQL 連線設定

```
DSN: {user}:{pass}@tcp({host}:{port})/{name}?parseTime=true&charset=utf8mb4&loc=Local
```

- ORM：GORM
- 字元集：utf8mb4
- 時區：本地時間
- 連線池：由 GORM 預設管理

### 9.4 開發環境 vs 生產環境

| 項目 | 開發環境 | 生產環境 |
|------|---------|---------|
| 資料庫 | SQLite | MySQL 8.0 |
| CORS | 啟用 | 限制特定 origin |
| 日誌等級 | DEBUG | INFO |
| API Port | 8080 | 由環境變數指定 |
