# Cacao 產品規格與技術架構文件（改善版 v2）

> 本文件為 Cacao 專案的完整規格與技術架構設計。
> 架構：**Tauri v2 Mobile + 純本地 SQLite + 同 WiFi P2P 同步**（無雲端伺服器）。
> 最後更新：2026-02-09

---

## 架構決策摘要

| 決策 | 選擇 | 說明 |
|------|------|------|
| 後端 | 無雲端伺服器 | 純本地架構，無需維運伺服器 |
| 認證 | 無帳號系統 | 裝置設定角色（Giver/Baby）即可使用 |
| 同步 | 同 WiFi 區域網路 | mDNS 裝置發現 + 嵌入式 HTTP 同步 |
| 衝突策略 | Giver 裝置優先 | 衝突時以 Giver 端資料為權威 |
| 資料儲存 | 本地 SQLite | 所有資料存放在裝置本地 |
| 家庭建立 | 裝置配對 | 同 WiFi 下輸入配對碼建立家庭關係 |

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
13. [P2P 同步協議](#13-p2p-同步協議)
14. [建置與部署](#14-建置與部署)

### Part C — 營運與品質

15. [安全性考量](#15-安全性考量)
16. [測試策略](#16-測試策略)
17. [錯誤碼目錄](#17-錯誤碼目錄)
18. [效能指標](#18-效能指標)
19. [CI/CD 管線](#19-cicd-管線)
20. [無障礙設計](#20-無障礙設計)

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
- 同 WiFi 區域網路下跨裝置同步（家長手機 ↔ 子女手機）
- 離線可用、同網路時自動同步

### 1.4 核心價值

| 價值 | 說明 |
|------|------|
| 簡單易用 | 無需註冊帳號，設定角色即可使用 |
| 透明記帳 | 所有金錢異動都有完整紀錄 |
| 教育理財 | 讓子女從零用金管理中學習金錢觀念 |
| 即時溝通 | 同 WiFi 時請款審核即時同步 |
| 隱私優先 | 資料完全存放在裝置本地，無雲端 |

### 1.5 使用者角色

| 角色 | 英文代號 | 說明 |
|------|---------|------|
| 家長/給予者 | `giver` | 建立家庭、設定津貼、審核請款、管理錢包、同步權威 |
| 子女/寶寶 | `baby` | 查看餘額、提出請款、查看津貼、學習理財 |

### 1.6 家庭角色（Family Role）

| 家庭角色 | 說明 |
|---------|------|
| `giver` | 可發放津貼、建立錢包、審核請款、為同步權威端 |
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
  開啟 App → 設定裝置
  → 輸入顯示名稱
  → 選擇角色：Giver
  → 建立家庭（輸入家庭名稱）
  → 建立錢包（如「現金零用金」）
  → 產生配對碼 → 讓子女掃描或輸入配對碼加入家庭

日常使用：
  開啟 App → Dashboard
  → 查看待審核請款（紅色徽章）
  → 點擊請款 → 查看詳情/收據
  → 核准或駁回（駁回需填寫原因）
  → 查看錢包餘額與交易紀錄
  → 同 WiFi 時自動與 Baby 裝置同步

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
  開啟 App → 設定裝置
  → 輸入顯示名稱
  → 選擇角色：Baby
  → 輸入 Giver 提供的配對碼（需在同 WiFi 下）
  → 配對成功 → 加入家庭
  → 同步家庭資料（錢包、津貼等）

日常使用：
  開啟 App → Dashboard
  → 查看錢包餘額
  → 提出請款
    → 輸入金額、分類、備註
    → 可附加收據圖片
    → 送出請款
  → 同 WiFi 時自動同步 → 等待 Giver 審核
  → 收到通知（核准/駁回）
  → 查看交易紀錄
```

### 2.3 MVP 功能優先級

| 優先級 | 功能 | 說明 |
|--------|------|------|
| P0 | 裝置設定 | 選擇角色、設定顯示名稱 |
| P0 | 家庭配對 | 同 WiFi 下配對碼建立家庭 |
| P0 | 錢包管理 | 建立錢包、查看餘額 |
| P0 | 請款流程 | 建立→送出→審核→記帳 |
| P0 | WiFi 同步 | mDNS 發現 + HTTP 同步 |
| P1 | 定期津貼 | 自動發放排程 |
| P1 | 交易紀錄 | 完整帳本查詢 |
| P1 | 通知系統 | App 內通知 |
| P2 | 生物辨識鎖定 | 指紋 / Face ID 解鎖 App |
| P2 | 審計日誌 | 操作追蹤 |
| P3 | 多語言 | 英文支援 |
| P3 | 高對比主題 | 無障礙支援 |
| P3 | 資料匯出 | CSV 交易紀錄匯出 |

### 2.4 裝置配對流程

```
Giver 端：
  家庭管理 → 「邀請成員」
  → 系統產生 6 位數配對碼（有效期 5 分鐘）
  → 同時啟動 mDNS 廣播 + 本地 HTTP 同步服務
  → 畫面顯示配對碼 + QR Code
  → 等待 Baby 裝置連入

Baby 端：
  設定 → 「加入家庭」
  → 輸入 6 位數配對碼（或掃描 QR Code）
  → 系統透過 mDNS 尋找同 WiFi 的 Giver 裝置
  → 找到 → 以配對碼驗證身份
  → 驗證成功 → 交換裝置 UUID + 家庭資料
  → 配對完成 → 開始同步
```

### 2.5 資料匯出流程

```
設定頁 → 匯出交易紀錄
  → 選擇日期範圍 + 錢包
  → 本地產生 CSV 檔案
  → 透過系統分享功能儲存/傳送
```

### 2.6 首次使用引導（Onboarding）

```
Giver 引導：
  Step 1: 設定顯示名稱 + 選擇角色 (Giver)
  Step 2: 建立家庭（輸入家庭名稱）
  Step 3: 建立第一個錢包
  Step 4: 邀請成員（產生配對碼，可跳過稍後配對）
  → 進度指示器 1/4 ~ 4/4

Baby 引導：
  Step 1: 設定顯示名稱 + 選擇角色 (Baby)
  Step 2: 加入家庭（輸入配對碼）
  Step 3: 查看錢包餘額介紹
  → 進度指示器 1/3 ~ 3/3
```

---

## 3. 頁面設計

### 3.1 頁面總覽

| 頁面 | 路徑 | Giver | Baby | 說明 |
|------|------|-------|------|------|
| 裝置設定 | `/setup` | ✅ | ✅ | 首次使用：名稱 + 角色選擇 |
| 配對 | `/pairing` | ✅ | ✅ | 產生/輸入配對碼 |
| Dashboard | `/` | ✅ | ✅ | 首頁，依角色顯示不同內容 |
| 錢包列表 | `/wallets` | ✅ | ✅ | 家庭所有錢包 |
| 錢包詳情 | `/wallets/:id` | ✅ | ✅ | 錢包餘額 + 交易紀錄 |
| 津貼管理 | `/allowances` | ✅ | ✅(唯讀) | 津貼列表與設定 |
| 請款列表 | `/requests` | ✅ | ✅ | 所有請款 |
| 建立請款 | `/requests/new` | ❌ | ✅ | Baby 建立新請款 |
| 請款詳情 | `/requests/:id` | ✅ | ✅ | 查看/審核請款 |
| 交易紀錄 | `/transactions` | ✅ | ✅ | 所有交易 |
| 通知中心 | `/notifications` | ✅ | ✅ | 通知列表 |
| 設定 | `/settings` | ✅ | ✅ | 主題/語言/裝置/匯出 |
| 家庭管理 | `/family` | ✅ | ❌ | 管理成員（Giver 專屬） |

### 3.2 裝置設定頁 `/setup`（首次使用）

**功能**：
- 顯示名稱輸入
- 角色選擇（Giver / Baby）— 大圖示卡片式選擇
- 「開始使用」按鈕

**互動行為**：
- 選擇 Giver → 導向建立家庭 → Onboarding 流程
- 選擇 Baby → 導向配對頁面

### 3.3 配對頁 `/pairing`

#### Giver 端（產生配對碼）

- 顯示 6 位數配對碼（大字體）+ QR Code
- 倒計時 5 分鐘（過期自動產生新碼）
- 同步狀態指示（等待中 / 連接中 / 配對成功）

#### Baby 端（輸入配對碼）

- 6 格數字輸入框 + QR Code 掃描按鈕
- 錯誤提示「請確認與家長在同一個 WiFi 網路」
- 配對成功 → 同步家庭資料 → 導向 Dashboard

### 3.4 Dashboard `/`

#### Giver 版

- 歡迎訊息 + 同步狀態指示器（🟢 已連線 / 🔴 未連線 / 🔄 同步中）
- 待審核請款數量徽章
- 錢包餘額摘要卡片
- 最近 5 筆交易
- 快捷操作：審核請款、手動記帳

#### Baby 版

- 歡迎訊息 + 同步狀態指示器
- 錢包餘額（主要錢包突顯）
- 最近津貼發放
- 請款狀態摘要
- 快捷操作：建立請款

**狀態設計**：

| 狀態 | 顯示內容 |
|------|---------|
| Loading | 卡片骨架屏 |
| Error | 錯誤訊息 + 「重試」按鈕 |
| Empty | 引導建立錢包（Giver）/ 等待同步（Baby） |

### 3.5 ~ 3.12 其他頁面

錢包列表、錢包詳情、津貼管理、請款列表、建立請款、請款詳情、交易紀錄、通知中心 — 功能設計與前版相同，每頁均包含 Loading / Error / Empty 狀態設計。

通知產生時機：同步完成後偵測到新資料（如請款被核准）→ 本地產生通知。

### 3.13 設定頁 `/settings`

- 裝置資訊（顯示名稱、角色、裝置 ID）
- 主題切換（Light / Dark / High Contrast）
- 語言切換（繁體中文 / English）
- App 鎖定（PIN / 生物辨識 開關）
- 匯出交易紀錄（CSV）
- 同步狀態（最後同步時間、已配對裝置列表）
- 重設裝置（清除所有資料）

### 3.14 家庭管理 `/family`（Giver 專屬）

- 家庭名稱、已配對成員列表（名稱、角色、最後同步時間）
- 邀請成員（產生新配對碼）
- 移除成員（取消配對）

### 3.15 確認對話框

| 操作 | 確認訊息 |
|------|---------|
| 封存錢包 | 「確定要封存此錢包嗎？」 |
| 移除成員 | 「確定要將此成員移除嗎？」 |
| 取消請款 | 「確定要取消此請款嗎？」 |
| 重設裝置 | 「確定要重設嗎？所有本地資料將被清除。」 |

### 3.16 底部導航（Tab Bar）

| Tab | 圖示 | 路徑 |
|-----|------|------|
| 首頁 | Home | `/` |
| 錢包 | Wallet | `/wallets` |
| 請款 | Receipt | `/requests` |
| 通知 | Bell | `/notifications` |
| 設定 | Gear | `/settings` |

---

## 4. 核心功能規格

### 4.1 裝置設定

- 首次啟動進入設定：輸入顯示名稱 + 選擇角色（Giver / Baby）
- 系統自動產生裝置 UUID
- 存入本地 `profiles` 表
- 角色設定後不可變更（需重設裝置）

### 4.2 裝置配對與家庭建立

- Giver 建立家庭 → 產生 6 位數配對碼（有效期 5 分鐘）
- Baby 輸入配對碼 → mDNS 搜尋 Giver → 驗證 → 配對成功
- 配對後交換 UUID + profile + 家庭資料
- 一個 Giver 可配對多個 Baby
- 關係存入 `paired_devices` 表

### 4.3 App 鎖定（可選）

- 4~6 位數 PIN 碼（Argon2id hash）
- 生物辨識（`tauri-plugin-biometric`）
- App 進入背景後回前台需解鎖

### 4.4 ~ 4.8 家庭管理、錢包、津貼、請款、交易

與前版業務邏輯相同。關鍵差異：
- 津貼發放 **僅在 Giver 裝置執行**（App 啟動/前台時檢查）
- 請款送出後在下次 WiFi 同步時傳遞給 Giver
- 核准結果在下次同步時傳回 Baby

#### 4.8.1 附件儲存

- 本地：`{app_data}/attachments/{uuid}.jpg`
- 限制：每張 ≤ 5MB、每筆 ≤ 3 張、JPEG/PNG/HEIC
- 同步時透過嵌入式 HTTP 附件端點傳輸

### 4.9 通知系統

- 純本地通知：同步完成後偵測新資料 → 產生通知
- 狀態：`unread` → `read`

### 4.10 WiFi P2P 同步

- Giver 啟動嵌入式 HTTP server + mDNS 廣播
- Baby 透過 mDNS 發現 Giver → 連接同步
- 觸發時機：App 啟動/前台、手動、WiFi 連線
- 衝突解決：Giver 優先
- 詳見 [Section 13](#13-p2p-同步協議)

### 4.11 審計日誌

記錄操作者裝置、家庭、行為、資源類型/ID。

### 4.12 App 生命週期

- 前台：啟動 mDNS + HTTP server（Giver）/ 搜尋同步（Baby）
- 背景：停止一切同步

---

## 5. 資料庫結構

### 5.1 設計原則

- 金額以 cents (i64) 儲存
- 時間使用 TEXT（ISO 8601）
- 軟刪除用 `is_deleted`
- 所有實體表含 `uuid`（同步識別）+ `sync_version`（變更計數）+ `last_synced_at` + `is_deleted`
- SQLite WAL mode + foreign_keys

### 5.2 profiles 表

```sql
CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    display_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('giver','baby')),
    locale TEXT NOT NULL DEFAULT 'zh-TW',
    theme TEXT NOT NULL DEFAULT 'default',
    pin_hash TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### 5.3 families 表

```sql
CREATE TABLE IF NOT EXISTS families (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    name TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'TWD',
    timezone TEXT NOT NULL DEFAULT 'Asia/Taipei',
    created_by_device TEXT NOT NULL,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### 5.4 family_members 表

```sql
CREATE TABLE IF NOT EXISTS family_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    profile_uuid TEXT NOT NULL,
    family_role TEXT NOT NULL CHECK (family_role IN ('giver','baby','viewer')),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','removed')),
    joined_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (family_id, profile_uuid),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE
);
```

### 5.5 paired_devices 表

```sql
CREATE TABLE IF NOT EXISTS paired_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_uuid TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('giver','baby')),
    family_id INTEGER NOT NULL,
    last_seen_at TEXT,
    last_sync_at TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE
);
```

### 5.6 wallets 表

```sql
CREATE TABLE IF NOT EXISTS wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('cash','bank','card','virtual')),
    balance_cents INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'TWD',
    warning_threshold_cents INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','archived')),
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    UNIQUE (family_id, name)
);
```

### 5.7 allowances 表

```sql
CREATE TABLE IF NOT EXISTS allowances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
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
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);
```

### 5.8 requests 表

```sql
CREATE TABLE IF NOT EXISTS requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    requester_member_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    amount_cents INTEGER NOT NULL,
    category TEXT CHECK (category IN ('food','transport','education','entertainment','clothing','health','other')),
    notes TEXT,
    attachment_url TEXT,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','pending','approved','rejected','cancelled')),
    decision_by_member_id INTEGER,
    decision_at TEXT,
    rejection_reason TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_requests_family_status ON requests(family_id, status);
```

### 5.9 transactions 表

```sql
CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('credit','debit')),
    amount_cents INTEGER NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('allowance','request','manual','adjustment')),
    source_id INTEGER,
    category TEXT,
    occurred_at TEXT NOT NULL,
    notes TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_transactions_wallet_date ON transactions(wallet_id, occurred_at);
CREATE INDEX IF NOT EXISTS idx_transactions_family_date ON transactions(family_id, occurred_at);
```

### 5.10 notifications 表

```sql
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL CHECK (event_type IN (
        'request_submitted','request_approved','request_rejected',
        'allowance_disbursed','low_balance','member_joined'
    )),
    payload TEXT,
    is_read INTEGER NOT NULL DEFAULT 0,
    read_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### 5.11 audit_logs 表

```sql
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    actor_device TEXT,
    family_id INTEGER,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### 5.12 sync_state 表

```sql
CREATE TABLE IF NOT EXISTS sync_state (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

### 5.13 餘額校驗

同步完成後執行：`SELECT SUM(credit) - SUM(debit) FROM transactions` 比對 `wallets.balance_cents`，飄移時自動建立 `adjustment` 交易修正。

---

## 6. 列舉值與常數

### 6.1 裝置角色

| 值 | 說明 |
|------|------|
| `giver` | 家長（同步權威端） |
| `baby` | 子女 |

### 6.2 錢包類型

`cash` / `bank` / `card` / `virtual`

### 6.3 津貼頻率

`daily` / `weekly` / `biweekly` / `monthly` / `custom`

### 6.4 請款狀態

`draft` / `pending` / `approved` / `rejected` / `cancelled`

### 6.5 交易類型

`credit` / `debit`

### 6.6 交易來源

`allowance` / `request` / `manual` / `adjustment`

### 6.7 通知事件

`request_submitted` / `request_approved` / `request_rejected` / `allowance_disbursed` / `low_balance` / `member_joined`

### 6.8 請款分類

`food` / `transport` / `education` / `entertainment` / `clothing` / `health` / `other`

---

## 7. 業務規則

### 7.1 金額處理

cents (i64)，NT$100 = `10000`，前端除以 100 顯示。

### 7.2 錢包餘額變更

必須使用 SQLite transaction，同時寫入 `transactions` 表。餘額不足回傳 `INSUFFICIENT_BALANCE`。

### 7.3 請款狀態機

```
draft → pending → approved / rejected / cancelled
```

- 駁回必須填 `rejection_reason`
- 核准自動扣款 + 寫交易 + 遞增 `sync_version`

### 7.4 津貼發放

僅 Giver 裝置執行。App 前台時檢查到期津貼並發放。結果在下次同步傳給 Baby。

### 7.5 同步

- 雙向：Baby push → Giver merge → Giver pull → Baby overwrite
- Giver 為權威端
- transactions append-only
- balance_cents 同步後由交易重算

### 7.6 授權

- Giver：所有管理操作
- Baby：查看 + 請款
- Viewer：唯讀

### 7.7 輸入驗證

名稱 1~50 字、金額正整數、備註 ≤ 500 字、附件 JPEG/PNG/HEIC ≤ 5MB × 3、PIN 4~6 位數。

### 7.8 資料匯出

Giver 匯出 CSV（日期/錢包/類型/金額/來源/分類/備註），本地產生。

---

# Part B — 技術架構

---

## 8. 技術堆疊總覽

| 層級 | 技術 | 說明 |
|------|------|------|
| 原生殼層 | Tauri v2 | Android/iOS WebView |
| 後端邏輯 | Rust | 業務邏輯 + SQLite + P2P 同步 |
| 資料庫 | SQLite (sqlx) | WAL mode |
| IPC | tauri-specta | Type-safe TypeScript 綁定 |
| 嵌入式 HTTP | axum | 同步用本地 HTTP server |
| 裝置發現 | mdns-sd | mDNS 廣播與發現 |
| 前端 | React 19 + Vite | UI |
| 樣式 | Tailwind CSS 4 + shadcn/ui | CSS-first |
| 路由 | React Router v7 | `createBrowserRouter` |
| 狀態 | TanStack Query v5 + Zustand | Server + Client state |
| i18n | i18next | zh-TW / en |
| 生物辨識 | tauri-plugin-biometric | 指紋 / Face ID |
| 安全儲存 | tauri-plugin-store | PIN hash 等 |

---

## 9. 專案結構

```
Cacao/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── migrations/
│   └── src/
│       ├── lib.rs
│       ├── error.rs              # AppError
│       ├── specta.rs             # tauri-specta
│       ├── commands/
│       │   ├── setup.rs          # 裝置設定 + 配對
│       │   ├── family.rs
│       │   ├── wallet.rs
│       │   ├── allowance.rs
│       │   ├── request.rs
│       │   ├── transaction.rs
│       │   └── notification.rs
│       ├── db/pool.rs
│       ├── models/
│       ├── services/
│       ├── sync/
│       │   ├── server.rs         # 嵌入式 Axum (Giver)
│       │   ├── client.rs         # HTTP client (Baby)
│       │   ├── discovery.rs      # mDNS
│       │   ├── protocol.rs       # 同步協議
│       │   └── merge.rs          # 衝突解決
│       └── scheduler/
│           └── allowance_runner.rs
├── src/
│   ├── App.tsx
│   ├── app/
│   │   ├── setup.tsx
│   │   ├── pairing.tsx
│   │   ├── dashboard.tsx
│   │   ├── wallets/
│   │   ├── requests/
│   │   ├── ...
│   │   └── settings/
│   ├── components/
│   │   ├── ui/                   # shadcn/ui
│   │   ├── sync-indicator.tsx
│   │   └── ...
│   ├── lib/bindings.ts           # tauri-specta 產生
│   ├── stores/
│   │   ├── profile-store.ts
│   │   ├── ui-store.ts
│   │   └── sync-store.ts
│   ├── i18n/
│   └── styles/globals.css
├── docs/
└── package.json
```

---

## 10. Rust 後端架構（Tauri）

### 10.1 架構：Commands → Services → SQLite → Sync Engine

### 10.2 AppError

```rust
#[derive(Debug, Serialize, Type)]
pub struct AppError { pub code: String, pub message: String }
```

### 10.3 Cargo.toml 依賴

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
```

---

## 11. React 前端架構

### 11.1 路由

```tsx
const router = createBrowserRouter([
  { path: '/setup', element: <SetupPage /> },
  { path: '/pairing', element: <PairingPage /> },
  {
    element: <SetupGuard />,
    children: [{
      element: <AppLayout />,
      children: [
        { index: true, element: <DashboardPage /> },
        { path: 'wallets', element: <WalletsPage /> },
        { path: 'wallets/:id', element: <WalletDetailPage /> },
        // ... 其他路由
      ],
    }],
  },
]);
```

### 11.2 前端依賴

與前版相同（React 19, React Router v7, TanStack Query v5, Zustand, Tailwind CSS 4, shadcn/ui, i18next, zod），無 autoprefixer。

---

## 12. 資料流架構

```
┌─────────────────────────────────────┐
│            React UI                  │
│  TanStack Query + Zustand           │
└──────────────┬──────────────────────┘
               │ tauri-specta invoke
               ▼
┌─────────────────────────────────────┐
│        Tauri Commands                │
│  → Services → SQLite                 │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│        P2P Sync Engine               │
│  mDNS + Embedded Axum HTTP           │
│  Giver (server) ←→ Baby (client)    │
│  Same WiFi LAN only                 │
└─────────────────────────────────────┘
```

---

## 13. P2P 同步協議

### 13.1 mDNS

Giver 廣播 `_cacao._tcp.local.`，TXT 含 `family_uuid` + `device_name`。Baby 搜尋並比對 `family_uuid` 連接。

### 13.2 嵌入式 HTTP Server（Giver 端）

```
POST /sync/handshake    — 配對驗證
POST /sync/push         — Baby 推送變更
GET  /sync/pull?since=  — Baby 拉取 Giver 資料
GET  /sync/attachments/:uuid — 附件下載
POST /sync/attachments  — 附件上傳
```

### 13.3 同步流程

1. Baby → `POST /sync/push`（本地變更）
2. Giver 合併（Giver 優先解決衝突）
3. Baby → `GET /sync/pull?since={last_sync}`
4. Baby 覆蓋本地 → 重算 `balance_cents` → 產生通知

### 13.4 衝突解決

- **Giver 端資料永遠優先**
- Baby push 的變更：Giver 的 `sync_version` ≥ Baby → 拒絕；< Baby → 接受
- Baby pull 後直接覆蓋本地
- transactions 為 append-only，無衝突
- balance_cents 由 transactions 重算

---

## 14. 建置與部署

```bash
pnpm install
cargo tauri dev                        # 桌面預覽
cargo tauri android dev / build        # Android
cargo tauri ios dev / build            # iOS
```

| 項目 | 值 |
|------|------|
| Bundle ID | `com.cacao.app` |
| 最低 Android | API 24 |
| 最低 iOS | 14.0 |

---

# Part C — 營運與品質

---

## 15. 安全性考量

- 同步僅限 WiFi LAN，不經公網
- 配對碼一次性、5 分鐘有效
- 已配對裝置以 `device_uuid` + `family_uuid` 驗證
- PIN hash 使用 Argon2id
- SQLite 在 App 私有目錄（OS 保護）
- SQL 參數化查詢（防注入）

---

## 16. 測試策略

- **Unit**：Rust services（sqlx + in-memory SQLite）、React（Vitest + Testing Library）
- **Integration**：兩個 SQLite instance 模擬 Giver/Baby 同步 + 衝突
- **E2E**：設定 → 配對 → 建立錢包 → 請款 → 同步 → 核准 → 同步 → 驗證餘額

---

## 17. 錯誤碼目錄

| Code | 說明 |
|------|------|
| `DEVICE_ALREADY_SETUP` | 裝置已設定 |
| `PAIRING_CODE_INVALID` | 配對碼錯誤 |
| `PAIRING_CODE_EXPIRED` | 配對碼已過期 |
| `PEER_NOT_FOUND` | 找不到 Giver 裝置 |
| `WALLET_NOT_FOUND` | 錢包不存在 |
| `WALLET_ARCHIVED` | 錢包已封存 |
| `WALLET_NAME_EXISTS` | 同名錢包已存在 |
| `INSUFFICIENT_BALANCE` | 餘額不足 |
| `REQUEST_NOT_FOUND` | 請款不存在 |
| `REQUEST_ALREADY_DECIDED` | 請款已決定 |
| `REQUEST_INVALID_STATUS` | 狀態不允許此操作 |
| `REJECTION_REASON_REQUIRED` | 駁回需填原因 |
| `VALIDATION_ERROR` | 輸入驗證失敗 |
| `FORBIDDEN` | 無權限 |
| `SYNC_IN_PROGRESS` | 同步中 |
| `INTERNAL_ERROR` | 非預期錯誤 |

---

## 18. 效能指標

| 指標 | 目標 |
|------|------|
| 冷啟動 | < 2s |
| 頁面切換 | < 200ms |
| DB 查詢 | < 50ms (p95) |
| WiFi 同步 | < 3s |
| mDNS 發現 | < 5s |
| 配對完成 | < 10s |

---

## 19. CI/CD 管線

PR 檢查：`cargo clippy` + `cargo test` + `cargo fmt` + `pnpm lint` + `pnpm test`

Release（tag `v*.*.*`）：`cargo tauri android/ios build --release`

---

## 20. 無障礙設計

- 螢幕閱讀器：`aria-label`、`aria-live`
- 觸控目標：≥ 48x48 dp
- 字型縮放：支援 200%，用 `rem`
- 色彩對比：WCAG AA 4.5:1
- 動畫：遵循 `prefers-reduced-motion`
