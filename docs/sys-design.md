# Cacao 系統設計藍圖

本文件描述 Cacao MVP 的技術堆疊、模組邊界、資料模型、API 流程與部署策略。所有內容以繁體中文撰寫，並與 `docs/product-guide.md`、`docs/plan-30d.md` 保持同步。

---

## 1. 技術堆疊
| 層級 | 技術 | 說明 |
| --- | --- | --- |
| 後端服務 | Go 1.24、Gin、Wire/DI（預留） | REST API、jobs、共用 domain 模組 |
| 資料庫 | MySQL 8（本地可改 SQLite）、Goose/Migrate | 儲存 families、wallets、allowances、requests、transactions |
| 快取／儲存 | Redis（未來）、MinIO/S3（附件） | MVP 先以本地檔案暫存，規格預留 |
| 行動端 | React Native (Expo SDK 54)、Expo Router、TypeScript、React Query | iOS/Android App，支援 light/dark/high contrast 主題 |
| 通知 | Email/Webhook Stub、Expo Notifications | MVP 以 log 模式運作，後續導入推播與外部服務 |
| DevOps | GitHub Actions、Docker Compose、EAS、Taskfile | CI/CD、環境管理與 build 流程 |

---

## 2. 架構概覽
```
┌──────────────┐       ┌─────────────┐
│  Mobile App  │ <---->│  REST API   │<--> MySQL
│ (Expo Router)│       │ (Go + Gin)  │     │
└──────────────┘       └─────────────┘     │
         ▲                    ▲            │
         │ 通知 webhook/log   │ jobs       ▼
   Expo Notifications     Allowance Runner ──> future queue / notifier
```
- Mobile 只與 REST API 溝通，所有資源皆經 token 驗證。
- Jobs 以 `server/cmd/jobs` 執行，負責 allowance 排程與通知重送。
- `shared/api-schema` 為單一契約來源，產生 TS/Go SDK（待實作）。

---

## 3. 模組邊界
| 模組 | 位置 | 職責 |
| --- | --- | --- |
| `server/cmd/api` | Go binary | 組合 config、DI、啟動 HTTP server |
| `server/internal/platform` | Go package | config、router、server、logger、DB、middleware |
| `server/internal/domain/*` | Go package | 業務邏輯（auth、families、wallets、allowances、requests、transactions） |
| `server/internal/api/*` | Go package | 請求驗證、回應格式（目前只有 auth） |
| `server/internal/jobs` | Go package | Allowance scheduler、job runner、未來 worker |
| `apps/mobile/app` | Expo Router routes | Stack 與 tabs，登入、Dashboard、請款、設定 |
| `apps/mobile/features/*` | Hooks + UI modules | AuthProvider、Requests、Dashboard、Settings 等 |
| `apps/mobile/theme` | ThemeProvider | Light/Dark/High Contrast 主題與配色表 |
| `shared/api-schema` | OpenAPI | Swagger/JSON schema，生成 client SDK |
| `infra/docker-compose` | Docker | MySQL、Mailhog、MinIO、觀察工具 |
| `infra/github` | GitHub Actions | lint/test/build/workflow 檔 |

---

## 4. 資料模型
| 表 | 主要欄位 | 說明 |
| --- | --- | --- |
| `families` | id, name, owner_id, created_at | 家庭基本資料、擁有者 |
| `members` | id, family_id, user_id, role | 家庭與使用者的對應，角色 (giver/baby) |
| `wallets` | id, family_id, name, balance_cents, currency | 錢包餘額，僅允許非負數，未來支援多幣別 |
| `allowances` | id, wallet_id, schedule_type, amount_cents, next_run_at, status | 定期津貼設定與狀態 |
| `requests` | id, wallet_id, requester_id, amount_cents, note, receipt_uri, status, reviewed_by | 請款紀錄與審核資訊 |
| `transactions` | id, wallet_id, request_id, type, amount_cents, balance_after, metadata | 核准請款或手動調整的流水帳 |
| `notifications` | id, type, payload, status, retries | 通知輸出的記錄（MVP 以 log 表示，可延伸） |

索引重點：
- `wallets (family_id, name)` unique；`requests` 依 `wallet_id` + `status` 建索引。
- 需要稽核的欄位（如 `requests.reviewed_by`）必須記錄使用者與 timestamp。

---

## 5. API 摘要
| Method | Path | 功能 | 備註 |
| --- | --- | --- | --- |
| POST | `/api/v1/auth/login` | 使用帳密登入 | 回傳 token + 使用者資訊 |
| GET | `/api/v1/health` | 健康檢查 | 供監控探針使用 |
| POST | `/api/v1/families` | 建立家庭 | MVP 先由單一 Giver 建立 |
| GET | `/api/v1/families/:id/summary` | 家庭概況（錢包、請款） | Dashboard 使用 |
| POST | `/api/v1/wallets` | 建立錢包 | 驗證 balance >= 0 |
| POST | `/api/v1/allowances` | 建立津貼 | 排程資訊寫入 DB |
| POST | `/api/v1/requests` | 建立請款 | 附件暫存 URI |
| GET | `/api/v1/families/:id/requests` | 查詢請款 | 支援狀態篩選 |
| PATCH | `/api/v1/requests/:id/approve` | 核准請款 | 同時寫入 transaction |
| PATCH | `/api/v1/requests/:id/decline` | 駁回請款 | 需要 reason |

所有 API 回應採 `{ data, error }` 格式，錯誤需帶 `code`（例如 `invalid_credentials`）。

---

## 6. 工作流程
### 6.1 登入
1. 客戶端送出帳密至 `/auth/login`。
2. 伺服器驗證後產生 token（目前為 HMAC + random nonce）。
3. 回傳 `token`、`expiresAt` 與 `roles`，前端儲存於 memory（未來加 SecureStore）。

### 6.2 請款核准
1. Baby 建立請款（pending）。
2. Giver 於列表挑選請款 → `approve` API。
3. domain 層檢查餘額、寫入 transaction、更新 request 狀態。
4. Notifier 接收到事件，推送 Email/Log，App 透過 React Query 自動刷新。

### 6.3 Allowance 排程
1. Job runner 每分鐘檢查 `allowances` 是否到期。
2. 針對符合條件的 allowance 建立 transaction + 通知。
3. 更新 `next_run_at`，若失敗寫入 retry 資訊。

---

## 7. 行動 App 設計
- **路由**：`app/_layout.tsx` 建立 Stack（登入、Tabs）。Tabs 含 Dashboard、Requests、Settings。
- **資料層**：React Query 管理 API cache，`services/api.ts` 處理 fetch 與錯誤訊息。
- **狀態**：`features/auth/AuthProvider` 管理 token、錯誤狀態；`theme/ThemeProvider` 讓所有畫面可切換 Light/Dark/High Contrast。
- **UI 指南**：
  - 字體：系統預設 + 16px 基準。
  - 顏色：`theme` 定義背景、文字、邊框、強調色；High Contrast 模式使用黑/白 + 黃色提示。
  - 可及性：所有互動元件需有 44px 點擊區域。

---

## 8. 測試策略
| 層級 | 工具 | 內容 |
| --- | --- | --- |
| 單元測試 | `go test ./internal/...` | Domain 邏輯、validator、jobs |
| API 測試 | Postman/k6、httpexpect | 登入、家庭、請款、核准流程 |
| 行動端 | Expo Jest、Detox (後續) | hooks 與畫面快照、基本導航 |
| 整合測試 | docker compose + k6 | 走完登入→請款→核准 → 通知 |
| 監控 | JSON Log、未來接 DataDog/OTLP | 記錄 request id、latency、error rate |

---

## 9. 部署與環境
- **本機**：`docker compose` 啟動 MySQL、Mailhog、MinIO；`npm run dev:api` / `dev:mobile` 啟動服務。
- **Staging**：GitHub Actions build → deploy 至 Kubernetes/Render（預留）。Secrets 管理於 GitHub。
- **EAS**：`apps/mobile/eas/` 保存憑證與 `eas.json`；build 透過 `eas build --profile preview`。
- **設定**：所有環境變數皆以 `CACAO_*` 前綴，列於 `.env.example`。

---

## 10. 安全與隱私
- 使用 bcrypt（或 scrypt）存放密碼，禁止明文。
- Token 目前為自簽式 HMAC，未來可改 JWT 並加入 refresh token。
- 所有請款附件僅存檔名與預簽 URI，實體由 MinIO/S3 管理。
- 需實作 Rate Limit（Gin middleware）以避免暴力攻擊。
- Log 不得包含敏感字串（密碼、session secret）。

---

## 11. 文件維護
- 系統設計若有修改，請立刻更新本檔並同步 `docs/product-guide.md`、`docs/plan-30d.md`。
- 重大決策請寫入 `docs/adr/`（若未建立請新增資料夾）。
- 在每次週檢視將完成度標記於本檔最後章節，以利審查。

---

## 12. 未來工作
1. 建立 `shared/client-sdk/ts`，讓 Expo 直接引用 typed API。
2. 導入 OAuth / 家長社群登入，減少帳號維護成本。
3. 串接真實推播（APNs/FCM）與郵件服務（SendGrid）。
4. 引入事件匯流排（NATS/Kafka）供通知與審計使用。

任何對架構或資料模型的調整，都必須在 merge 前附上文件 diff，確保團隊同步。
