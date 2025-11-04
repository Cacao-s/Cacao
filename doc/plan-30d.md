# Cacao 30 天行動計畫

此計畫協助在 30 個專注工作日內，從零完成可上架的 Cacao MVP。所有任務皆以 `doc/product-guide.md` 與 `doc/sys-design.md` 為準繩，請於每日完成後更新 `LOG.md` 以追蹤實際進度。

---

## 使用方式
- 每日任務皆包含 **Focus / Key Steps / Done When**；若未完成，請移至下一個可行日並於 `LOG.md` 記錄原因。
- 建議同時啟動三個終端機：Go API、Expo（行動 App）、工具/測試環境；推播、模擬器與資料庫需提前設定。
- 當任務要求更新文件時，請同步維護 `doc/product-guide.md`、`doc/sys-design.md` 或計畫中指定的其他檔案。

---

## 每週主題與成果

| 週次 | 主題 | 本週目標摘要 |
| --- | --- | --- |
| 週 1（Day 1-7） | 產品收斂 + 後端基礎 | 明確需求、完成資料模型，建置可運行的使用者/家庭 API。 |
| 週 2（Day 8-14） | 金流核心服務 | 完成錢包、零用金、請款、交易與同步 API，並建立測試。 |
| 週 3（Day 15-21） | 行動 App 體驗 | Expo App 具備登入、錢包、請款、通知、離線機制並同步文件。 |
| 週 4（Day 22-30） | 品質、部署、上架 | 強化安全與觀測、完成 CI/CD、推播實測、商店素材與 Demo。 |

---

## 第 1 週 — 產品定義與後端基礎

### Day 1　釐清願景與旅程
- **Focus**：確認角色需求與成功畫面。
- **Key Steps**：閱讀並更新 `doc/product-guide.md` 的痛點、旅程、成功指標；列出 Giver/Baby 三大需求；整理主要流程故事板並記錄於 `LOG.md`。
- **Done When**：能口述完整旅程與前提，`doc/product-guide.md` 與 `LOG.md` 已同步調整。

### Day 2　資料模型與 API 藍圖
- **Focus**：設計資料結構與 REST 草稿。
- **Key Steps**：在 `docs/data-model.md` 製作 ER 圖；撰寫 `docs/api-outline.md` 的資源、授權、錯誤碼；補充冪等策略並記錄依賴。
- **Done When**：資料模型覆蓋全流程且經檢視無缺漏，`docs/api-outline.md` 已更新。

### Day 3　環境設定與配置管理
- **Focus**：讓 Go API 透過 `.env` 啟動。
- **Key Steps**：改寫 `infra/db/cacaoMysql.go` 使用環境變數；建立 `config/app_config.go` 讀取設定；完成 `docs/backend-startup.md`。
- **Done When**：`go run ./server` 只靠 `.env` 即能連線 MySQL，文件同步。

### Day 4　使用者與家庭
- **Focus**：建置使用者 CRUD 與家庭關聯。
- **Key Steps**：補強 `models/user.go` 驗證與 JSON 標籤；擴充 `repositories/user_repository.go` 查詢、軟刪、分頁；新增家庭邀請 API。
- **Done When**：使用者/家庭 API 測試通過並回傳一致格式，`doc/sys-design.md` 已標註最新流程。

### Day 5　身份驗證與授權
- **Focus**：完成登入、Token、Middleware。
- **Key Steps**：實作 `services/auth_service.go`（bcrypt、JWT Access/Refresh）；建立 `/auth/register/login/refresh/logout`；路由掛載授權與速率限制。
- **Done When**：未帶 Token 的受保護路由回 401，Refresh 成功換發 Access Token。

### Day 6　測試與種子資料
- **Focus**：建立測試與預設資料。
- **Key Steps**：撰寫 `repositories/user_repository_test.go`（SQLite in-memory + testify）；新增 `db/seed_users.sql`、`cmd/seed/main.go`；更新 `LOG.md` 記錄測試涵蓋率。
- **Done When**：`go test ./...` 全數通過，種子工具可建立範例家庭。

### Day 7　週檢視
- **Focus**：同步程式與文件、盤點風險。
- **Key Steps**：執行 `gofmt`、`golangci-lint`；檢查尚未更新的文件並調整 `doc/product-guide.md`、`doc/sys-design.md`；於 `LOG.md` 記錄風險與下週重點。
- **Done When**：程式碼與文件一致，下週目標明確，風險列表已建立。

---

## 第 2 週 — 金流核心服務

### Day 8　錢包服務
- **Focus**：建立錢包與餘額調整。
- **Key Steps**：新增 `models/wallet.go`；實作錢包 CRUD 與餘額驗證（交易鎖）；撰寫單元測試並更新 `doc/sys-design.md`。
- **Done When**：錢包 API 通過測試，餘額操作安全。

### Day 9　零用金規則
- **Focus**：排程發放與狀態管理。
- **Key Steps**：定義 `allowances` 資料表與模型；實作建立/更新/啟停 API；設定排程（cron）並記錄 Log。
- **Done When**：排程與手動觸發皆能建立交易，狀態更新正確。

### Day 10　請款流程
- **Focus**：Baby 申請與附件。
- **Key Steps**：建立 `requests` 模型；實作請款建立、附件上傳（預簽 URL）；補齊授權檢查與錯誤格式。
- **Done When**：請款可建立並成功上傳附件，API 返回一致結果。

### Day 11　審核與交易
- **Focus**：Giver 審核與交易生成。
- **Key Steps**：完成核准/退回 API；自動建立交易記錄與餘額變更；更新儀表板統計欄位。
- **Done When**：審核會同步建立交易並更新儀表板資料。

### Day 12　通知事件
- **Focus**：建立通知資料流。
- **Key Steps**：設計 `notifications` 表；實作 `/notifications/poll`、事件寫入；規劃推播佇列與重試機制。
- **Done When**：通知可查詢、標記已讀，事件資料完整。

### Day 13　離線同步 API
- **Focus**：行動端批次同步機制。
- **Key Steps**：實作 `/sync/operations` 支援 `temp_id` 冪等；撰寫異常測試；定義回傳格式與錯誤碼。
- **Done When**：離線操作重送不會產生重複交易，錯誤訊息清楚。

### Day 14　週檢視
- **Focus**：整合金流流程與 QA。
- **Key Steps**：透過 Postman/k6 走完整流程；更新 `docs/api-outline.md`、`doc/sys-design.md`；整理 QA 清單並記錄於 `LOG.md`。
- **Done When**：流程順暢、文件同步、測試工具可重複執行。

---

## 第 3 週 — 行動 App 體驗

### Day 15　Expo 專案骨架
- **Focus**：建立行動專案與路由。
- **Key Steps**：初始化 Expo（TypeScript）；設定 `app.config.ts` 與環境變數；建立 `app/`、`features/`、`stores/`、`services/` 等結構；連線後端健康檢查。
- **Done When**：模擬器可啟動並顯示基本頁面。

### Day 16　主題與共用元件
- **Focus**：完成 Design Token 與基礎組件。
- **Key Steps**：建置 ThemeProvider、default/light/dark 主題；建立 Button、Card、ListItem 等元件；撰寫故事或單元測試（可選）。
- **Done When**：主題切換即時，元件可在多處重用。

### Day 17　錢包與儀表板畫面
- **Focus**：呈現核心資訊。
- **Key Steps**：串接 `/wallets`、`/transactions/summary`；建立 Dashboard 卡片、趨勢圖；完成錢包列表與建立/編輯表單。
- **Done When**：App 可顯示餘額、待辦請款與趨勢圖表。

### Day 18　請款建立與離線草稿
- **Focus**：Baby 請款流程。
- **Key Steps**：完成請款表單（拍照/相簿）；離線時寫入 SQLite（或 MMKV）並提示待同步；錯誤狀態需明確。
- **Done When**：請款流程順暢，離線草稿可回填並同步。

### Day 19　審核與通知中心
- **Focus**：Giver 審核體驗與推播設定。
- **Key Steps**：建立審核頁面、附件預覽；串接 `/notifications/poll`；註冊裝置 Token，保存於後端。
- **Done When**：審核即時更新，推播（模擬或真實）可接收。

### Day 20　設定、語系與主題偏好
- **Focus**：完成個人與家庭設定。
- **Key Steps**：整合 i18n；提供語言/主題/通知偏好切換；偏好同步到 `/users/me` 並儲存在 Secure Store。
- **Done When**：App 可切換語言與主題，偏好跨裝置生效。

### Day 21　週檢視
- **Focus**：整合 App 功能與文件。
- **Key Steps**：走一次 Giver/Baby 完整旅程；更新 `mobile/README.md`、`doc/sys-design.md`；盤點尚未完成的推播/iOS 設定並記錄於 `LOG.md`。
- **Done When**：App 功能無阻斷缺口，待辦清單明確。

---

## 第 4 週 — 品質、部署、上架

### Day 22　安全與稽核
- **Focus**：補強安全與稽核機制。
- **Key Steps**：導入速率限制、CORS、錯誤處理；新增 `audit_logs` 寫入；檢查 log 是否洩漏敏感資訊；更新 `doc/sys-design.md` 安全章節。
- **Done When**：安全檢查清單完成，稽核紀錄可查。

### Day 23　可觀測性
- **Focus**：建立監控與日誌。
- **Key Steps**：整合 zap/zerolog，加入 Request ID；提供 `/health` 與 Prometheus 指標；行動端導入 Sentry/Expo Error Handler。
- **Done When**：能追蹤一次請求從 App 到 API 的完整流程。

### Day 24　CI/CD 自動化
- **Focus**：讓測試與建置自動化。
- **Key Steps**：設定 GitHub Actions 執行 lint/test/build；配置 Expo EAS Profile；撰寫 `docs/deploy-playbook.md`。
- **Done When**：CI Pipeline 穩定通過，可產出 Android/iOS build。

### Day 25　推播實機測試
- **Focus**：驗證真實裝置通知。
- **Key Steps**：設定 Firebase 專案、APNs key；在實體 Android/iOS 測試前景/背景推播；紀錄通知導向行為與錯誤。
- **Done When**：通知可在至少一台 iOS 與一台 Android 成功收發。

### Day 26　自動化測試與 QA
- **Focus**：補齊關鍵測試。
- **Key Steps**：撰寫 API 合約測試（Postman/k6）；建立 Detox/Maestro E2E 流程；把測試整合入 CI。
- **Done When**：一條指令即可驗證核心旅程，測試結果穩定。

### Day 27　效能與負載
- **Focus**：測量與記錄效能。
- **Key Steps**：撰寫 Go benchmark；使用 k6 測試 API；在 `docs/performance.md` 紀錄數據與優化建議。
- **Done When**：掌握效能基線並列出改善方向。

### Day 28　商店素材與法遵
- **Focus**：準備上架所需文件。
- **Key Steps**：撰寫 Privacy Policy/Terms（`docs/policies/`）；準備圖示、螢幕截圖、商店文案；更新 `README.md` 與 Demo 指引。
- **Done When**：商店素材齊備，可進入審核流程。

### Day 29　最終 QA 與 RC Build
- **Focus**：產出候選版本。
- **Key Steps**：手動跑完整旅程並記錄問題；產出 `v0.1.0-rc1`（TestFlight/Internal Testing）；更新 `doc/sys-design.md` 實作狀態。
- **Done When**：RC Build 可提供測試者使用，重大問題已關閉。

### Day 30　Demo 與回顧
- **Focus**：展示成果、規劃下一步。
- **Key Steps**：錄製 5 分鐘 Demo；撰寫 `docs/retro.md`（完成、挑戰、下一階段）；整理 Post-MVP backlog 與建議。
- **Done When**：Demo 與回顧完成，下一階段目標清楚且文件齊備。

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
