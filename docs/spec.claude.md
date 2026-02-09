# Cacao 產品規格與技術架構文件

> 本文件為 Cacao 專案的完整規格與技術架構設計。
> 基於 `docs/spec.md` 的原始規格完善，並以 **Tauri v2 + React + Tailwind CSS + shadcn/ui** 重新規劃技術實作。
> 最後更新：2026-02-09

---

## 目錄

### Part A — 產品規格

1. [專案概述](#1-專案概述)
2. [使用者旅程](#2-使用者旅程)
3. [頁面設計](#3-頁面設計)
4. [核心功能規格](#4-核心功能規格)
5. [資料庫結構](#5-資料庫結構)
6. [列舉值與常數](#6-列舉值與常數)
7. [業務規則](#7-業務規則)

### Part B — 技術架構

8. [技術堆疊總覽](#8-技術堆疊總覽)
9. [專案結構](#9-專案結構)
10. [Rust 後端架構（Tauri）](#10-rust-後端架構tauri)
11. [React 前端架構](#11-react-前端架構)
12. [資料流架構](#12-資料流架構)
13. [遠端 API 伺服器](#13-遠端-api-伺服器)
14. [建置與部署](#14-建置與部署)

---

# Part A — 產品規格

---

## 1. 專案概述

### 1.1 產品名稱

**Cacao** — 家庭零用金管理平台

### 1.2 產品願景

讓每個家庭都能透過簡單的數位工具，建立透明、有教育意義的零用金管理機制。家長輕鬆發放與管控，子女學習理財與自主消費。

### 1.3 產品目標

- 定期自動發放零用金（津貼）
- 子女提出請款、家長即時審核
- 多錢包管理與完整交易紀錄
- 跨裝置同步（家長手機 ↔ 子女手機）
- 離線可用、連線後自動同步

### 1.4 核心價值

| 價值 | 說明 |
|------|------|
| 簡單易用 | 家長與子女都能快速上手 |
| 透明記帳 | 所有金錢異動都有完整紀錄 |
| 教育理財 | 讓子女從零用金管理中學習金錢觀念 |
| 即時溝通 | 請款審核流程即時通知雙方 |
| 離線可用 | 無網路環境也能查看資料與記帳 |

### 1.5 使用者角色

| 角色 | 英文代號 | 說明 |
|------|---------|------|
| 家長/給予者 | `giver` | 建立家庭、設定津貼、審核請款、管理錢包 |
| 子女/寶寶 | `baby` | 查看餘額、提出請款、查看津貼、學習理財 |
| 管理員 | `admin` | 系統管理（預留角色） |

### 1.6 家庭角色（Family Role）

| 家庭角色 | 說明 |
|---------|------|
| `giver` | 可發放津貼、建立錢包、審核請款 |
| `baby` | 可接收津貼、提出請款 |
| `viewer` | 唯讀檢視家庭資料 |

### 1.7 目標平台

| 平台 | 技術 |
|------|------|
| Android | Tauri v2 Mobile（WebView） |
| iOS | Tauri v2 Mobile（WebView） |

### 1.8 預設語系與幣別

- 語系：`zh-TW`（繁體中文），支援 `en`（英文）
- 幣別：`TWD`（新台幣）
- 時區：`Asia/Taipei`

---

## 2. 使用者旅程

### 2.1 Giver（家長）旅程

```
首次使用：
  註冊帳號（Email/密碼 或 Google）
  → 建立家庭
  → 建立錢包（如「現金零用金」）
  → 邀請子女加入家庭
  → 設定定期津貼（如每週 100 元）

日常使用：
  開啟 App → Dashboard
  → 查看待審核請款（紅色徽章）
  → 點擊請款 → 查看詳情/收據
  → 核准或駁回（駁回需填寫原因）
  → 查看錢包餘額與交易紀錄

管理操作：
  → 調整津貼金額/頻率
  → 暫停/恢復津貼
  → 新增/封存錢包
  → 手動新增交易（如現金補貼）
  → 查看通知中心
```

### 2.2 Baby（子女）旅程

```
首次使用：
  收到家長邀請
  → 註冊帳號（Email/密碼 或 Google）
  → 加入家庭
  → 自動開始接收津貼

日常使用：
  開啟 App → Dashboard
  → 查看錢包餘額
  → 提出請款
    → 輸入金額、分類、備註
    → 可附加收據圖片
    → 送出請款
  → 等待通知（核准/駁回）
  → 查看交易紀錄

查詢操作：
  → 查看津貼發放歷史
  → 查看請款歷史與狀態
  → 查看通知中心
```

### 2.3 MVP 功能優先級

| 優先級 | 功能 | 說明 |
|--------|------|------|
| P0 | 身份認證 | Email/密碼 登入註冊 |
| P0 | 家庭管理 | 建立家庭、邀請成員 |
| P0 | 錢包管理 | 建立錢包、查看餘額 |
| P0 | 請款流程 | 建立→送出→審核→記帳 |
| P1 | 定期津貼 | 自動發放排程 |
| P1 | 交易紀錄 | 完整帳本查詢 |
| P1 | 通知系統 | App 內通知 |
| P2 | Google OAuth | 第三方登入 |
| P2 | 離線同步 | 離線操作 + 自動同步 |
| P2 | 審計日誌 | 操作追蹤 |
| P3 | 多語言 | 英文支援 |
| P3 | 高對比主題 | 無障礙支援 |

---

## 3. 頁面設計

### 3.1 頁面總覽

| 頁面 | 路徑 | Giver | Baby | 說明 |
|------|------|-------|------|------|
| 登入 | `/auth/login` | ✅ | ✅ | Email/密碼 + Google 登入 |
| 註冊 | `/auth/register` | ✅ | ✅ | 建立新帳號 |
| Dashboard | `/` | ✅ | ✅ | 首頁，依角色顯示不同內容 |
| 錢包列表 | `/wallets` | ✅ | ✅ | 家庭所有錢包 |
| 錢包詳情 | `/wallets/:id` | ✅ | ✅ | 錢包餘額 + 交易紀錄 |
| 津貼管理 | `/allowances` | ✅ | ✅(唯讀) | 津貼列表與設定 |
| 請款列表 | `/requests` | ✅ | ✅ | 所有請款 |
| 建立請款 | `/requests/new` | ❌ | ✅ | Baby 建立新請款 |
| 請款詳情 | `/requests/:id` | ✅ | ✅ | 查看/審核請款 |
| 交易紀錄 | `/transactions` | ✅ | ✅ | 所有交易 |
| 通知中心 | `/notifications` | ✅ | ✅ | 通知列表 |
| 設定 | `/settings` | ✅ | ✅ | 主題/語言/帳號 |
| 家庭管理 | `/family` | ✅ | ❌ | 管理成員（Giver 專屬） |

### 3.2 登入頁 `/auth/login`

**功能**：
- Email + 密碼輸入表單
- 「登入」按鈕
- Google OAuth 登入按鈕
- 「前往註冊」連結

**互動行為**：
- 輸入驗證：Email 格式、密碼非空
- 登入失敗顯示錯誤訊息（帳號不存在、密碼錯誤）
- 登入成功後導向 Dashboard

### 3.3 註冊頁 `/auth/register`

**功能**：
- Email、密碼、確認密碼、顯示名稱輸入
- 「註冊」按鈕
- 「前往登入」連結

**互動行為**：
- 輸入驗證：Email 格式、密碼長度 ≥ 8、密碼一致
- Email 重複顯示錯誤
- 註冊成功後自動登入並導向 Dashboard

### 3.4 Dashboard `/`

#### Giver 版 Dashboard

**顯示內容**：
- 歡迎訊息（顯示名稱）
- 待審核請款數量（醒目徽章）
- 家庭錢包餘額摘要（卡片列表）
- 最近交易紀錄（最新 5 筆）
- 快捷操作：審核請款、手動記帳

#### Baby 版 Dashboard

**顯示內容**：
- 歡迎訊息
- 我的錢包餘額（主要錢包突顯）
- 最近津貼發放紀錄
- 請款狀態摘要（待審核 / 已核准 / 已駁回）
- 快捷操作：建立請款

### 3.5 錢包列表 `/wallets`

**功能**：
- 以卡片形式顯示所有錢包
- 每張卡片：錢包名稱、類型圖示、餘額、幣別
- 低餘額警告提示（餘額 ≤ 閾值時）
- Giver：可新增錢包（FAB 按鈕）
- 點擊卡片進入錢包詳情

**新增錢包對話框**（Giver）：
- 名稱、類型（下拉選擇）、初始餘額、警告閾值
- 驗證：名稱非空、同家庭不重複

### 3.6 錢包詳情 `/wallets/:id`

**功能**：
- 錢包名稱、類型、餘額大字顯示
- 交易紀錄列表（時間倒序）
  - 每筆：金額（綠色入帳/紅色出帳）、來源、分類、時間
- Giver：可手動新增交易、封存錢包
- 篩選功能：依日期範圍、類型（入帳/出帳）、來源

### 3.7 津貼管理 `/allowances`

**功能**：
- 津貼列表（卡片形式）
  - 每張：發放者→接收者、金額、頻率、狀態、下次發放時間
- Giver：可新增、編輯、暫停/恢復、封存津貼
- Baby：唯讀查看自己的津貼

**新增/編輯津貼表單**（Giver）：
- 接收者（下拉選擇家庭 Baby 成員）
- 錢包（下拉選擇）
- 金額
- 頻率（daily / weekly / biweekly / monthly / custom）
- 備註
- 驗證：金額 > 0、接收者必選

### 3.8 請款列表 `/requests`

**功能**：
- 請款列表（依狀態分 Tab：全部 / 待審核 / 已核准 / 已駁回）
- 每筆：金額、分類、狀態標籤（顏色區分）、請款者、時間
- Baby：右下角 FAB「建立請款」按鈕
- 點擊進入請款詳情

### 3.9 建立請款 `/requests/new`

**功能**（Baby 專屬）：
- 錢包選擇（下拉）
- 金額輸入
- 分類選擇（預設分類 + 自訂）
- 備註文字框
- 附件上傳（拍照或從相簿選取）
- 「儲存草稿」與「送出請款」按鈕

**驗證**：
- 金額 > 0
- 錢包必選

### 3.10 請款詳情 `/requests/:id`

**功能**：
- 顯示：金額、分類、備註、附件圖片、狀態、時間
- 狀態時間軸（建立→送出→審核結果）

**Giver 操作**（僅 `pending` 狀態）：
- 「核准」按鈕
- 「駁回」按鈕（展開駁回原因輸入框）

**Baby 操作**：
- `draft` 狀態：可編輯、送出、取消
- `pending` 狀態：可取消
- 已核准/已駁回：唯讀查看
- 已駁回時顯示駁回原因

### 3.11 交易紀錄 `/transactions`

**功能**：
- 交易列表（時間倒序）
- 每筆：金額（入帳綠/出帳紅）、來源類型圖示、分類、備註、時間
- 篩選：日期範圍、錢包、類型、來源
- 月度統計摘要（總入帳 / 總出帳 / 淨變動）

### 3.12 通知中心 `/notifications`

**功能**：
- 通知列表（時間倒序）
- 每筆：事件圖示、摘要文字、時間、已讀/未讀狀態
- 點擊標記已讀並導向相關頁面
- 「全部標記已讀」按鈕
- 未讀通知數量顯示在底部導航圖示上

**通知事件類型**：
- 請款送出（→ Giver）
- 請款核准（→ Baby）
- 請款駁回（→ Baby）
- 津貼發放（→ Baby）
- 低餘額警告（→ Giver）
- 成員加入家庭（→ Giver）

### 3.13 設定頁 `/settings`

**功能**：
- 帳號資訊（顯示名稱、Email）
- 主題切換（Light / Dark / High Contrast）
- 語言切換（繁體中文 / English）
- 登出按鈕

### 3.14 家庭管理 `/family`（Giver 專屬）

**功能**：
- 家庭名稱與設定
- 成員列表（顯示角色、狀態）
- 邀請成員（輸入 Email）
- 移除成員
- 變更成員角色

### 3.15 底部導航（Tab Bar）

| Tab | 圖示 | 路徑 | 說明 |
|-----|------|------|------|
| 首頁 | Home | `/` | Dashboard |
| 錢包 | Wallet | `/wallets` | 錢包列表 |
| 請款 | Receipt | `/requests` | 請款列表 |
| 通知 | Bell | `/notifications` | 通知中心（含未讀徽章） |
| 設定 | Gear | `/settings` | 設定頁 |

---

## 4. 核心功能規格

### 4.1 身份認證

#### 4.1.1 Email + 密碼登入

- 使用者以 email 註冊帳號
- 密碼使用 bcrypt 雜湊儲存
- 本地登入驗證後，產生 JWT token 存入本地
- 遠端同步時使用 JWT 與 API 伺服器認證

#### 4.1.2 Google OAuth 登入

- 支援 Google 帳號登入
- 儲存 `google_sub`（Google 使用者唯一識別碼）
- 首次登入自動建立帳號

#### 4.1.3 認證規則

- 本地操作：驗證本地 SQLite 中的 session 狀態
- 遠端同步：所有 API 請求需攜帶 JWT token
- Token 過期回傳 `401`，需重新登入

### 4.2 家庭管理

- Giver 可建立家庭（`families`）
- 家庭名稱 + 建立者的組合必須唯一
- 可邀請成員加入家庭（`family_members`）
- 每位成員在家庭中有特定角色（giver / baby / viewer）
- 邀請中的成員狀態為 `pending`，接受後為 `active`
- 成員可被移除（狀態改為 `removed`）

### 4.3 錢包管理

- 每個家庭可建立多個錢包（`wallets`）
- 錢包類型：`cash`（現金）、`bank`（銀行帳戶）、`card`（卡片）、`virtual`（虛擬錢包）
- 餘額以整數（cents）儲存，避免浮點數誤差
- 可設定低餘額警告閾值（`warning_threshold_cents`）
- 錢包可封存（`archived`），封存後不可進行新交易
- 同一家庭內錢包名稱唯一

### 4.4 定期津貼

- Giver 可為 Baby 設定定期津貼（`allowances`）
- 支援頻率：`daily` / `weekly` / `biweekly` / `monthly` / `custom`
- 系統根據 `next_run_at` 自動執行發放
- 發放完成後更新 `last_run_at` 並計算下次 `next_run_at`
- 津貼可暫停（`paused`）或封存（`archived`）
- 每筆津貼關聯一個特定錢包

### 4.5 請款流程

Baby 可向 Giver 提出請款申請（`requests`），狀態流轉：

```
draft → pending → approved
                → rejected
                → cancelled (Baby 可在 draft/pending 時取消)
```

- **draft（草稿）**：Baby 尚未送出的請款
- **pending（待審核）**：Baby 送出後等待 Giver 審核
- **approved（核准）**：Giver 核准，自動從錢包扣款並記錄交易
- **rejected（駁回）**：Giver 駁回，必須填寫駁回原因
- **cancelled（取消）**：Baby 自行取消

請款可附帶：金額、分類、備註、附件圖片 URL

### 4.6 交易紀錄

所有金錢異動記錄在 `transactions` 表中，為不可變更的帳本：

- 類型：`credit`（入帳）或 `debit`（出帳）
- 來源類型：`allowance`、`request`、`manual`、`adjustment`
- 每筆交易關聯家庭與錢包
- 記錄實際發生時間（`occurred_at`）

### 4.7 通知系統

- 基於事件驅動的通知佇列
- 通知狀態：`pending` → `sent` → `read` / `failed`
- 通知內容以 JSON 格式儲存
- 失敗的通知由背景任務重新發送

### 4.8 離線同步機制

- 支援離線操作，操作記錄暫存在本地 `sync_queue`
- 記錄：`device_id`、`operation_type`、`payload`（JSON）、`temp_id`
- 同步狀態：`pending` → `synced` / `failed`
- 失敗時自動重試，記錄重試次數與錯誤訊息
- `(user_id, temp_id)` 唯一，防止重複操作

### 4.9 審計日誌

- 所有重要操作記錄在 `audit_logs`
- 記錄：操作者、家庭、行為、資源類型、資源 ID、額外 metadata

---

## 5. 資料庫結構

### 5.1 設計原則

- 所有金額以整數 **cents（int64）** 儲存
- 時間欄位使用 `TEXT`，ISO 8601 格式
- 軟刪除使用 `status` 欄位
- 外鍵設定 `ON DELETE CASCADE` 或 `ON DELETE SET NULL`
- 唯一約束使用 `UNIQUE INDEX`
- 本地 SQLite 啟用 WAL mode 與 foreign_keys

### 5.2 users 表 — 使用者帳號

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
| `password_hash` | TEXT | bcrypt 雜湊密碼 |
| `google_sub` | TEXT UNIQUE | Google OAuth 識別碼 |
| `display_name` | TEXT | 顯示名稱 |
| `locale` | TEXT | 語系，預設 `zh-TW` |
| `theme` | TEXT | 主題，預設 `default` |
| `role` | TEXT | 系統角色：`giver` / `baby` / `admin` |
| `status` | TEXT | 帳號狀態：`active` / `invited` / `disabled` |
| `created_at` | TEXT | 建立時間 |
| `updated_at` | TEXT | 更新時間 |

### 5.3 families 表 — 家庭群組

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
| `created_by` | INTEGER FK → users.id | 建立者 |

**索引**：`(name, created_by)` 唯一索引

### 5.4 family_members 表 — 家庭成員

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

**唯一約束**：`(family_id, user_id)`

### 5.5 wallets 表 — 錢包

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

**唯一約束**：`(family_id, name)`

### 5.6 allowances 表 — 定期津貼

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

### 5.7 requests 表 — 請款申請

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

### 5.8 transactions 表 — 交易紀錄

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

**注意**：此表無 `updated_at`，交易紀錄為不可變更的帳本。

### 5.9 notifications 表 — 通知佇列

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

### 5.10 sync_queue 表 — 離線同步佇列

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

### 5.11 audit_logs 表 — 審計日誌

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

### 5.12 種子資料

```sql
INSERT OR IGNORE INTO users (email, password_hash, display_name, role)
VALUES ('giver@example.com', 'bcrypt-placeholder', 'Primary Giver', 'giver');

INSERT OR IGNORE INTO families (name, created_by)
SELECT 'Demo Family', id FROM users WHERE email = 'giver@example.com';

INSERT OR IGNORE INTO family_members (family_id, user_id, family_role, status, joined_at)
SELECT f.id, u.id, 'giver', 'active', datetime('now')
FROM families f JOIN users u ON u.id = f.created_by;
```

### 5.13 SQLite 初始設定

```sql
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
PRAGMA user_version = 1;
```

---

## 6. 列舉值與常數

### 6.1 使用者角色（User Role）

| 值 | 說明 |
|------|------|
| `giver` | 家長/給予者 |
| `baby` | 子女/接收者 |
| `admin` | 系統管理員 |

### 6.2 家庭角色（Family Role）

| 值 | 說明 |
|------|------|
| `giver` | 可發放津貼、審核請款 |
| `baby` | 可接收津貼、提出請款 |
| `viewer` | 唯讀檢視 |

### 6.3 使用者狀態（User Status）

| 值 | 說明 |
|------|------|
| `active` | 正常使用中 |
| `invited` | 已邀請，尚未接受 |
| `disabled` | 已停用 |

### 6.4 家庭成員狀態（Family Member Status）

| 值 | 說明 |
|------|------|
| `active` | 活躍成員 |
| `pending` | 邀請待接受 |
| `removed` | 已移除 |

### 6.5 錢包類型（Wallet Type）

| 值 | 說明 |
|------|------|
| `cash` | 現金 |
| `bank` | 銀行帳戶 |
| `card` | 信用卡/金融卡 |
| `virtual` | 虛擬錢包 |

### 6.6 錢包狀態（Wallet Status）

| 值 | 說明 |
|------|------|
| `active` | 使用中 |
| `archived` | 已封存 |

### 6.7 津貼頻率（Allowance Frequency）

| 值 | 說明 |
|------|------|
| `daily` | 每日 |
| `weekly` | 每週 |
| `biweekly` | 每兩週 |
| `monthly` | 每月 |
| `custom` | 自訂（搭配 `interval_count`） |

### 6.8 津貼狀態（Allowance Status）

| 值 | 說明 |
|------|------|
| `active` | 執行中 |
| `paused` | 已暫停 |
| `archived` | 已封存 |

### 6.9 請款狀態（Request Status）

| 值 | 說明 |
|------|------|
| `draft` | 草稿 |
| `pending` | 待審核 |
| `approved` | 已核准 |
| `rejected` | 已駁回 |
| `cancelled` | 已取消 |

### 6.10 交易類型（Transaction Type）

| 值 | 說明 |
|------|------|
| `credit` | 入帳（錢包餘額增加） |
| `debit` | 出帳（錢包餘額減少） |

### 6.11 交易來源（Transaction Source Type）

| 值 | 說明 |
|------|------|
| `allowance` | 定期津貼發放 |
| `request` | 請款核准 |
| `manual` | 手動記帳 |
| `adjustment` | 系統調整 |

### 6.12 通知狀態（Notification Delivery Status）

| 值 | 說明 |
|------|------|
| `pending` | 待發送 |
| `sent` | 已發送 |
| `failed` | 發送失敗 |
| `read` | 已讀 |

### 6.13 同步狀態（Sync Queue Status）

| 值 | 說明 |
|------|------|
| `pending` | 待同步 |
| `synced` | 已同步 |
| `failed` | 同步失敗 |

---

## 7. 業務規則

### 7.1 金額處理

- 所有金額以 **cents（分）** 為單位，使用 **整數（i64）** 儲存
- 例如：NT$100 儲存為 `10000`
- 前端顯示時除以 100 並格式化為 `NT$ 100`
- 避免浮點數精度問題

### 7.2 錢包餘額變更

- 涉及錢包餘額變更的操作 **必須使用 SQLite transaction**
- 防止並發操作導致餘額不一致
- 餘額變更後 **同時** 寫入 `transactions` 表
- 核准請款時檢查餘額是否足夠（餘額不足 → 回傳錯誤，不扣款）

### 7.3 請款審核流程狀態機

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
  1. 檢查錢包餘額 ≥ 請款金額（不足則回傳錯誤）
  2. 從錢包扣除金額（建立 `debit` 交易）
  3. 記錄審核者（`decision_by_member_id`）與時間（`decision_at`）
  4. 發送通知給請款者
  5. 寫入審計日誌

### 7.4 津貼自動發放

- 背景任務定期檢查 `allowances` 表
- 條件：`status = 'active'` 且 `next_run_at <= NOW()`
- 執行流程：
  1. 在目標錢包新增 `credit` 交易
  2. 更新錢包餘額
  3. 更新 `last_run_at` 為當前時間
  4. 根據 `frequency` 和 `interval_count` 計算並更新 `next_run_at`
  5. 發送通知給接收者
  6. 寫入審計日誌

### 7.5 離線同步

- 離線時的操作存入本地 `sync_queue`
- 恢復連線後依序同步至遠端 API
- 同步失敗自動重試，記錄 `retries` 次數
- `temp_id` 用於樂觀更新：本地先生成臨時 ID，同步成功後替換為伺服器 ID
- 相同 `(user_id, temp_id)` 不會重複提交
- 同步衝突解決策略：Last Write Wins（伺服器端時間戳為準）

### 7.6 授權規則

- 使用者只能存取自己所屬家庭的資料
- 操作前必須檢查 `family_id` 和 `user_id` 的對應關係
- Giver 可以：建立/管理錢包、設定/管理津貼、審核請款、手動記帳
- Baby 可以：查看餘額、提出請款、查看交易紀錄
- Viewer 只能：檢視家庭資料（唯讀）

### 7.7 輸入驗證

- 所有使用者輸入必須驗證長度、格式、範圍
- Email 格式驗證（RFC 5322）
- 密碼長度 ≥ 8 字元
- 金額必須為正整數
- 顯示名稱長度 1 ~ 50 字元
- 備註長度 ≤ 500 字元

---

# Part B — 技術架構

---

## 8. 技術堆疊總覽

### 8.1 App 端（Tauri v2 Mobile）

| 層級 | 技術 | 說明 |
|------|------|------|
| 原生殼層 | Tauri v2 | Android/iOS 原生容器 + WebView |
| 後端邏輯 | Rust | 業務邏輯、SQLite 操作、同步引擎 |
| 本地資料庫 | SQLite（sqlx） | WAL mode，嵌入式資料庫 |
| 前端框架 | React 19 | UI 元件與狀態管理 |
| 建置工具 | Vite | 前端打包與開發伺服器 |
| 樣式 | Tailwind CSS 4 | Utility-first CSS |
| 元件庫 | shadcn/ui | 高度可客製化的 React 元件 |
| 路由 | React Router v7 | 客戶端路由 |
| Server State | TanStack Query v5 | 非同步資料管理 |
| Client State | Zustand | 輕量級狀態管理 |
| 國際化 | i18next + react-i18next | 多語言支援 |
| 語言 | TypeScript 5.x | 前端型別安全 |

### 8.2 遠端 API 伺服器

| 項目 | 技術 | 說明 |
|------|------|------|
| 語言 | Rust | 與 Tauri 統一技術棧 |
| HTTP 框架 | Axum | 高效能非同步框架 |
| 資料庫 | PostgreSQL | 生產環境關聯式資料庫 |
| ORM | sqlx | 編譯期 SQL 驗證 |
| 認證 | JWT（jsonwebtoken crate） | 無狀態認證 |
| 密碼雜湊 | argon2 | 安全密碼儲存 |

### 8.3 開發工具

| 工具 | 用途 |
|------|------|
| Cargo | Rust 套件管理 |
| npm/pnpm | 前端套件管理 |
| Tauri CLI | 建置與開發指令 |
| SQLx CLI | 資料庫 migration |

---

## 9. 專案結構

```
Cacao/
├── src-tauri/                    # Rust 後端（Tauri）
│   ├── Cargo.toml                # Rust 依賴
│   ├── tauri.conf.json           # Tauri 配置
│   ├── capabilities/             # Tauri 權限設定
│   ├── migrations/               # SQLite migration 檔案
│   │   ├── 001_init.sql
│   │   └── ...
│   └── src/
│       ├── lib.rs                # Tauri 主入口（註冊 commands、state、plugins）
│       ├── commands/             # Tauri IPC commands（按領域分模組）
│       │   ├── mod.rs
│       │   ├── auth.rs           # 認證相關 commands
│       │   ├── family.rs         # 家庭管理 commands
│       │   ├── wallet.rs         # 錢包 commands
│       │   ├── allowance.rs      # 津貼 commands
│       │   ├── request.rs        # 請款 commands
│       │   ├── transaction.rs    # 交易 commands
│       │   └── notification.rs   # 通知 commands
│       ├── db/                   # 資料庫層
│       │   ├── mod.rs
│       │   └── pool.rs           # SQLite 連線池初始化
│       ├── models/               # 資料模型（struct 對應資料表）
│       │   ├── mod.rs
│       │   ├── user.rs
│       │   ├── family.rs
│       │   ├── wallet.rs
│       │   ├── allowance.rs
│       │   ├── request.rs
│       │   ├── transaction.rs
│       │   ├── notification.rs
│       │   └── sync.rs
│       ├── services/             # 業務邏輯層
│       │   ├── mod.rs
│       │   ├── auth_service.rs
│       │   ├── family_service.rs
│       │   ├── wallet_service.rs
│       │   ├── allowance_service.rs
│       │   ├── request_service.rs
│       │   ├── transaction_service.rs
│       │   └── notification_service.rs
│       ├── sync/                 # 遠端同步引擎
│       │   ├── mod.rs
│       │   ├── engine.rs         # 同步排程與協調
│       │   ├── client.rs         # HTTP client（連接遠端 API）
│       │   └── conflict.rs       # 衝突解決策略
│       └── scheduler/            # 背景排程
│           ├── mod.rs
│           └── allowance_runner.rs  # 津貼自動發放
│
├── src/                          # React 前端
│   ├── main.tsx                  # React 入口
│   ├── App.tsx                   # 根元件（Provider 層）
│   ├── app/                     # 路由頁面
│   │   ├── layout.tsx            # 主 Layout（Tab Bar）
│   │   ├── auth/
│   │   │   ├── login.tsx
│   │   │   └── register.tsx
│   │   ├── dashboard.tsx         # Dashboard（首頁）
│   │   ├── wallets/
│   │   │   ├── index.tsx         # 錢包列表
│   │   │   └── [id].tsx          # 錢包詳情
│   │   ├── allowances/
│   │   │   └── index.tsx         # 津貼管理
│   │   ├── requests/
│   │   │   ├── index.tsx         # 請款列表
│   │   │   ├── new.tsx           # 建立請款
│   │   │   └── [id].tsx          # 請款詳情
│   │   ├── transactions/
│   │   │   └── index.tsx         # 交易紀錄
│   │   ├── notifications/
│   │   │   └── index.tsx         # 通知中心
│   │   ├── family/
│   │   │   └── index.tsx         # 家庭管理
│   │   └── settings/
│   │       └── index.tsx         # 設定頁
│   ├── components/               # 共用元件
│   │   ├── ui/                   # shadcn/ui 元件（自動生成）
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── dialog.tsx
│   │   │   ├── input.tsx
│   │   │   ├── select.tsx
│   │   │   ├── tabs.tsx
│   │   │   ├── badge.tsx
│   │   │   ├── toast.tsx
│   │   │   └── ...
│   │   ├── wallet-card.tsx       # 錢包卡片
│   │   ├── request-card.tsx      # 請款卡片
│   │   ├── transaction-item.tsx  # 交易列表項
│   │   ├── notification-item.tsx # 通知列表項
│   │   ├── amount-display.tsx    # 金額顯示（cents → 格式化）
│   │   ├── status-badge.tsx      # 狀態標籤
│   │   └── tab-bar.tsx           # 底部導航
│   ├── features/                 # 功能模組
│   │   ├── auth/
│   │   │   ├── auth-provider.tsx # AuthContext
│   │   │   └── use-auth.ts      # 認證 hook
│   │   └── sync/
│   │       └── sync-status.tsx   # 同步狀態指示器
│   ├── hooks/                    # 自訂 hooks
│   │   ├── use-wallets.ts        # 錢包 CRUD hooks
│   │   ├── use-allowances.ts     # 津貼 CRUD hooks
│   │   ├── use-requests.ts       # 請款 CRUD hooks
│   │   ├── use-transactions.ts   # 交易查詢 hooks
│   │   └── use-notifications.ts  # 通知 hooks
│   ├── lib/                      # 工具函式
│   │   ├── tauri.ts              # Tauri IPC 型別安全封裝
│   │   ├── format.ts             # 金額/日期格式化
│   │   └── constants.ts          # 常數定義
│   ├── stores/                   # Zustand stores
│   │   ├── auth-store.ts         # 認證狀態
│   │   ├── ui-store.ts           # UI 狀態（主題、語言）
│   │   └── sync-store.ts         # 同步狀態
│   ├── i18n/                     # 國際化
│   │   ├── index.ts              # i18next 初始化
│   │   ├── zh-TW.json            # 繁體中文翻譯
│   │   └── en.json               # 英文翻譯
│   └── styles/
│       └── globals.css           # Tailwind 全域樣式 + shadcn/ui CSS variables
│
├── docs/
│   ├── spec.md                   # 原始規格
│   └── spec.claude.md            # 本文件
├── index.html                    # Vite HTML 入口
├── package.json                  # 前端依賴
├── tailwind.config.ts            # Tailwind 配置
├── tsconfig.json                 # TypeScript 配置
├── vite.config.ts                # Vite 配置
└── components.json               # shadcn/ui 配置
```

---

## 10. Rust 後端架構（Tauri）

### 10.1 架構分層

```
Commands（IPC 介面層）
    ↓ 呼叫
Services（業務邏輯層）
    ↓ 操作
Models + SQLite（資料層）
```

- **Commands**：接收前端 `invoke()` 呼叫，驗證參數，呼叫 Service，回傳結果
- **Services**：包含業務邏輯，操作資料庫，處理事務性操作
- **Models**：Rust struct 對應資料表，用於 sqlx 查詢結果反序列化

### 10.2 SQLite 連線池

```rust
// src-tauri/src/db/pool.rs
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn init_pool(app_data_dir: &Path) -> Result<SqlitePool> {
    let db_path = app_data_dir.join("cacao.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // 啟用 WAL mode 與 foreign keys
    sqlx::query("PRAGMA journal_mode = WAL;").execute(&pool).await?;
    sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await?;

    // 執行 migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
```

### 10.3 Tauri Commands 設計

每個領域模組提供一組 `#[tauri::command]` 函式：

#### 認證 Commands

```rust
#[tauri::command]
async fn login(email: String, password: String, pool: State<'_, SqlitePool>) -> Result<User, String>;

#[tauri::command]
async fn register(email: String, password: String, display_name: String, role: String, pool: State<'_, SqlitePool>) -> Result<User, String>;

#[tauri::command]
async fn get_current_user(pool: State<'_, SqlitePool>) -> Result<Option<User>, String>;

#[tauri::command]
async fn logout(pool: State<'_, SqlitePool>) -> Result<(), String>;
```

#### 家庭 Commands

```rust
#[tauri::command]
async fn create_family(name: String, pool: State<'_, SqlitePool>) -> Result<Family, String>;

#[tauri::command]
async fn get_families(pool: State<'_, SqlitePool>) -> Result<Vec<Family>, String>;

#[tauri::command]
async fn invite_member(family_id: i64, email: String, role: String, pool: State<'_, SqlitePool>) -> Result<FamilyMember, String>;

#[tauri::command]
async fn remove_member(family_id: i64, member_id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;
```

#### 錢包 Commands

```rust
#[tauri::command]
async fn create_wallet(family_id: i64, name: String, wallet_type: String, pool: State<'_, SqlitePool>) -> Result<Wallet, String>;

#[tauri::command]
async fn get_wallets(family_id: i64, pool: State<'_, SqlitePool>) -> Result<Vec<Wallet>, String>;

#[tauri::command]
async fn get_wallet(id: i64, pool: State<'_, SqlitePool>) -> Result<Wallet, String>;

#[tauri::command]
async fn archive_wallet(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;
```

#### 津貼 Commands

```rust
#[tauri::command]
async fn create_allowance(params: CreateAllowanceParams, pool: State<'_, SqlitePool>) -> Result<Allowance, String>;

#[tauri::command]
async fn get_allowances(family_id: i64, pool: State<'_, SqlitePool>) -> Result<Vec<Allowance>, String>;

#[tauri::command]
async fn update_allowance(id: i64, params: UpdateAllowanceParams, pool: State<'_, SqlitePool>) -> Result<Allowance, String>;

#[tauri::command]
async fn pause_allowance(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;

#[tauri::command]
async fn resume_allowance(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;
```

#### 請款 Commands

```rust
#[tauri::command]
async fn create_request(params: CreateRequestParams, pool: State<'_, SqlitePool>) -> Result<Request, String>;

#[tauri::command]
async fn get_requests(family_id: i64, status: Option<String>, pool: State<'_, SqlitePool>) -> Result<Vec<Request>, String>;

#[tauri::command]
async fn submit_request(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;

#[tauri::command]
async fn approve_request(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;

#[tauri::command]
async fn reject_request(id: i64, reason: String, pool: State<'_, SqlitePool>) -> Result<(), String>;

#[tauri::command]
async fn cancel_request(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;
```

#### 交易 Commands

```rust
#[tauri::command]
async fn get_transactions(family_id: i64, wallet_id: Option<i64>, pool: State<'_, SqlitePool>) -> Result<Vec<Transaction>, String>;

#[tauri::command]
async fn create_manual_transaction(params: CreateTransactionParams, pool: State<'_, SqlitePool>) -> Result<Transaction, String>;
```

#### 通知 Commands

```rust
#[tauri::command]
async fn get_notifications(pool: State<'_, SqlitePool>) -> Result<Vec<Notification>, String>;

#[tauri::command]
async fn mark_notification_read(id: i64, pool: State<'_, SqlitePool>) -> Result<(), String>;

#[tauri::command]
async fn mark_all_notifications_read(pool: State<'_, SqlitePool>) -> Result<(), String>;
```

### 10.4 業務邏輯層（Services）

以 `request_service.rs`（最複雜的模組）為例：

```rust
pub struct RequestService;

impl RequestService {
    /// 核准請款：在單一 SQLite transaction 中完成
    pub async fn approve(pool: &SqlitePool, request_id: i64, approver_member_id: i64) -> Result<()> {
        let mut tx = pool.begin().await?;

        // 1. 取得請款資料，驗證狀態為 pending
        let request = sqlx::query_as::<_, Request>("SELECT * FROM requests WHERE id = ? AND status = 'pending'")
            .bind(request_id)
            .fetch_one(&mut *tx).await?;

        // 2. 檢查錢包餘額
        let wallet = sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE id = ?")
            .bind(request.wallet_id)
            .fetch_one(&mut *tx).await?;

        if wallet.balance_cents < request.amount_cents {
            return Err("餘額不足".into());
        }

        // 3. 扣除錢包餘額
        sqlx::query("UPDATE wallets SET balance_cents = balance_cents - ?, updated_at = datetime('now') WHERE id = ?")
            .bind(request.amount_cents).bind(request.wallet_id)
            .execute(&mut *tx).await?;

        // 4. 建立交易紀錄
        sqlx::query("INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, source_id, occurred_at) VALUES (?, ?, 'debit', ?, 'request', ?, datetime('now'))")
            .bind(request.family_id).bind(request.wallet_id)
            .bind(request.amount_cents).bind(request_id)
            .execute(&mut *tx).await?;

        // 5. 更新請款狀態
        sqlx::query("UPDATE requests SET status = 'approved', decision_by_member_id = ?, decision_at = datetime('now'), updated_at = datetime('now') WHERE id = ?")
            .bind(approver_member_id).bind(request_id)
            .execute(&mut *tx).await?;

        // 6. 寫入審計日誌
        sqlx::query("INSERT INTO audit_logs (actor_id, family_id, action, resource_type, resource_id) VALUES (?, ?, 'approve', 'request', ?)")
            .bind(approver_member_id).bind(request.family_id).bind(request_id.to_string())
            .execute(&mut *tx).await?;

        tx.commit().await?;

        // 7. 發送通知（transaction 外，允許失敗）
        NotificationService::send(pool, request.requester_member_id, "request_approved", &request_id.to_string()).await.ok();

        Ok(())
    }
}
```

### 10.5 背景排程器

```rust
// src-tauri/src/scheduler/allowance_runner.rs
use tokio::time::{interval, Duration};

pub async fn start(pool: SqlitePool) {
    let mut ticker = interval(Duration::from_secs(60)); // 每分鐘檢查一次

    loop {
        ticker.tick().await;

        // 查詢到期的津貼
        let due = sqlx::query_as::<_, Allowance>(
            "SELECT * FROM allowances WHERE status = 'active' AND next_run_at <= datetime('now')"
        ).fetch_all(&pool).await;

        if let Ok(allowances) = due {
            for allowance in allowances {
                AllowanceService::execute(&pool, &allowance).await.ok();
            }
        }
    }
}
```

### 10.6 同步引擎

```rust
// src-tauri/src/sync/engine.rs
pub struct SyncEngine {
    pool: SqlitePool,
    client: SyncClient,
}

impl SyncEngine {
    /// 啟動同步循環
    pub async fn start(self) {
        loop {
            // 1. 推送本地變更到遠端
            self.push_changes().await;

            // 2. 拉取遠端變更到本地
            self.pull_changes().await;

            // 3. 等待一段時間或網路狀態變化
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }

    /// 推送本地 sync_queue 中的待同步項目
    async fn push_changes(&self) {
        let pending = sqlx::query_as::<_, SyncQueueItem>(
            "SELECT * FROM sync_queue WHERE status = 'pending' ORDER BY created_at ASC"
        ).fetch_all(&self.pool).await.unwrap_or_default();

        for item in pending {
            match self.client.push(&item).await {
                Ok(_) => {
                    sqlx::query("UPDATE sync_queue SET status = 'synced', updated_at = datetime('now') WHERE id = ?")
                        .bind(item.id).execute(&self.pool).await.ok();
                }
                Err(e) => {
                    sqlx::query("UPDATE sync_queue SET status = 'failed', retries = retries + 1, last_error = ?, updated_at = datetime('now') WHERE id = ?")
                        .bind(e.to_string()).bind(item.id).execute(&self.pool).await.ok();
                }
            }
        }
    }

    /// 從遠端拉取新資料
    async fn pull_changes(&self) {
        // 取得最後同步時間戳
        // 向遠端請求 since 該時間戳之後的變更
        // 寫入本地 SQLite
    }
}
```

### 10.7 Rust 依賴（Cargo.toml）

```toml
[dependencies]
tauri = { version = "2", features = ["mobile"] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
bcrypt = "0.16"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
reqwest = { version = "0.12", features = ["json"] }
```

---

## 11. React 前端架構

### 11.1 路由設計

使用 React Router v7，路由結構對應 [3.1 頁面總覽](#31-頁面總覽)：

```tsx
// src/App.tsx
<BrowserRouter>
  <Routes>
    {/* 公開路由 */}
    <Route path="/auth/login" element={<LoginPage />} />
    <Route path="/auth/register" element={<RegisterPage />} />

    {/* 需登入的路由 */}
    <Route element={<AuthGuard />}>
      <Route element={<AppLayout />}>   {/* 含 Tab Bar */}
        <Route path="/" element={<DashboardPage />} />
        <Route path="/wallets" element={<WalletsPage />} />
        <Route path="/wallets/:id" element={<WalletDetailPage />} />
        <Route path="/allowances" element={<AllowancesPage />} />
        <Route path="/requests" element={<RequestsPage />} />
        <Route path="/requests/new" element={<NewRequestPage />} />
        <Route path="/requests/:id" element={<RequestDetailPage />} />
        <Route path="/transactions" element={<TransactionsPage />} />
        <Route path="/notifications" element={<NotificationsPage />} />
        <Route path="/family" element={<FamilyPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Route>
  </Routes>
</BrowserRouter>
```

### 11.2 Tauri IPC 封裝

```typescript
// src/lib/tauri.ts
import { invoke } from '@tauri-apps/api/core';

// ---- 認證 ----
export const auth = {
  login: (email: string, password: string) =>
    invoke<User>('login', { email, password }),
  register: (email: string, password: string, displayName: string, role: string) =>
    invoke<User>('register', { email, password, displayName, role }),
  getCurrentUser: () =>
    invoke<User | null>('get_current_user'),
  logout: () =>
    invoke<void>('logout'),
};

// ---- 錢包 ----
export const wallets = {
  list: (familyId: number) =>
    invoke<Wallet[]>('get_wallets', { familyId }),
  get: (id: number) =>
    invoke<Wallet>('get_wallet', { id }),
  create: (familyId: number, name: string, walletType: string) =>
    invoke<Wallet>('create_wallet', { familyId, name, walletType }),
  archive: (id: number) =>
    invoke<void>('archive_wallet', { id }),
};

// ---- 請款 ----
export const requests = {
  list: (familyId: number, status?: string) =>
    invoke<Request[]>('get_requests', { familyId, status }),
  create: (params: CreateRequestParams) =>
    invoke<Request>('create_request', { params }),
  submit: (id: number) =>
    invoke<void>('submit_request', { id }),
  approve: (id: number) =>
    invoke<void>('approve_request', { id }),
  reject: (id: number, reason: string) =>
    invoke<void>('reject_request', { id, reason }),
  cancel: (id: number) =>
    invoke<void>('cancel_request', { id }),
};

// ... 其他領域依此類推
```

### 11.3 TanStack Query Hooks

```typescript
// src/hooks/use-wallets.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { wallets } from '@/lib/tauri';

export function useWallets(familyId: number) {
  return useQuery({
    queryKey: ['wallets', familyId],
    queryFn: () => wallets.list(familyId),
  });
}

export function useCreateWallet() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (params: { familyId: number; name: string; walletType: string }) =>
      wallets.create(params.familyId, params.name, params.walletType),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: ['wallets', vars.familyId] });
    },
  });
}
```

### 11.4 Zustand Stores

```typescript
// src/stores/auth-store.ts
import { create } from 'zustand';

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  setUser: (user: User | null) => void;
  clear: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  isAuthenticated: false,
  setUser: (user) => set({ user, isAuthenticated: !!user }),
  clear: () => set({ user: null, isAuthenticated: false }),
}));
```

```typescript
// src/stores/ui-store.ts
import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

type Theme = 'light' | 'dark' | 'high-contrast';
type Locale = 'zh-TW' | 'en';

interface UIState {
  theme: Theme;
  locale: Locale;
  setTheme: (theme: Theme) => void;
  setLocale: (locale: Locale) => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      theme: 'light',
      locale: 'zh-TW',
      setTheme: (theme) => set({ theme }),
      setLocale: (locale) => set({ locale }),
    }),
    { name: 'cacao-ui', storage: createJSONStorage(() => localStorage) }
  )
);
```

### 11.5 shadcn/ui 元件清單

本專案使用以下 shadcn/ui 元件：

| 元件 | 用途 |
|------|------|
| `Button` | 所有按鈕 |
| `Card` | 錢包卡片、津貼卡片、Dashboard 摘要 |
| `Dialog` | 新增錢包、新增津貼、駁回原因輸入 |
| `Input` | 文字輸入（Email、密碼、金額、備註） |
| `Label` | 表單標籤 |
| `Select` | 下拉選擇（錢包類型、頻率、成員） |
| `Tabs` | 請款列表分 Tab（全部/待審核/已核准/已駁回） |
| `Badge` | 狀態標籤（核准/駁回/待審核） |
| `Toast` | 操作結果提示 |
| `Avatar` | 使用者頭像 |
| `Separator` | 區塊分隔 |
| `ScrollArea` | 列表滾動區域 |
| `Sheet` | 側邊抽屜（設定面板） |
| `Skeleton` | 載入中骨架屏 |
| `Switch` | 主題切換開關 |
| `DropdownMenu` | 更多操作選單 |
| `AlertDialog` | 確認對話框（刪除/封存） |
| `Form` | 表單驗證（搭配 react-hook-form + zod） |

### 11.6 主題系統

基於 shadcn/ui 的 CSS Variables 主題機制：

```css
/* src/styles/globals.css */
@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 0 0% 3.9%;
    --primary: 24 80% 50%;        /* Cacao 棕色主色調 */
    --primary-foreground: 0 0% 98%;
    /* ... 其他 shadcn/ui variables */
  }

  .dark {
    --background: 0 0% 3.9%;
    --foreground: 0 0% 98%;
    --primary: 24 80% 60%;
    /* ... */
  }

  .high-contrast {
    --background: 0 0% 0%;
    --foreground: 0 0% 100%;
    --primary: 50 100% 50%;
    /* ... */
  }
}
```

### 11.7 國際化

```typescript
// src/i18n/index.ts
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import zhTW from './zh-TW.json';
import en from './en.json';

i18n.use(initReactI18next).init({
  resources: {
    'zh-TW': { translation: zhTW },
    'en': { translation: en },
  },
  lng: 'zh-TW',
  fallbackLng: 'zh-TW',
});
```

翻譯檔案結構：
```json
// src/i18n/zh-TW.json
{
  "common": {
    "save": "儲存",
    "cancel": "取消",
    "confirm": "確認",
    "delete": "刪除",
    "loading": "載入中..."
  },
  "auth": {
    "login": "登入",
    "register": "註冊",
    "email": "電子郵件",
    "password": "密碼",
    "displayName": "顯示名稱"
  },
  "wallet": {
    "title": "錢包",
    "balance": "餘額",
    "create": "建立錢包",
    "archive": "封存錢包"
  },
  "request": {
    "title": "請款",
    "create": "建立請款",
    "approve": "核准",
    "reject": "駁回",
    "cancel": "取消",
    "rejectionReason": "駁回原因"
  }
}
```

### 11.8 前端依賴（package.json）

```json
{
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "react-router-dom": "^7.0.0",
    "@tauri-apps/api": "^2.0.0",
    "@tanstack/react-query": "^5.0.0",
    "zustand": "^5.0.0",
    "i18next": "^24.0.0",
    "react-i18next": "^15.0.0",
    "react-hook-form": "^7.0.0",
    "@hookform/resolvers": "^3.0.0",
    "zod": "^3.0.0",
    "class-variance-authority": "^0.7.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0",
    "lucide-react": "^0.400.0",
    "date-fns": "^4.0.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "typescript": "^5.6.0",
    "vite": "^6.0.0",
    "@vitejs/plugin-react": "^4.0.0",
    "tailwindcss": "^4.0.0",
    "autoprefixer": "^10.0.0",
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0"
  }
}
```

---

## 12. 資料流架構

### 12.1 整體資料流

```
┌─────────────────────────────────────────────┐
│                React UI                      │
│  (TanStack Query + Zustand + shadcn/ui)     │
└─────────────────┬───────────────────────────┘
                  │ invoke()
                  ▼
┌─────────────────────────────────────────────┐
│           Tauri IPC (Commands)               │
│  參數驗證 → 呼叫 Service → 回傳結果          │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────┐
│           Services（業務邏輯）                │
│  請款狀態機 / 津貼發放 / 餘額變更            │
└────────┬──────────────────┬─────────────────┘
         │                  │
         ▼                  ▼
┌─────────────────┐ ┌───────────────────────┐
│   SQLite (本地)  │ │  Sync Engine          │
│   WAL mode      │ │  push/pull changes    │
│   sqlx          │ │  conflict resolution  │
└─────────────────┘ └───────────┬───────────┘
                                │ HTTP (reqwest)
                                ▼
                    ┌───────────────────────┐
                    │  Remote API Server    │
                    │  Rust Axum            │
                    │  PostgreSQL           │
                    └───────────────────────┘
```

### 12.2 讀取資料流程

```
使用者開啟錢包頁面
  → React Router 渲染 WalletsPage
  → useWallets(familyId) hook 觸發
  → TanStack Query 呼叫 wallets.list(familyId)
  → invoke('get_wallets', { familyId })
  → Rust command 收到請求
  → WalletService::list(pool, familyId)
  → sqlx::query("SELECT * FROM wallets WHERE family_id = ?")
  → 回傳 Vec<Wallet>
  → 前端收到資料，渲染錢包卡片列表
```

### 12.3 寫入資料流程（以核准請款為例）

```
Giver 按下「核准」按鈕
  → useMutation 呼叫 requests.approve(id)
  → invoke('approve_request', { id })
  → Rust command 收到請求
  → RequestService::approve(pool, id, approver_id)
  → 開始 SQLite transaction
    → 驗證請款狀態 = pending
    → 檢查錢包餘額 ≥ 金額
    → 扣除錢包餘額
    → 建立 debit 交易
    → 更新請款狀態 → approved
    → 寫入審計日誌
  → 提交 transaction
  → 發送通知給 Baby
  → 將變更寫入 sync_queue
  → 回傳成功
  → 前端 invalidateQueries(['requests', ...])
  → UI 自動刷新
```

### 12.4 離線 → 上線同步流程

```
離線操作：
  使用者建立請款 → 寫入本地 SQLite + sync_queue(status=pending)
  → UI 正常顯示（樂觀更新，使用 temp_id）

恢復連線：
  SyncEngine 偵測到網路連線
  → 讀取 sync_queue 中 status=pending 的項目
  → 依序推送到遠端 API
    → 成功：sync_queue.status → synced
    → 失敗：retries + 1，status → failed
  → 從遠端拉取其他裝置的變更
  → 寫入本地 SQLite
  → 發出事件通知前端刷新
```

---

## 13. 遠端 API 伺服器

### 13.1 技術堆疊

| 項目 | 技術 |
|------|------|
| 語言 | Rust |
| HTTP 框架 | Axum |
| 資料庫 | PostgreSQL |
| ORM | sqlx |
| 認證 | JWT（jsonwebtoken） |
| 密碼雜湊 | argon2 |
| 序列化 | serde / serde_json |

### 13.2 API 端點設計

沿用原始 spec.md 的 API 設計，路徑規範：`/api/v1/{resource}/{id?}/{action?}`

#### 認證

| 方法 | 路徑 | 說明 |
|------|------|------|
| `POST` | `/api/v1/auth/register` | 註冊帳號 |
| `POST` | `/api/v1/auth/login` | Email/密碼登入 |
| `POST` | `/api/v1/auth/google` | Google OAuth 登入 |
| `GET` | `/api/v1/auth/me` | 取得當前使用者 |

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

#### 同步

| 方法 | 路徑 | 說明 |
|------|------|------|
| `POST` | `/api/v1/sync/push` | 推送本地變更 |
| `GET` | `/api/v1/sync/pull?since=<timestamp>` | 拉取遠端變更 |

### 13.3 統一回應格式

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
        "code": "INSUFFICIENT_BALANCE",
        "message": "錢包餘額不足"
    }
}
```

### 13.4 JWT 認證

- 登入成功回傳 JWT token（含 `user_id`、`email`、`role`、過期時間）
- 後續請求在 Header 帶入：`Authorization: Bearer <token>`
- Token 有效期：7 天
- 使用 RS256 或 HS256 簽署

---

## 14. 建置與部署

### 14.1 開發環境需求

| 工具 | 版本 | 用途 |
|------|------|------|
| Rust | 1.77+ | Tauri 後端 |
| Node.js | 20+ | React 前端 |
| Android Studio | Latest | Android SDK + 模擬器 |
| Xcode | 15+ | iOS 開發（需 macOS） |
| Tauri CLI | 2.x | `cargo install tauri-cli` |

### 14.2 開發指令

```bash
# 安裝依賴
npm install

# 啟動開發伺服器（桌面預覽）
cargo tauri dev

# 啟動 Android 開發
cargo tauri android dev

# 啟動 iOS 開發
cargo tauri ios dev
```

### 14.3 Android 建置

```bash
# 初始化 Android 專案（首次）
cargo tauri android init

# 建置 Debug APK
cargo tauri android build --debug

# 建置 Release APK/AAB
cargo tauri android build --release

# 產出位置
# APK: src-tauri/gen/android/app/build/outputs/apk/
# AAB: src-tauri/gen/android/app/build/outputs/bundle/
```

### 14.4 iOS 建置

```bash
# 初始化 iOS 專案（首次，需 macOS）
cargo tauri ios init

# 建置 Debug
cargo tauri ios build --debug

# 建置 Release（需 Apple Developer 帳號）
cargo tauri ios build --release

# 在 Xcode 中開啟專案進行簽署與上傳
open src-tauri/gen/apple/Cacao.xcodeproj
```

### 14.5 遠端 API 部署

```bash
# 建置 API 伺服器
cd server && cargo build --release

# Docker 部署
docker build -t cacao-api .
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host/cacao \
  -e JWT_SECRET=your-secret \
  cacao-api
```

### 14.6 App 設定

| 項目 | 值 |
|------|------|
| App 名稱 | Cacao |
| Bundle ID | `com.cacao.app` |
| 螢幕方向 | portrait（直式） |
| 最低 Android 版本 | API 24（Android 7.0） |
| 最低 iOS 版本 | 14.0 |
