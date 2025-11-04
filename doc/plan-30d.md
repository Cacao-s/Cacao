# Cacao 30 天行動計畫

此計畫協助在 30 個專注工作日內完成 Cacao MVP，並達到上架 App Store / Google Play 的準備標準。內容引用 `doc/vision.md`、`doc/mvp-spec.md`、`doc/sys-design.md`，請隨進度更新紀錄（建議建立 `LOG.md`）。

---

## 使用方式
- 每日含 **Focus / Key Steps / Done When**。若當日未完成，請移至下一個可行日並記錄阻礙。
- 建議同時開啟三個終端機：Go API、Expo（行動 App）、工具/測試。
- 推播、模擬器與資料庫建置需提前準備，避免卡在環境設定。

---

## 每週主題

| 週次 | 主題 | 核心成果 |
| --- | --- | --- |
| 週 1（Day 1-7） | 產品收斂 + 後端基礎 | 需求確認、資料模型、使用者/家庭 API 穩定 |
| 週 2（Day 8-14） | 金流核心服務 | 錢包、零用金、請款、交易、同步 API 完成並測試 |
| 週 3（Day 15-21） | 行動 App 體驗 | Expo App 具備登入、錢包、請款、通知、離線機制 |
| 週 4（Day 22-30） | 品質、部署、上架 | CI/CD、推播、商店素材、最終 QA 與 Demo |

---

## 第 1 週 — 產品定義與後端基礎

### Day 1　釐清願景與旅程
- **Focus**：確認角色需求與成功畫面。
- **Key Steps**：更新 `doc/vision.md` 內的痛點、旅程、成功指標；列出 Giver/Baby 三大需求；整理主要流程故事板。
- **Done When**：可口述完整旅程並得到認可，文檔同步。

### Day 2　資料模型與 API 藍圖
- **Focus**：設計資料結構與 REST 草稿。
- **Key Steps**：在 `docs/data-model.md` 描繪 ER 圖；撰寫 `docs/api-outline.md` 方法、授權、錯誤碼；紀錄冪等策略。
- **Done When**：資料模型覆蓋全流程，API 無重大缺漏。

### Day 3　環境設定與配置管理
- **Focus**：讓 Go API 透過 `.env` 啟動。
- **Key Steps**：完成 `infra/db/cacaoMysql.go` 環境變數讀取；建立 `config/app_config.go`；撰寫 `docs/backend-startup.md`。
- **Done When**：`go run ./server` 只靠 `.env` 即能連線 MySQL。

### Day 4　使用者與家庭
- **Focus**：建置使用者 CRUD 與家庭關聯。
- **Key Steps**：於 `models/user.go` 補強標籤；`repositories/user_repository.go` 加入查詢、軟刪、分頁；建立家庭邀請 API。
- **Done When**：使用者/家庭 API 測試通過並回傳一致格式。

### Day 5　身份驗證與授權
- **Focus**：完成登入、Token、Middleware。
- **Key Steps**：實作 `services/auth_service.go`（bcrypt、JWT）；建立 `/auth/register/login/refresh/logout`；路由掛載授權與速率限制。
- **Done When**：未帶 Token 的受保護路由回 401，Refresh 可換發 Access Token。

### Day 6　測試與種子資料
- **Focus**：建立測試與預設資料。
- **Key Steps**：撰寫 `repositories/user_repository_test.go`（SQLite in-memory + testify）；新增 `db/seed_users.sql`、`cmd/seed/main.go`。
- **Done When**：`go test ./...` 全數通過，種子資料可建立範例家庭。

### Day 7　週檢視
- **Focus**：同步程式與文件。
- **Key Steps**：執行 `gofmt`, `golangci-lint`；更新 `doc/mvp-spec.md`、`doc/sys-design.md` 狀態；盤點風險與依賴。
- **Done When**：程式、文件一致，下週里程碑明確。

---

## 第 2 週 — 金流核心服務

### Day 8　錢包服務
- **Focus**：建立錢包與餘額調整。
- **Key Steps**：新增 `models/wallet.go`；實作錢包 CRUD、餘額驗證（交易鎖）；撰寫單元測試。
- **Done When**：錢包 API 通過測試，餘額操作安全。

### Day 9　零用金規則
- **Focus**：排程發放與狀態管理。
- **Key Steps**：定義 `allowances` 表格；實作建立/更新/啟停；建立排程（cron）紀錄 log。
- **Done When**：排程與手動觸發都能建立交易，狀態更新正確。

### Day 10　請款流程
- **Focus**：Baby 申請與審核前置。
- **Key Steps**：建立 `requests` 模型；實作建立請款、上傳附件；串接審核 API。
- **Done When**：請款可建立並在 API 中查得，附件流程可運作。

### Day 11　審核與交易
- **Focus**：Giver 審核到交易生成。
- **Key Steps**：完成核准/退回 API，寫入交易表；建置儀表板所需統計（餘額、趨勢、分類）。
- **Done When**：審核會建立交易並更新儀表板資料。

### Day 12　通知事件
- **Focus**：建立通知資料流。
- **Key Steps**：設計 `notifications` 表；實作 `/notifications/poll`、事件寫入；預留推播佇列。
- **Done When**：通知可被查詢並標記已讀，事件資料完整。

### Day 13　離線同步 API
- **Focus**：行動端批次同步機制。
- **Key Steps**：實作 `/sync/operations`，支援 `temp_id` 冪等；撰寫異常測試；定義回傳格式。
- **Done When**：重送不會產生重複交易，錯誤訊息清楚。

### Day 14　週檢視
- **Focus**：整合金流流程。
- **Key Steps**：透過 Postman/k6 走完整流程；更新 `docs/api-outline.md`、`doc/sys-design.md`；整理 QA 清單。
- **Done When**：流程順暢、文件同步、測試工具可重複執行。

---

## 第 3 週 — 行動 App 體驗

### Day 15　Expo 專案骨架
- **Focus**：建立行動專案與路由。
- **Key Steps**：初始化 Expo（TypeScript）；設定 `app.config.ts`；建立 `app/`、`features/`、`stores/` 等目錄；連線後端健康檢查。
- **Done When**：模擬器可啟動並顯示基本頁面。

### Day 16　主題與組件
- **Focus**：完成 Design Token 與共用元件。
- **Key Steps**：建置 ThemeProvider、default/light/dark 主題；建立 Button、Card、ListItem 等元件；通過 Storybook/測試（可選）。
- **Done When**：主題切換即時，元件可於多處重用。

### Day 17　錢包與儀表板
- **Focus**：呈現核心資訊。
- **Key Steps**：串接 `/wallets`, `/transactions/summary`；建立 Dashboard 卡片、趨勢圖；製作錢包列表與新增/編輯表單。
- **Done When**：可在 App 中查看餘額、待辦請款、趨勢。

### Day 18　請款建立與附件
- **Focus**：Baby 請款流程。
- **Key Steps**：實作請款表單（拍照/相簿）；顯示送出與錯誤狀態；離線時寫入 SQLite，提示待同步。
- **Done When**：請款流程順暢，離線草稿可回填。

### Day 19　審核與通知中心
- **Focus**：Giver 審核體驗與推播設定。
- **Key Steps**：建立審核畫面、附件預覽；串接 `/notifications/poll`；註冊裝置 Token，儲存偏好。
- **Done When**：審核即時更新，推播（模擬或真實）可接收。

### Day 20　設定、語系與主題
- **Focus**：完成偏好設定。
- **Key Steps**：整合 i18n；設定語言/主題/通知開關；偏好同步至 `/users/me`。
- **Done When**：App 內可切換語言與主題，偏好跨裝置生效。

### Day 21　週檢視
- **Focus**：整合 App 功能與文件。
- **Key Steps**：走一遍 Giver/Baby 全旅程；更新 `mobile/README.md`；盤點剩餘推播/iOS 設定與問題清單。
- **Done When**：App 功能無阻斷缺口，待辦明確。

---

## 第 4 週 — 品質、部署、上架

### Day 22　安全強化
- **Focus**：補強安全與稽核。
- **Key Steps**：導入速率限制、CORS、錯誤處理；新增 `audit_logs` 寫入；檢查敏感資訊是否留在 log。
- **Done When**：安全檢查清單完成，稽核紀錄可用。

### Day 23　可觀測性
- **Focus**：建立監控與日誌。
- **Key Steps**：整合 zap/zerolog，加入 Request ID；提供 `/health`、Prometheus 指標；行動端導入 Sentry/Expo Error Handler。
- **Done When**：能追蹤一次請求從 App 到 API 的完整流程。

### Day 24　CI/CD 自動化
- **Focus**：讓測試與建置自動化。
- **Key Steps**：GitHub Actions 執行 lint/test/build；設定 Expo EAS Profile；撰寫 `docs/deploy-playbook.md`。
- **Done When**：CI Pipeline 穩定通過，可產出 Android/iOS build。

### Day 25　推播實機測試
- **Focus**：驗證真實裝置通知。
- **Key Steps**：設定 Firebase 專案、APNs key；在實體機測試前景/背景推播；確認通知導向頁面正確。
- **Done When**：通知可在至少一台 iOS、一台 Android 上成功收發。

### Day 26　自動化測試與 QA
- **Focus**：補齊關鍵測試。
- **Key Steps**：撰寫 API 合約測試（Postman/k6）；建立 Detox/Maestro E2E；將測試加入 CI。
- **Done When**：一條指令可驗證核心旅程，測試結果穩定。

### Day 27　效能與負載
- **Focus**：測量與記錄效能。
- **Key Steps**：撰寫 Go benchmark；使用 k6 測試 API；在 `docs/performance.md` 紀錄數據與建議。
- **Done When**：掌握效能基線並列出優化方向。

### Day 28　商店素材與法遵
- **Focus**：準備上架必備文件。
- **Key Steps**：撰寫 Privacy Policy/Terms（`docs/policies/`）；準備圖示、截圖、文案；更新 `README.md` 與 Demo 概述。
- **Done When**：商店素材完整，可提交審核。

### Day 29　最終 QA 與 RC Build
- **Focus**：提交候選版本。
- **Key Steps**：手動跑完整旅程並寫入回報；產出 `v0.1.0-rc1`（TestFlight/Internal Testing）；更新 `doc/sys-design.md` 狀態章。
- **Done When**：RC Build 可供測試者使用，重大問題已關閉。

### Day 30　Demo 與回顧
- **Focus**：展示成果、規劃下一步。
- **Key Steps**：錄製 5 分鐘 Demo；撰寫 `docs/retro.md`（完成、挑戰、下一階段）；整理 Post-MVP backlog。
- **Done When**：Demo 與回顧完成，下一階段目標清楚。

---

## 快速指令速查
```bash
# 後端開發
go run ./server
go test ./...
golangci-lint run

# 行動 App
pnpm expo start --android    # 或 --ios
eas build -p android --profile preview
eas build -p ios --profile preview

# E2E 範例
pnpm detox test              # 或 maestro test scripts/
```

保持紀律並持續迭代，30 天後即可取得具備完整旅程的 Cacao MVP，邁向商店上架。加油！ 💪
