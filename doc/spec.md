# Cacao 專案規格書

本規格書彙整 `handbookv1.md` 與 `handbookv2.md` 的重點，說明 Cacao 家庭零用錢管理系統的產品願景、核心需求與技術落地指引，作為後續開發與驗收的依據。

## 1. 專案目標與定位
- 打造一套專為家庭（Giver 與 Baby）設計的零用錢/家務獎勵管理 App，協助父母透明發放、控管零用金，並培養孩子紀錄花費的習慣。
- 先以 Android PWA/MVP 驗證市場，聚焦「親子理財教育」小眾市場，待流程穩定後再擴充 iOS 與進階付費功能。
- 主打家庭互動、交易一致性與即時通知，確保線上線下金流紀錄一致、可追溯。

## 2. 角色與利害關係人
- **Giver（父母/監護人）**：設定零用錢規則、審核 Baby 的申請、查看報表、管理支付工具。
- **Baby（孩子）**：提出請款請求、記錄支出、查看餘額與任務狀態、接收通知。
- **系統管理員（內部角色，選配）**：維護系統設定、檢視例外紀錄、管理日誌（若後續引入）。

## 3. 核心使用者旅程
1. Giver 建立家庭帳號並邀請 Baby（或由 Baby 註冊後加入家庭）。
2. Giver 設定零用金規則（定期發放或臨時任務）並選擇錢包/支付方式。
3. Baby 提出請款或紀錄支出，支援現金、轉帳、信用卡代付、直接代購等模式。
4. Giver 審核請求並更新餘額；核准結果通知 Baby。
5. 雙方在時間軸與儀表板查看歷史紀錄、分類統計與趨勢。
6. 系統持續同步本地快取與雲端資料，確保多裝置資料一致。

## 4. MVP 功能需求
- **帳號與角色管理**：支援使用者註冊、登入（ Gmail OAuth2 + 密碼支援 ），角色與家庭綁定、軟刪除與分頁查詢。
- **身份驗證與授權**：使用 JWT + Refresh Token（HttpOnly Cookie），API 須辨識角色權限；未授權請求回傳 401 並支援自動換證。
- **錢包管理**：管理多個錢包（實體錢包、銀行帳戶、信用卡等），紀錄餘額、幣種與餘額上下限，禁止負數。
- **零用金/Allowance 設定**：定義定期發放規則（週/月/自訂），支援臨時獎勵，與錢包連動扣款。
- **請款與審核流程**：Baby 送出請款（含金額、用途、附件），Giver 審核（核准/退回）、紀錄備註與通知。
- **交易與記帳**：紀錄支出/收入、分類、標籤、時間軸呈現，支援附註與回溯修改。
- **儀表板與統計**：展示餘額、近期待辦請求、分類統計、每月趨勢，協助家庭回顧。
- **通知機制**：MVP 以輪詢為主，保留 FCM / Line Notify / WebSocket 擴充接口；關鍵事件需即時提醒。
- **離線支援**：前端使用 SQLite 快取，離線可寫入，恢復連線後與 MySQL 後端雙向同步，以伺服器資料為最終依據。
- **錯誤與回饋**：統一 API 回應格式，清楚顯示錯誤訊息與重試建議。
- **介面國際化與主題**：前端提供多國語系（至少 zh-TW、en，可擴充 JSON/檔案資源）與語言切換控件，並支援三種主題（預設、淺色、深色）；偏好需儲存於使用者設定。
- **文件與啟動流程**：`.env` 驗證、`docs/backend-startup.md`、`docs/api-outline.md`、`docs/product-notes.md` 等文件須與實際行為同步。

## 5. 後續擴充（Post-MVP）功能
- 多 Giver / 多 Baby 家庭關聯與管理者授權分級。
- 自動發錢、批次通知歷史、報表匯出。
- 頭像與媒體上傳、家庭任務看板、遊戲化設計。
- 行動裝置打包（Capacitor → Android APK）與上架流程。
- 付費方案（一次性買斷或家庭共享訂閱）與行銷素材。

## 6. 系統架構概覽
- **前端（Nuxt 3 SPA）**：使用 Pinia 狀態管理、Ant Design Vue 元件、Tailwind CSS、Ionic 模組；配置 `nitro devProxy` 指向 Go API，支援 PWA 與離線封裝。整合 Nuxt i18n（或等效方案）管理多國語系，並建立主題系統（可切換三種 Theme，支援持久化與自訂 token）。
- **行動封裝**：採 Capacitor 封裝為原生容器，設定 `capacitor.config.ts` 指向 `dist`，可進一步包成 APK。
- **後端（Go / Gin）**：RESTful API，採用 GORM、Repository Pattern，分層為 controller/service/repository/model；以 `.env` 管理設定（資料庫、JWT、外部服務）。
- **資料庫**：正式環境使用 MySQL / MSSQL（唯一真實帳本）；前端快取使用 SQLite（可清除）。Repository 介面需支援資料庫抽換。
- **同步策略**：離線操作紀錄序列化，回線後送往後端，由伺服器以原子交易處理，避免重複寫入；伺服器端事件驅動同步回前端。
- **外部整合**：Gmail OAuth2、未來導入 FCM / Line Notify、自動部署管線（GitHub Actions 或等效）。

## 7. 資料模型（核心實體）
- `Users`：包含角色（Giver/Baby）、Email、登入資訊、家庭關聯、軟刪除欄位。
- `Wallets`：對應家庭支付工具，欄位含名稱、類型（現金/帳戶/卡片）、幣別、餘額、擁有者。
- `Allowances`：定期或臨時發放規則，連結 Giver、Baby、Wallet、金額、週期、下一次發放時間。
- `Requests`：Baby 的請款申請，含狀態（pending/approved/rejected）、金額、用途、附件、審核者資訊。
- `Transactions`：所有金流紀錄（發放、支出、退款、調整），需保留來源（Allowance/Request/Manual）、分類與時間戳。
- `AuditLogs`（選配）：關鍵操作審計與除錯；建議納入以支援可觀測性目標。

## 8. API 原則
- 依資源劃分路由：`/auth`, `/users`, `/wallets`, `/allowances`, `/requests`, `/transactions` 等。
- 採 RESTful 命名，GET/POST/PUT/PATCH/DELETE 搭配。
- 所有受保護路由需檢查 JWT 與角色；加入 rate limit 及 request ID。
- 回傳一致的 JSON 格式：`{ success, data, error }`；錯誤含 code 與 message。
- 準備 API 規格文件（`docs/api-outline.md`）與 Postman / k6 測試腳本。

## 9. 配置與環境管理
- `.env` 檔必須包含資料庫連線、JWT 密鑰、OAuth 憑證、外部服務鍵值、Dev/Prod 切換。
- `config/app_config.go`（或等效模組）集中載入環境變數，啟動時給出缺漏提示。
- `docker-compose.yml` 提供本地整套服務（Nuxt、Go API、MySQL），支援 profiles 區分 dev/prod。
- 本地與 CI 應使用同一組初始化腳本與種子資料（`db/seed_users.sql`、`cmd/seed`）。

## 10. 安全性要求
- 登入與敏感 API 套用速率限制、CAPTCHA（可選）與審計；避免暴力攻擊。
- 密碼使用 bcrypt（或 Google 帳號），伺服器端再次校驗角色權限，不依賴前端。
- HTTP Only + Secure Cookie 存放 Refresh Token，Access Token 夾帶於 Authorization header。
- 重要事件（登入失敗、角色變更）寫入審計日誌；日誌不得包含密碼、Access Token。
- 配置 HTTPS、CORS 白名單、CSRF 防護（若需表單操作）。

## 11. 可觀測性與日誌
- 導入結構化日誌（zap/zerolog），包含 request ID、使用者 ID、執行時間。
- 提供 `/health` 或 Prometheus 指標，回傳系統狀態、DB 連線、排程佇列等。
- 前端預留錯誤上報（Sentry 或自建 webhook），捕捉 runtime error 與 API 失敗。

## 12. 測試與品質保證
- 後端單元測試：`repositories` 以 SQLite in-memory 搭配 testify；`services` 覆蓋業務邏輯。
- API 測試：Postman、k6 或等效工具驗證主要流程（登入、發放、請款、記帳）。
- 前端 E2E：Cypress / Playwright 測試登入、請款與核准、儀表板。
- CI 流程：執行 `gofmt`, `golangci-lint`, `go test ./...`，前端 lint 與單元測試。
- 種子資料工具：快速建立 Demo 帳號與樣本資料供展示與開發。

## 13. 部署與交付
- Docker multi-stage 建立獨立映像（Go API、Nuxt 前端），保持體積小、可快速部署。
- `docker compose up` 一鍵啟動前後端與資料庫；Production profile 使用獨立設定與 volume。
- 上雲策略：選擇 Render/Railway/Fly.io 等，建立 image build & push pipeline，記錄於 `docs/deploy-playbook.md`。
- 前端建議使用 Node/Nginx 服務，或透過 CDN/靜態託管（若 API 分離）。

## 14. 效能與可用性
- 關鍵服務需提供 benchmark（Allowance 發放、Transaction 記錄）並記錄在 `docs/performance.md`。
- 進行 k6 壓力測試取得 QPS、延遲基線，設定改善計畫（索引、快取、分頁策略）。
- 設計 DB 索引，避免慢查詢；交易流程採資料庫交易鎖定避免重複扣款。
- 重要操作應提供冪等設計（重試不重複扣款）。

## 15. 驗收標準（MVP）
- 能在 1-2 分鐘內口述系統核心旅程與成功條件，MVP 定義清楚。
- `go run ./server` 僅需 `.env` 即可啟動；`docker compose up` 可啟動完整環境。
- 使用者註冊/登入、錢包管理、零用金設定、請款審核、記帳、通知流程皆可於前後端互動成功。
- 所有關鍵 API 有測試覆蓋、日誌清晰、無敏感資訊洩漏。
- 提供 Demo 素材、操作腳本與版本標籤（如 `v0.1.0-rc1`），可於 Day 30 進行完整展示。

---

此規格書將隨專案進展更新，請於新增功能或調整架構時同步維護，確保團隊成員對需求與實作保持一致認知。
