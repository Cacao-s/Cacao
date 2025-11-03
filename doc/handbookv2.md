# Cacao 30 天實作手冊

這份文件是專屬 Amanda 的行動計畫，用 30 個專注工作日將「家庭零用錢管理系統」從概念打造到 MVP。每天依序完成當日任務，保持紀錄（建議使用 `LOG.md` 或筆記 App），遇到新發現就即時補充，下一遍回顧會更輕鬆。

---

## 使用說明
- 按日執行；若進度延後，就把該項移到下一個可行的工作日，務必完成再往下。
- 每天包含 **Focus（今天的重點）**、**Key Steps（具體步驟）**、**Done When（完成判斷）**。
- 預設專案路徑為 `Cacao/Cacao`，包含 Go 後端與 Nuxt 前端框架；不再安排基礎 Git/SQL 教學，而是直接進入功能開發。
- 建議同時開三個終端機：Go API、Nuxt 前端、Docker/工具；保持伺服器快速重啟。
- 卡住或改動計畫時，請在當日區塊下方記錄原因，方便後續檢討。

---

## 每週主題

| 週次 | 目標主題 | 核心成果 |
| ---- | -------- | -------- |
| 第 1 週（Day 1-7） | 產品定義 + 後端基礎 | 明確的角色需求、資料模型與穩定的使用者 API |
| 第 2 週（Day 8-14） | 金流流程建立 | 錢包、交易、請款、定期發放的完整服務 |
| 第 3 週（Day 15-21） | Nuxt 介面體驗 | 登入/註冊、儀表板、Pinia 狀態與時間軸呈現 |
| 第 4 週（Day 22-30） | 安全、部署、驗證 | 安全性、觀測性、Docker、部署與 Demo |

---

## 每日任務

### Day 1 — 收斂產品目標
- **Focus**：釐清 Giver 與 Baby 的痛點與成功畫面。
- **Key Steps**：
  - 在 `docs/product-notes.md`（不存在就建立）列出雙方的前三大需求。
  - 描述主要旅程：Giver 設定零用錢 → Baby 申請 → Giver 核准 → Baby 記帳。
  - 撰寫 MVP 驗收條件（不超過 8 行要點）。
- **Done When**：能不看文件就口述完整旅程與 MVP 成功條件。

### Day 2 — 資料與 API 藍圖
- **Focus**：把旅程轉換成資料表與接口需求。
- **Key Steps**：
  - 以 Markdown 表格或 ER 圖工具畫出 `Users`、`Wallets`、`Allowances`、`Requests`、`Transactions` 的關係。
  - 在 `docs/api-outline.md` 梳理 REST API，依資源分組。
  - 標示每個 API 是否需要授權。
- **Done When**：資料結構與 API 列表涵蓋 Day 1 全流程，沒有遺漏。

### Day 3 — 設定與資料庫啟動
- **Focus**：讓後端只靠環境變數即可啟動。
- **Key Steps**：
  - 更新 `infra/db/cacaoMysql.go`：完全使用 `os.Getenv` 讀取設定，並提供易讀的錯誤訊息。
  - 建立 `config/app_config.go`（或類似檔案）集中管理環境設定，在 `server/main.go` 載入。
  - 把啟動流程與指令寫進 `docs/backend-startup.md`。
- **Done When**：`go run ./server` 只需 `.env` 即成功，無硬編碼憑證。

### Day 4 — 優化使用者領域
- **Focus**：讓使用者 CRUD 可實戰使用。
- **Key Steps**：
  - 在 `models/user.go` 加上 `binding` 驗證、JSON 欄位配置。
  - 擴充 repository（依 email 查詢、軟刪除、分頁）。
  - 重構 `controllers/user_controller.go`，統一回傳格式與錯誤處理。
- **Done When**：POST/PUT/DELETE 使用者時皆有預期行為與訊息。

### Day 5 — 登入與身份驗證
- **Focus**：建立登入機制與保護路由。
- **Key Steps**：
  - 新增 `services/auth_service.go` 處理密碼雜湊與比對（bcrypt）。
  - 實作 `/auth/login`，回傳 JWT；於 `routes/router.go` 加上授權 middleware。
  - 將 JWT 秘鑰、存活時間寫入環境變數。
- **Done When**：未帶 token 的請求會回 401，登入可取得有效 JWT。

### Day 6 — 測試與預設資料
- **Focus**：為使用者功能建立測試與種子資料。
- **Key Steps**：
  - 撰寫 `repositories/user_repository_test.go`，使用 SQLite in-memory + testify。
  - 建立 `db/seed_users.sql` 作為快速導入資料。
  - 提供 `cmd/seed/main.go` 或 Makefile 目標自動執行種子資料。
- **Done When**：`go test ./repositories` 全數通過且種子工具可建立基本帳號。

### Day 7 — 週檢視與調整
- **Focus**：整理一週成果並校正後續計畫。
- **Key Steps**：
  - 執行 `gofmt`, `golangci-lint`（或可用工具），清理依賴。
  - 更新 `product-notes`、`api-outline`、`backend-startup` 等文件內容。
  - 檢查接下來的排程是否需要調整、標註風險。
- **Done When**：程式碼乾淨、文件同步、接續計畫確認。

---

### Day 8 — 錢包模型與 API
- **Focus**：建立錢包結構以追蹤餘額。
- **Key Steps**：
  - 新增 `models/wallet.go`，定義金額型別與外鍵約束。
  - 建立 repository/service/controller，提供 `/api/wallets` CRUD 與餘額調整。
  - 調整餘額時使用 GORM 交易鎖住流程，避免競態。
- **Done When**：錢包 API 能建立、查詢、調整餘額，並遵守業務規則（如禁止負數）。

### Day 9 — 交易紀錄引擎
- **Focus**：任何金錢變動都要留下交易紀錄。
- **Key Steps**：
  - 新增 `models/transaction.go`，包含交易種類（allowance/request/spend/correction）。
  - 建立 `RecordTransaction` 服務：同時更新錢包與寫入交易（需交易保護）。
  - 暴露 `/api/transactions` GET，支援篩選與排序。
- **Done When**：每次餘額變動都寫入交易表，餘額與紀錄保持一致。

### Day 10 — 定期發放資料結構
- **Focus**：為 Allowance 排程做準備。
- **Key Steps**：
  - 建立 `models/allowance.go`：金額、頻率、下次執行時間、時區等欄位。
  - 實作 `/api/allowances` CRUD，限制 Giver 管理自己帳戶。
  - 先完成資料儲存，排程邏輯稍後實作。
- **Done When**：Allowance 可被管理且欄位驗證完整。

### Day 11 — Baby 請款流程
- **Focus**：Baby 能申請額外零用錢。
- **Key Steps**：
  - 建立 `models/request.go`，狀態含 pending/approved/rejected/paid。
  - 實作 `/api/requests`：Baby 建立、Giver 審核與更新狀態。
  - 核准後觸發交易與錢包更新。
- **Done When**：請款 API 完整支援申請、審核、狀態與金額異動。

### Day 12 — 時間軸資料準備
- **Focus**：提供前端統一的活動串。
- **Key Steps**：
  - 建立服務 `GetRecentActivity(userID)`，合併交易與請款資訊。
  - 新增 `/api/activity` 回傳活動列表（類型、標題、時間、關聯 ID）。
  - 加上分頁或游標參數，方便前端無限滾動。
- **Done When**：前端可直接使用 `/api/activity` 呈現時間軸，不必再組合資料。

### Day 13 — 定期發放排程（初版）
- **Focus**：讓 Allowance 自動發放。
- **Key Steps**：
  - 實作 goroutine + ticker，定期檢查 `Allowances.next_run`。
  - 若到期則執行交易、更新錢包與 `next_run`。
  - 詳細記錄執行情況與錯誤（log）。
- **Done When**：啟動伺服器後，允許排程自動執行至少一次發放。

### Day 14 — 後端整合測試
- **Focus**：確保關鍵金流流程可回歸測試。
- **Key Steps**：
  - 建立 `tests/api_allowance_test.go`，涵蓋發放流程。
  - 建立 `tests/api_request_test.go`，涵蓋請款核准流程。
  - 將測試整合進 CI（GitHub Actions 或腳本）。
- **Done When**：`go test ./tests` 成功，金流流程可一鍵驗證。

---

### Day 15 — Nuxt 專案基礎
- **Focus**：確認前端架構與 API 對接方式。
- **Key Steps**：
  - 檢查 `cacaoweb/` 結構，整理 `stores/`、`components/`、`pages/`。
  - 在 `nuxt.config.ts` 設定 `runtimeConfig`，從 `.env` 讀取 API base URL。
  - 建立全域 API 工具（Axios 或 `useFetch` 包裝），加入 token 攜帶策略。
- **Done When**：前端可啟動並以硬編碼 JWT 成功呼叫受保護 API。

### Day 16 — 認證介面流程
- **Focus**：完成登入與註冊畫面。
- **Key Steps**：
  - 建立 `/auth/login.vue`、`/auth/register.vue`，使用 Ant Design Vue 表單元件。
  - 連接後端登入 / 註冊 API，成功後將 JWT 存入 Pinia + 永續化插件。
  - 實作路由守門員，未登入導向 `/auth/login`。
- **Done When**：使用者可透過 UI 自助註冊並登入系統。

### Day 17 — 版型與導覽架構
- **Focus**：建立共用佈局與角色導向導覽。
- **Key Steps**：
  - 建立 `layouts/dashboard.vue`（側邊欄 + 頂部工具列）。
  - 依角色顯示不同選單項目。
  - 實作路由層級 middleware，限制角色訪問權限。
- **Done When**：登入不同角色會看到專屬選單與頁面限制。

### Day 18 — Giver 儀表板
- **Focus**：提供 Giver 一目了然的資訊與操作。
- **Key Steps**：
  - 建立 `/giver/dashboard.vue`，呈現錢包摘要、待審請款、下次發放。
  - 整合 `/api/activity` 以時間軸顯示。
  - 加入快速操作（核准請款、立即發放）。
- **Done When**：Giver 儀表板能載入真實資料並觸發 API。

### Day 19 — Baby 儀表板
- **Focus**：讓 Baby 隨時掌握餘額與申請狀態。
- **Key Steps**：
  - 建立 `/baby/dashboard.vue`，顯示餘額、定期發放、最近支出。
  - 加入請款對話框，連結 `/api/requests`。
  - 顯示請款狀態（待審、已核准等）。
- **Done When**：Baby 可從前端送出請款並看到狀態更新。

### Day 20 — 共用元件與狀態強化
- **Focus**：降低重工並管理資料更新。
- **Key Steps**：
  - 抽出 `MoneyCard`、`TimelineList`、`RequestTable` 等共用組件。
  - 在 Pinia 建立同步動作，記錄快取時間避免重複請求。
  - 製作全域錯誤處理（通知/Toast）與 token 失效處理。
- **Done When**：狀態同步穩定，UI 有一致的錯誤提示。

### Day 21 — 使用性與體驗優化
- **Focus**：讓介面感覺完整、流暢。
- **Key Steps**：
  - 檢查色彩對比、按鈕語意、鍵盤操作（Ant Design 基礎上微調）。
  - 加入載入狀態與骨架畫面。
  - 在 `docs/ui-decisions.md` 紀錄設計規範與元件選擇。
- **Done When**：主要流程有適當的 Loading、Error、Empty 狀態，操作順暢。

---

### Day 22 — 安全性檢查
- **Focus**：全面強化安全措施。
- **Key Steps**：
  - 引入速率限制 middleware（例如 gin-contrib/limiter），保護登入 API。
  - 在 service 層再次確認角色與權限，不只依賴 controller。
  - 檢查 log，移除敏感資訊；統一錯誤格式。
- **Done When**：`docs/security.md` 列出已完成的安全項目與待辦。

### Day 23 — 可觀測性
- **Focus**：便於診斷與監控。
- **Key Steps**：
  - 導入結構化日誌（zap/zerolog），支援 request ID。
  - 加上基本指標（Prometheus exporter 或 `/health` 回傳統計）。
  - 前端接入 Sentry（或預留 hook）捕捉錯誤。
- **Done When**：可以追蹤一次 API 呼叫的全流程並查詢健康指標。

### Day 24 — Docker Compose 統整
- **Focus**：一鍵啟動完整環境。
- **Key Steps**：
  - 撰寫 Go API Dockerfile（multi-stage build），並在 `docker-compose.yml` 加入服務。
  - 撰寫 Nuxt Dockerfile（build + serve via Node/Nginx），加入 compose。
  - 使用 docker network 與共享 env，確保服務互通。
- **Done When**：`docker compose up` 可同時啟動前後端與 MySQL，前端可正常使用 API。

### Day 25 — 部署演練
- **Focus**：準備上雲部署。
- **Key Steps**：
  - 選擇部署平台（Render、Railway、Fly.io、DO...），記錄需求。
  - 建立部署腳本或 GitHub Actions，完成 image build/push。
  - 在本機模擬 production（使用 compose prod profile 或相似配置）。
- **Done When**：`docs/deploy-playbook.md` 清楚描述部署流程與設定。

### Day 26 — 自動化 QA
- **Focus**：確保關鍵流程有測試保護。
- **Key Steps**：
  - 撰寫 API 合約測試（Postman Collection/k6），對準部署環境。
  - 建立前端 E2E 測試（Playwright/Cypress），涵蓋登入、發放、請款。
  - 把 QA 流程加入 CI。
- **Done When**：一個指令即可驗證前後端關鍵旅程。

### Day 27 — 效能與負載觀察
- **Focus**：知道系統的可承受度。
- **Key Steps**：
  - 撰寫 Go benchmark（Allowance、RecordTransaction）。
  - 使用 k6 進行 API 壓力測試，記錄 QPS、延遲。
  - 在 `docs/performance.md` 紀錄結果與優化建議（索引、快取等）。
- **Done When**：掌握效能基線數據與下一步優化方向。

### Day 28 — 資源與溝通素材
- **Focus**：為 Demo 與分享準備素材。
- **Key Steps**：
  - 截圖、流程圖存放於 `docs/media/`。
  - 草擬 README 摘要或簡介頁文案（服務價值、功能亮點）。
  - 規劃未來使用者導覽或教學腳本。
- **Done When**：有一套可對外展示的素材與說明。

### Day 29 — 最終 QA
- **Focus**：正式 Demo 前最後確認。
- **Key Steps**：
  - 手動走一遍核心流程：建立帳號、登入、設定發放、建立/核准請款、記錄支出。
  - 監看 log，確認沒有警告或錯誤。
  - Tag 版本（例如 `v0.1.0-rc1`）並推送。
- **Done When**：手動測試全數成功，版本標籤建立。

### Day 30 — Demo 與回顧
- **Focus**：展示成果並整理學習。
- **Key Steps**：
  - 錄製 5 分鐘操作影片（螢幕錄影 + 旁白）。
  - 在 `docs/retro.md` 寫回顧：完成事項、遇到的挑戰、下一版重點。
- **Done When**：影片與回顧完成，列出下一階段待辦並慶祝成果！

---

## 快速指令參考

```powershell
# 啟動後端（開發）
go run ./server

# 執行 Go 測試
go test ./...

# 啟動 Nuxt（開發）
cd cacaoweb
pnpm dev

# 一鍵啟動 Docker 服務
docker compose up
```

保持紀律、每天完成一小步，30 天後就能擁有屬於自己的 Cacao MVP。加油，Amanda！💪
