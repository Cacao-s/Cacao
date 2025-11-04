# Cacao 系統設計總覽

本文件提供 Cacao MVP 的技術設計藍圖，涵蓋技術堆疊、架構、模組職責、資料模型、API 規範、部署與測試策略。內容與 `doc/product-guide.md` 保持一致，並聚焦行動 App 首發即上架的要求。

---

## 1. 技術堆疊
| 分層 | 採用技術 | 說明 |
| --- | --- | --- |
| Mobile App | React Native + Expo Router（TypeScript） | 單一程式碼支援 iOS/Android，整合推播、深層連結與環境設定 |
| 狀態管理 | React Query、Zustand、Expo SQLite/MMKV | React Query 負責伺服器狀態，Zustand 管理本地狀態與離線佇列 |
| UI / 主題 | 自訂 ThemeProvider（default / light / dark）、Design Tokens | 確保主題切換、無障礙與一致視覺 |
| 後端 API | Go 1.21 + Gin + GORM | 控制層分離、擴充性佳，支援交易與排程 |
| 認證授權 | Gmail OAuth2、JWT Access/Refresh、bcrypt | 行動端使用 Secure Store/HttpOnly Cookie 儲存憑證 |
| 資料庫 | MySQL 8（正式）、SQLite（測試與 App 緩存） | MySQL 為唯一真實來源；SQLite 只存暫存資料 |
| 推播 | Firebase Cloud Messaging、APNs、Expo Notifications | 提供裝置註冊、偏好設定與推播派送 |
| 日誌/監控 | zap/zerolog、Prometheus、Sentry/Expo EAS | 結構化日誌 + 指標，追蹤請求與排程狀態 |
| CI/CD | GitHub Actions、Expo EAS、Docker | 自動執行 lint/test/build，產出後端映像與 App 包 |

---

## 2. 架構概念
```
┌────────────────────────────────┐
│            Mobile App          │ React Native + Expo
│  - UI (Router / Screens)       │
│  - State (React Query / Store) │
│  - Offline Queue (SQLite)      │
└───────────────┬────────────────┘
                │ HTTPS / JSON + Push
┌───────────────▼────────────────┐
│           API Gateway          │ Gin Router + Middleware
├───────────────┬────────────────┤
│ Controllers   │ Services       │ 驗證、授權、商業邏輯
├───────────────┴────────────────┤
│    Repositories (GORM)         │
└───────────────┬────────────────┘
                │
┌───────────────▼────────────────┐
│            MySQL 8             │ 單一真實來源
└───────────────┬────────────────┘
                │
┌───────────────▼────────────────┐
│     Background Jobs / Queue    │ Allowance 排程、通知派送、同步處理
└───────────────┬────────────────┘
                │
┌───────────────▼────────────────┐
│  FCM / APNs / Expo Push 通道    │
└────────────────────────────────┘
```

---

## 3. 模組責任矩陣
| 模組 | Mobile | Backend | Jobs/Queue |
| --- | --- | --- | --- |
| 身份與家庭 | 角色選擇、偏好設定、邀請流程 | 驗證、家庭關聯、偏好儲存 | 邀請過期通知（延伸） |
| 錢包 | 錢包列表、編輯、餘額視覺化 | 錢包 CRUD、餘額驗證、警戒 | 警戒推播（延伸） |
| 零用金 | 規則列表、啟停、歷史 | 建立/更新規則、餘額檢查 | 排程扣款、建立交易、通知 |
| 請款 | 草稿、附件、送出 | 建立請款、狀態更新 | 補件提醒（延伸） |
| 審核 | 核准/退回、備註 | 權限驗證、交易生成 | 審核提醒（延伸） |
| 交易 | 時間軸、分類、統計 | 交易寫入、查詢、報表資料 | 定期統計（延伸） |
| 通知 | 推播中心、偏好 | 事件建檔、裝置註冊 | 派送推播、重試 |
| 離線同步 | 佇列管理、狀態提示 | `/sync/operations`、冪等處理 | 失敗重播與告警 |

---

## 4. 資料模型摘要
| 表 | 重點欄位 | 說明 |
| --- | --- | --- |
| `users` | email, password_hash, google_sub, display_name, locale, theme, role, status | 使用者主檔，含系統角色（giver/baby/admin）與偏好 |
| `families` | name, currency, timezone, created_by | 家庭設定，與成員透過 `family_members` 關聯 |
| `family_members` | family_id, user_id, family_role, invited_by, joined_at | 家庭內角色（giver/baby/...），支援邀請流程 |
| `wallets` | family_id, name, type, balance, currency, warning_threshold, status | 錢包餘額由交易加總維持一致 |
| `allowances` | family_id, giver_member_id, receiver_member_id, wallet_id, amount, frequency, next_run_at, status | 定期/臨時發放規則與排程資訊 |
| `requests` | family_id, requester_member_id, wallet_id, amount, category, notes, attachment_url, status, decision_by, decision_at, rejection_reason | 請款記錄，狀態包含 draft/pending/approved/rejected/cancelled |
| `transactions` | family_id, wallet_id, type, amount, source_type, source_id, category, occurred_at, notes | 金流紀錄，`type` 分 credit/debit |
| `notifications` | user_id, event_type, payload, delivery_status, read_at | 推播與 App 內提醒的統一來源 |
| `sync_queue` | device_id, user_id, operation_type, payload, temp_id, status, retries, last_error | 行動端離線操作上傳後的狀態追蹤 |
| `audit_logs` | actor_id, family_id, action, resource_type, resource_id, metadata, created_at | 安全稽核、客服查詢用 |

詳細 Schema 與 Migration 由 `docs/data-model.md` 維護。

---

## 5. API 規範
- **路由分組**：`/auth`, `/users`, `/families`, `/wallets`, `/allowances`, `/requests`, `/transactions`, `/notifications`, `/sync`, `/health`, `/metrics`。
- **回應格式**：`{ success: boolean, data?: T, error?: { code, message, details? } }`。
- **授權**：使用 `Authorization: Bearer <AccessToken>`；Refresh Token 透過 HttpOnly Cookie 或 Secure Store 搭配 `/auth/refresh`。
- **冪等性**：`Idempotency-Key` header 運用於會改變狀態的請求（請款、同步、發放）。
- **附件上傳**：採預簽 URL（S3/GCS/R2）或 `multipart/form-data`，限制檔案大小與格式（例如 ≤5 MB, jpg/png/pdf）。
- **錯誤碼**：統一於 `docs/api-outline.md` 定義，包含身份失效、資源不存在、餘額不足、冪等衝突等案例。

Middleware 順序：Request ID → Logger → Recovery → Rate Limit → CORS → JWT 驗證 → 授權。

---

## 6. Mobile App 設計
- **專案結構**：`app/`（路由）、`features/`（領域模組）、`components/`、`stores/`、`services/`、`hooks/`、`i18n/`、`theme/`。
- **狀態流程**：React Query 連線 API → 成功後寫入 cache，並同步至必要畫面；離線時讀取 SQLite 緩存。
- **離線機制**：Zustand `offlineQueue` + SQLite，記錄操作與 `temp_id`；`useSync` hook 監聽網路狀態，自動批次呼叫 `/sync/operations`。
- **推播**：`expo-notifications` 註冊裝置 Token → 傳至 `/notifications/devices`；點擊推播透過 deep link 導向對應畫面。
- **主題/語系**：ThemeProvider 搭配 Design Token，支援 default/light/dark；`react-i18next` 或 Expo Localization 管理 zh-TW/en 文案。
- **測試**：Jest + React Native Testing Library（單元/UI），Detox 或 Maestro（E2E）。

---

## 7. 後端設計
- **專案結構**：`main.go` 啟動 → `config/` 載入環境 → `routes/` 註冊路由 → `controllers/` 處理 HTTP → `services/` 實作商業邏輯 → `repositories/` 存取資料庫 → `models/` 定義實體 → `jobs/` 處理排程與佇列 → `infra/` 整合外部服務。
- **服務層重點**：
  - `AuthService`：註冊、登入、OAuth、Token 發行/刷新、登出。
  - `FamilyService`：建立家庭、邀請流程、角色權限驗證。
  - `WalletService`：錢包 CRUD、餘額調整（使用 DB 交易）、警戒通知。
  - `AllowanceService`：規則建立、排程、餘額檢查、交易產生。
  - `RequestService`：草稿、提交、審核、附件管理。
  - `TransactionService`：交易寫入、查詢、統計資料計算。
  - `NotificationService`：裝置註冊、事件建檔、推播派送、重試。
  - `SyncService`：接收離線批次、冪等判斷、寫入交易。
- **排程/佇列**：使用 `robfig/cron` 或 `go-co-op/gocron` 執行 Allowance 發放、通知重送；必要時以資料庫鎖或 Redis 確保單一執行。

---

## 8. 安全與合規
- 密碼採 bcrypt（cost ≥ 12），登入啟用速率限制。
- JWT Access Token 約 15 分鐘，Refresh Token 14 天，可設定黑名單或版本號失效。
- 重要操作（登入、角色變更、餘額調整）寫入 `audit_logs`。
- Log 避免紀錄個資、Token；預設 INFO 等級，支援可調整。
- CORS 限制在官方網域與行動裝置使用的 scheme；所有 API 走 HTTPS。
- 準備隱私政策、服務條款、權限聲明，符合 Apple/Google 審核。

---

## 9. 部署與環境
- **開發**：Docker Compose 啟動 MySQL；Expo 連線 `http://localhost:8080`；環境變數由 `.env` 管理。
- **Staging**：部署於雲端（Render/Railway/Fly.io 等），整合 HTTPS、備份與監控；Expo EAS 產出測試 build（Android Internal / TestFlight）。
- **Production**：獨立資料庫與應用程式服務，設自動備援與監控告警；App 發布正式版。
- **CI/CD**：GitHub Actions 執行 lint/test → 建置 Docker 映像 → 觸發 EAS Build（預覽/正式）；部署腳本記錄於 `docs/deploy-playbook.md`。

---

## 10. 測試策略
- **後端**：`go test ./...`（單元/服務/整合）、`golangci-lint`、Postman/k6 合約與負載測試。
- **行動端**：Jest（邏輯/元件）、React Native Testing Library（UI）、Detox/Maestro（關鍵旅程）、Expo E2E Preview。
- **手動 QA**：依 `doc/plan-30d.md` 中的週檢視與最終 QA 清單執行。
- **覆蓋率目標**：後端核心模組 ≥ 70%，行動端關鍵模組 > 50%，E2E 覆蓋登入、請款、審核、通知。

---

## 11. 版本與維護
| 版本 | 日期 | 重點 |
| --- | --- | --- |
| v0.1 | 2024-10 | Nuxt + Go 初版架構（已停用） |
| v0.2 | 2024-11 | 切換 React Native + Expo，調整整體設計 | 
| v0.3 | ＜待定＞ | MVP 落地後更新實作細節與監控數據 |

若架構或技術決策有重大變動，請先更新本文件，再同步 `doc/product-guide.md` 與 `doc/plan-30d.md`，確保全體成員對系統狀態保持一致。
