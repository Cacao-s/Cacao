# Cacao 系統設計說明

本文件統整 `doc/handbookv1.md` 與 `doc/handbookv2.md` 的內容，進一步整理為可執行的系統設計說明（SDD），用於對齊產品願景、技術設計與實作驗收。所有章節皆以 MVP（Minimum Viable Product）為當前交付目標，並標示後續擴充方向。

## 1. 文件資訊
### 1.1 目的
界定 Cacao 家庭零用錢管理系統的功能、架構與技術決策，提供開發、測試、營運及利害關係人共同遵循的設計基準。

### 1.2 範圍
- 前端 Nuxt 3 PWA（含行動包裝、國際化、主題）。
- 後端 Go（Gin）API、資料庫、同步與通知邏輯。
- 相關部署、測試、安規與品質要求。

### 1.3 參考文件
- `doc/handbookv1.md`
- `doc/handbookv2.md`
- `README.md`
- `docs/` 目錄下既有或待建立的支援性文件（API Outline、Backend Startup、Deploy Playbook 等）。

### 1.4 名詞定義
- **Giver**：提供零用金或任務獎勵的家長/監護人。
- **Baby**：領取零用金並紀錄支出的孩子。
- **Allowance**：Giver 設定的發放規則，可定期或臨時。
- **Request**：Baby 提出的提款/請款申請，需要 Giver 審核。
- **Wallet**：家庭使用的支付工具或資金來源，如現金、銀行帳戶、信用卡。
- **Transaction**：任何金流事件，包含發放、支出、退款、調整。
- **MVP**：第一個可上線驗證的最小可行產品。

## 2. 系統概要
### 2.1 產品定位
- 解決家庭零用錢管理分散、紀錄難以共享的痛點。
- 提供親子雙方透明、可追蹤的協作流程。
- 先完成 Android PWA MVP 以驗證市場，再逐步擴充 iOS 與付費方案。

### 2.2 主要角色
- Giver（父母/監護人）
- Baby（孩子使用者）
- 系統管理員（內部角色，必要時導入，用於審計與維運）

### 2.3 核心使用情境
1. Giver 建立家庭並邀請 Baby。
2. Giver 規劃 Allowance，指定錢包與發放規則。
3. Baby 提交 Request 或紀錄支出。
4. Giver 審核 Request、核准或退回並更新餘額。
5. 系統同步產生 Transaction，雙方可於時間軸/儀表板查詢。
6. 通知與提醒機制協助即時掌握最新狀況。
7. 前端離線時可暫存操作，上線後自動與雲端同步。

### 2.4 運行環境
- 前端：瀏覽器（Chrome/Android WebView/Capacitor 容器）、PWA 模式。
- 後端：Go 1.21+，Gin Web Framework，Docker 容器化部署。
- 資料庫：MySQL（正式）、SQLite（前端快取）、SQLite in-memory（測試）。

## 3. 設計驅動因素
### 3.1 產品目標
- 建立交易一致性與透明化。
- 強化親子互動與財務教育。
- 提供可快速上線的 MVP，保留擴充彈性。

### 3.2 假設
- 家庭成員以 Email 註冊，可使用 Gmail OAuth2。
- 初期使用者集中在中文（繁體）市場，並具備英語擴充需求。
- 行動裝置使用 Android 為主，透過 PWA/Capacitor 形式安裝。
- 系統使用者人數與交易量在 MVP 階段可由單一資料庫承載。

### 3.3 設計約束
- 前後端需完全解耦，可獨立開發與部署。
- 所有設定以環境變數為主，不允許硬編碼機敏資訊。
- 支援多國語系與三種主題（預設、淺色、深色），使用者偏好需持久化。
- 離線模式僅限前端快取，伺服器資料為最終真實來源。
- 需符合個資保護與安全性最佳實務（JWT、Refresh Token、日誌紅線）。

## 4. 功能需求（MVP）
### 4.1 功能列表
- **帳號與角色管理**：註冊、登入、角色/家庭關聯、狀態管理、軟刪除。
- **身份驗證**：JWT Access + Refresh Token，HttpOnly Cookie、角色授權、逾期刷新。
- **家庭與成員管理**：Giver 建立家庭、邀請/接受流程、成員清單。
- **錢包管理**：增刪改查、多種錢包類型、餘額驗證、餘額上下限與幣別設定。
- **零用金設定（Allowance）**：建立、排程發放、臨時獎勵、與錢包綁定。
- **請款流程（Request）**：Baby 建立申請、上傳附件/備註、狀態追蹤。
- **審核流程**：Giver 核准/退回、意見備註、通知觸發。
- **交易紀錄（Transaction）**：自動生成金流紀錄、分類標籤、時間軸展示。
- **儀表板與統計**：餘額、待辦請求、分類統計、趨勢圖。
- **通知與提醒**：Polling 實作初始通知；接口預留 FCM/Line Notify/WebSocket。
- **離線支援**：SQLite 快取、操作佇列、上線後與雲端雙向同步。
- **錯誤與提示**：統一 API 回應格式、錯誤碼與訊息標準化。
- **UI 國際化與主題**：語系資源載入、語言切換、三種主題切換與持久化。
- **文件同步**：啟動指南、API Outline、產品筆記與設計文件隨變更更新。

### 4.2 主要使用流程
- **Allowance 發放**：排程或手動 → 檢查錢包餘額 → 建立 Transaction → 更新餘額 → 通知 Baby。
- **Request 審核**：Baby 建立 Request → 後端校驗條件 → Giver 審核 → 建立 Transaction 與通知。
- **離線紀錄**：前端寫入 SQLite → 連線恢復 → 將佇列同步至 API → API 以交易鎖確保一致性 → 返回最新狀態。

### 4.3 出版後擴充（Post-MVP）
- 多 Giver / 多 Baby 角色授權矩陣。
- 自動發錢、通知歷史、報表匯出、遊戲化機制。
- 媒體上傳、家庭任務看板、付費方案與上架流程。

## 5. 系統架構
### 5.1 高階架構描述
- **Client 層**：Nuxt 3 SPA（PWA + Capacitor），透過 HTTPS 與 API 通訊，LocalStorage 管理 Token 指標，SQLite 快取資料。
- **API 層**：Go (Gin) 服務，提供 REST API、身份驗證、業務邏輯、同步、通知觸發。
- **資料層**：MySQL/MSSQL 作為核心資料庫；使用 GORM ORM；Migration/Seed 工具維護 schema。
- **整合層**：OAuth2（Google）、未來通知服務（FCM/Line）、觀測工具（Sentry、Prometheus）。
- **部署層**：Docker 映像與 Compose，CI/CD 管線負責建置與部署。

### 5.2 元件概覽
- **前端應用**：頁面模組（Dashboard、Wallets、Allowances、Requests、Settings）、共用元件（表單、圖表、通知炫示）、Pinia store、i18n 模組、Theme 管理。
- **後端模組**：
  - `server/main.go`：啟動流程、路由、設定注入。
  - `routes`：路由設定與中介層（Authentication、Rate Limit、Request ID）。
  - `controllers`：處理 HTTP 請求、呼叫 service、統一回傳格式。
  - `services`：業務邏輯、交易流程、通知整合。
  - `repositories`：資料存取抽象層，支援多資料庫。
  - `infra`：資料庫連線、快取、外部服務客戶端。
  - `jobs`（規劃中）：排程 Allowance 發放、同步佇列處理。
- **同步與排程**：Cron/Go routine 監控待發放任務，確保離線操作回補。

### 5.3 資料流
- 使用者操作 → Nuxt 呼叫 API → Gin 認證與授權 → Service 執行交易（透過 Repository 與 DB 互動）→ 返回結果 → 前端更新狀態並緩存。
- SQLite 離線佇列 → 前端上線 → Sync Service → API 批次處理 → 回傳同步狀態。

### 5.4 安全界線
- 所有外部請求必須經過 HTTPS。
- API Gateway（或 Gin Middleware）負責身份認證與節流。
- 敏感設定透過環境變數注入，避免寫入程式碼庫。

## 6. 模組設計細節
### 6.1 前端
- **技術棧**：Nuxt 3（SPA mode）、Pinia（狀態）、Ant Design Vue + Tailwind CSS、Ionic 組件。
- **資料管理**：`useApi()` 封裝 API 請求與錯誤攔截；Pinia store 分模組（auth、wallet、allowance、request、transaction、settings）。
- **i18n**：使用 `@nuxtjs/i18n` 或等效方案，語系文案以 JSON 分檔管理；預設 zh-TW，附 en，允許擴充。
- **Theme**：建立 Theme Provider，三種 Theme（default/light/dark），使用 CSS 變數或 Tailwind config，偏好儲存在 `localStorage` 與使用者設定 API。
- **離線模式**：採用 Capacitor/IndexedDB 或 `@capacitor-community/sqlite` 與 WebAssembly SQLite，維護一份操作佇列。
- **路由守衛**：檢查 JWT、角色授權，未登入導向登入頁。
- **失敗回復**：顯示統一錯誤訊息、重試按鈕、離線提示。

### 6.2 後端
- **路由**：依資源分路由群組，加上 `/auth`, `/users`, `/families`, `/wallets`, `/allowances`, `/requests`, `/transactions`, `/notifications`.
- **中介層**：JWT 驗證、角色授權、Request ID、Rate Limit、Error Handling、CORS。
- **Service 層**：
  - `AuthService`：註冊、登入、JWT 發行、刷新、登出、密碼雜湊。
  - `UserService`：管理使用者資料、角色、家庭關聯。
  - `WalletService`：錢包 CRUD、餘額調整（使用 DB Transaction）。
  - `AllowanceService`：建立與排程發放，驗證是否可扣款。
  - `RequestService`：請款提交、附件處理、狀態管理。
  - `TransactionService`：生成交易紀錄、查詢、報表統計。
  - `NotificationService`：序列化通知事件，MVP 使用輪詢資料表或 Redis List。
  - `SyncService`：處理前端佇列批次，確保冪等。
- **資料庫層**：使用 GORM；每個 repository 提供介面與 MySQL/SQLite 實作。
- **錯誤處理**：自訂錯誤類型（Validation, NotFound, Forbidden, Conflict），統一轉換為 API 回應。

### 6.3 離線與同步策略
- 前端所有離線操作標記 `temp_id`，並包含最後同步時間。
- 後端 Sync API 接受批次請求，保證 Transaction 原子性，重複 `temp_id` 應視為冪等操作。
- 同步成功後回傳伺服器端正式 ID 與最新餘額。

### 6.4 通知策略
- MVP：輪詢 `/notifications/poll?since=<timestamp>`，回傳未處理通知。
- Roadmap：導入 FCM、Line Notify 或 WebSocket 推播；抽象通知介面便於擴充。

### 6.5 排程與批次
- 使用 `cron` 或 `robfig/cron` 觸發 Allowance 發放。
- 排程任務需支援互斥鎖（資料庫或 Redis）避免重複執行。
- 任務結果寫入 `jobs_run` 或日誌，供監控。

## 7. 資料設計
### 7.1 實體關係概述
- 一個 `User` 可屬於多個家庭；一個家庭可有多個 `Wallet`、`Allowance`、`Request`。
- `Allowance` 與 `Transaction` 為一對多；`Request` 與 `Transaction` 一對一（核准時）。
- `Wallet` 與 `Transaction` 一對多；餘額計算需以交易為準。

### 7.2 主要資料表
| Table | 重點欄位 | 備註 |
| :---- | :------- | :--- |
| `users` | `id`, `email`, `password_hash`, `google_sub`, `display_name`, `role`, `status`, `locale`, `theme`, `created_at`, `deleted_at` | `role` 代表系統層級角色（giver/baby）；`status` 包含 active/invited/suspended。 |
| `families` | `id`, `name`, `timezone`, `currency`, `created_by`, `created_at` | 負責儲存家庭組織資訊。 |
| `family_members` | `id`, `family_id`, `user_id`, `family_role`, `invited_by`, `joined_at` | `family_role` 可為 giver/baby；支援多成員。 |
| `wallets` | `id`, `family_id`, `name`, `type`, `balance`, `currency`, `owner_member_id`, `status`, `created_at` | `type`：cash/bank/card/other；餘額使用 DECIMAL(12,2) 或 int cents。 |
| `allowances` | `id`, `family_id`, `giver_member_id`, `receiver_member_id`, `wallet_id`, `amount`, `frequency`, `next_run_at`, `last_run_at`, `auto_approve`, `notes`, `status` | `frequency`：weekly/monthly/custom；`status`：active/paused。 |
| `requests` | `id`, `family_id`, `requester_member_id`, `wallet_id`, `amount`, `category`, `notes`, `receipt_url`, `status`, `decision_by`, `decision_at`, `rejection_reason` | `status`：pending/approved/rejected/cancelled。 |
| `transactions` | `id`, `family_id`, `wallet_id`, `amount`, `type`, `source_type`, `source_id`, `category`, `occurred_at`, `created_by`, `notes` | `type`：credit/debit；`source_type`：allowance/request/manual/adjustment。 |
| `notifications` | `id`, `family_id`, `user_id`, `event_type`, `payload`, `is_read`, `created_at` | 儲存需送達的通知事件。 |
| `audit_logs` | `id`, `actor_id`, `family_id`, `action`, `resource_type`, `resource_id`, `diff`, `created_at` | 用於安全與審計。 |
| `sync_queue` | `id`, `device_id`, `user_id`, `payload`, `status`, `retries`, `last_error`, `created_at` | 接收離線佇列，MVP 可選擇延後導入。 |

### 7.3 商業規則
- `wallets.balance` 必須與 `transactions` 累計一致，所有更新應透過交易。
- `allowances` 定期排程需檢查 `wallet.balance >= amount`，不足時將狀態標記為 `pending_funds` 並通知 Giver。
- `requests` 核准後自動建立交易；退回時不建立交易但需通知 Baby。
- `locale`、`theme` 儲存在 `users` 以支援多裝置同步。

## 8. API 設計摘要
| 模組 | Method | Path | 說明 | 驗證 |
| :--- | :----- | :--- | :--- | :--- |
| Auth | POST | `/auth/register` | Email/密碼註冊，建立家庭或加入既有家庭。 | 無（Rate Limit） |
| Auth | POST | `/auth/login` | 驗證密碼或交換 OAuth Token，簽發 JWT。 | 無 |
| Auth | POST | `/auth/refresh` | 透過 Refresh Token 取得新 Access Token。 | Refresh Token |
| Users | GET | `/users/me` | 取得當前使用者資訊、家庭與偏好。 | Access Token |
| Users | PUT | `/users/me` | 更新個人資料、語系、主題。 | Access Token |
| Families | POST | `/families` | 建立家庭並成為 Giver。 | Access Token |
| Families | POST | `/families/{id}/invite` | 邀請成員加入家庭。 | Giver |
| Wallets | GET | `/wallets` | 列出家庭錢包。 | 家庭成員 |
| Wallets | POST | `/wallets` | 建立錢包。 | Giver |
| Allowances | GET | `/allowances` | 列出 Allowance 設定。 | 家庭成員 |
| Allowances | POST | `/allowances` | 建立或更新發放規則。 | Giver |
| Requests | POST | `/requests` | Baby 建立請款申請。 | Baby |
| Requests | POST | `/requests/{id}/approve` | Giver 核准請款。 | Giver |
| Requests | POST | `/requests/{id}/reject` | Giver 退回請款。 | Giver |
| Transactions | GET | `/transactions` | 篩選交易（支援分頁、分類、期間）。 | 家庭成員 |
| Notifications | GET | `/notifications/poll` | 取得未讀通知。 | Access Token |
| Sync | POST | `/sync/operations` | 上傳離線操作批次。 | Access Token |

API 詳細 Request/Response 模型、錯誤碼與範例需在 `docs/api-outline.md` 中維護。

## 9. 前端 UX 指南
- 導覽架構：登入 → Dashboard（餘額、待辦）→ Wallets → Allowances → Requests → Transactions → Settings。
- 操作須提供成功/失敗即時回饋與 Undo（若可行）。
- 支援桌面與行動響應式版面，主題切換需即時生效。
- 多國語系文字需集中管理，避免硬編碼；測試語系切換後介面仍整齊。
- 可透過 `Pinia` modules 與 `useFetch`（或自訂 `useApi`）確保快取與加載體驗。

## 10. 配置與環境管理
- `.env` 需包含：`DB_DSN`, `DB_ENGINE`, `JWT_SECRET`, `JWT_REFRESH_SECRET`, `JWT_ACCESS_TTL`, `JWT_REFRESH_TTL`, `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `APP_BASE_URL`, `NOTIFICATION_PROVIDER` 等。
- `config/app_config.go` 載入環境變數並提供預設值、驗證與錯誤訊息。
- 使用 `docker-compose.yml` 建立 `api`, `web`, `db` 服務；支援 `dev` 與 `prod` profiles。
- 版本控制：所有 schema 變更需透過 migration 工具（如 `golang-migrate`）。

## 11. 非功能性需求
- **性能**：關鍵 API P95 延遲 < 300ms；支援至少 50 家庭、500 成員、10k 交易/月。
- **可用性**：目標 99% uptime；離線快取確保短暫斷線仍可操作。
- **安全**：遵循 OWASP Top 10，敏感資料加密/遮罩，Token 儲存於 HttpOnly Cookie，部分資訊使用短期記憶。
- **可靠性**：交易操作必須具冪等，採 DB Transaction 與樂觀鎖避免重複扣款。
- **可維護**：模組化、清晰分層、提供 linter/formatter、遵循 Clean Architecture 原則。
- **可觀測**：結構化日誌、Request ID、Prometheus 指標、健康檢查端點。
- **本地化與可及性**：最少支援 zh-TW / en，文案可延展；UI 須符合基本無障礙（鍵盤操作、色彩對比），主題支援高對比模式（dark/light 自訂）。

## 12. 測試策略
- **單元測試**：Repositories（SQLite in-memory）、Services（Mock Repos）。
- **整合測試**：使用 `httptest` 對主要 API 流程建測，含身份驗證、請款審核。
- **契約測試**：Postman Collection、k6 測試對部署環境驗證。
- **E2E 測試**：Playwright/Cypress（登入、建立錢包、發放、請款、審核、儀表板）。
- **可用性測試**：針對多國語系與主題切換進行 UI 覆核。
- **CI**：`go test ./...`, `golangci-lint run`, 前端 `pnpm lint`, `pnpm test`, 部署前 `pnpm build`。
- **資料種子**：`db/seed_users.sql`、`cmd/seed` 供開發與 Demo 使用。

## 13. 部署與 DevOps
- **容器化**：前後端各自 multi-stage Dockerfile；減少映像體積。
- **Compose**：本地開發使用 `docker compose up` 啟動 API、前端、MySQL。
- **CI/CD**：GitHub Actions（或等效）跑測試、建立映像、推送到 Registry，支援 staging/production。
- **監控**：部署 Prometheus/Grafana 或託管方案；串接錯誤追蹤（Sentry/Logtail）。
- **Secrets 管理**：使用部署平台的 secrets store，避免 commit。

## 14. 驗收標準（MVP）
- 能口述主要使用流程與成功條件（Giver 發放、Baby 請款、審核、記帳、通知）。
- `go run ./server` 在 `.env` 完備情況下可成功啟動；`docker compose up` 完整啟動並可操作。
- 前端提供註冊/登入、儀表板、錢包、請款、審核、交易查詢、設定（含語系 & 主題）。
- 核心 API 具測試覆蓋、並提供 Postman/k6 套件；CI pipeline 成功。
- 日誌不洩漏敏感資訊，結構化並可追蹤 request。
- 提供 Demo 資料與操作腳本，可在 Day 30 進行展示。

## 15. 後續擴充（Roadmap）
- 多 Giver/多 Baby 權限組織、家庭共用規則。
- 自動化金流（排程轉帳/提醒）、報表匯出（CSV/PDF）。
- 通知升級（即時推播、Line Notify、Email digest）。
- 家庭任務看板、遊戲化成就、介面客製（自訂顏色）。
- 付費方案（訂閱/買斷）、行銷素材、雲端上架流程。

## 16. 風險與未決事項
- 離線同步實作範圍：MVP 是否實作完整佇列系統或僅儲存草稿。
- 法規限制：若支援真實金流需追加 KYC/AML 檢討。
- 資料庫選型：MSSQL 需求時間表、是否支援多資料庫策略。
- OAuth 整合：Gmail OAuth 之外是否需其他登入方式。
- 多國語系文案維護流程與工具選擇。

## 17. 版本紀錄
| 日期 | 版本 | 說明 |
| :--- | :--- | :--- |
| 2024-11-?? | v0.1 | 初版 SDD（由 `handbookv1/v2` 與需求整合）。 |
| 2024-11-?? | v0.2 | 本次更新：重構為完整 SDD、納入多國語系與三種主題需求。 |

---

請在需求或架構調整後同步更新此文件，確保團隊成員對 Cacao 系統的理解與實作保持一致。

