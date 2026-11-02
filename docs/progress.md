# 專案進度

> 最後更新：2026-11-02

- Current focus: Phase 1 開發 5b：M1 身分完成（首次設定、選擇使用者、PIN、救援碼、孩子密碼、父母模式自動鎖回），單元測試 14 個、Maestro 端對端 2 條全過
- Next action: M2 記帳：孩子總覽、記收入、出金、存款明細、作廢

## Phase 1 階段進度：85%

> 第一版，只做 Android。計算方式：5 個階段平均；開發階段內 5a 基礎佔 20%，M1～M11 各佔約 7.3%。

- 階段 1 產品定義：[##########] 100%
- 階段 2 技術選型：[##########] 100%（D-01 改為純原生，D-16～D-24 定案）
- 階段 3 UI/UX 設計：[##########] 100%（21 個線框、導覽與流程圖；UQ-01～09 定案）
- 階段 4 架構設計：[##########] 100%（`docs/data-model.md`、`docs/auth-design.md`）
- 階段 5 開發實作（Android）：[###-------] 27%（5a 完成、M1 完成）

### 開發清單

5a 基礎
- [x] repo 骨架（android／ios 佔位／shared／tools）
- [x] 共用 schema 接 SQLDelight、資料庫測試
- [x] 計算邏輯（餘額、可出金額、瀑布填充）＋共用測試案例
- [x] 模擬器跑起來
- [x] 主題 token JSON＋codegen（4 個主題、字型跟主題）
- [x] 翻譯來源＋codegen（繁中、英文；預設英文，繁中放 values-b+zh+Hant）
- [x] CI：codegen 檢查、單元測試、編譯（本機已跑通；push 後才會在 GitHub 執行）
- [x] Maestro 端對端測試環境

5b 功能（照線框 `docs/design.md`）
- [x] M1 身分：首次設定、選擇使用者、PIN（Argon2id、輸錯等待、救援碼）、孩子密碼、父母模式自動鎖回（`tools/e2e/m1_identity.yaml`）
- [ ] M2 記帳：孩子列表、總覽、記收入、出金、存款明細、作廢
- [ ] M3 目標：列表、詳情、瀑布填充、確認／退回、目標出金
- [ ] M4 任務：建立（多選孩子、出題人）、完成＋當場驗收
- [ ] M5 常態零用錢與補記
- [ ] M6 錢包（孩子、父母）、成長紀錄
- [ ] M7 設定：主題、語言、音效、家庭成員（含幫孩子重設密碼）
- [ ] M8 備份：匯出／匯入、Google 雲端、30 天自動備份、首次設定的「從備份還原」
- [ ] M9 慶祝動畫與音效
- [ ] M10 截圖測試（每畫面 × 4 主題）＋ Maestro 主要流程
- [ ] M11 上架準備：OAuth 同意畫面、隱私權政策、Play Console、簽名金鑰；人工上架，由 Claude 一步步帶使用者熟悉流程；個人帳號需先封閉測試（12 人 × 14 天）

### 文件與負責 skill

| Task | Status | Skill owner | Notes |
| --- | --- | --- | --- |
| 產品規格 `docs/product/`（overview／features／roadmap） | Done | discover | 正式規格；`SPEC.md` 為歷史初稿 |
| 技術選型 `docs/tech-decisions.md` | Done | tech-research | D-01 改為純原生，D-16～D-24 定案；第一版只實作 Android |
| UI/UX 設計 `docs/design.md` | Done | design | 21 個線框、導覽與流程圖；UQ-01～09 定案 |
| 資料模型 `docs/data-model.md` | Done | data-model | 共用 SQL schema v1、推算規則；DQ-01～13 定案 |
| 認證與備份 `docs/auth-design.md` | Done | auth-design | PIN、救援碼、Google 備份、`.cacao` 格式、換手機還原 |
| 開發實作（Android） | In progress | — | 5a 完成、M1 完成；接著 M2 |
| 進度管理 `docs/progress.md` | Done | progress | 本文件 |

## Phase 2 待辦（下一個迭代，不計入 Phase 1 進度）

### iOS 版

- Xcode 專案（Swift＋SwiftUI，最低 iOS 17），Bundle ID 用 `com.ruru.cacao`
- 資料庫 GRDB，直接執行 `shared/db` 的 SQL
- `tools/codegen` 加上 Swift 輸出：主題 token、String Catalog
- 依線框做完所有畫面（與 Android 同一套）
- 安全：Argon2id 用 libsodium，驗證 Android 產生的 PIN 雜湊
- 測試：單元測試跑 `shared/test-cases`、swift-snapshot-testing（4 主題截圖）、Maestro 同一份流程
- 驗證 Android ↔ iPhone 備份互相還原
- CI 打開 iOS 工作（macOS 機器）
- Apple 開發者帳號（年費 US$99）、TestFlight

### CD 自動上架

- 打版本標籤 → GitHub Actions 編譯簽名 AAB → Google Play 內部測試軌道；正式版仍在 Play Console 手動發布
- 上傳金鑰與服務帳號存 GitHub Secrets
- iOS 版完成後，同樣接到 TestFlight

### 裝置之間同步

- 比較兩條路：
  1. **cr-sqlite**：SQLite 擴充套件，用 CRDT 合併兩台手機的資料；原生可直接載入，缺點是 iOS 要靜態連結、一人維護、改資料表結構要走專用流程
  2. **自己寫**：流水帳依 UUID 聯集＋可修改的資料（目標排序、設定）以最後修改為準
- 傳輸：同一個 Wi-Fi 下手機直連（探索與連線方式待選）；另一個候選是同一個 Google 帳號透過雲端硬碟交換
- 父母在自己的手機上直接核准任務
- Phase 1 已預留：主鍵 UUID、不自動遞增、只新增的流水帳、`updated_at`／`deleted_at`

### 功能

- 進階任務：重複、有期限、可部分完成
- 定期回顧報表完整版（圖表、趨勢）
- 預支／借款（待 PM 評估）

## UI 提案

- 畫布：<https://claude.ai/artifact/XkWynXKhLPrwvBhwCYXcC4>（主題與 Logo）
- 線框：<https://claude.ai/artifact/1hViDqnAG3cSEF4oLbudGg>（21 個畫面）
- **選定主題（4 個）**：可可牛奶、彩色積木、棉花糖、星夜金幣；一台手機一個設定，不綁帳號、不綁孩子
- **選定 Logo**：可可豆存錢筒（字標 Fraunces）

## 技術選型總覽

- **開發框架：純原生**（D-01）：Android Kotlin＋Compose（Phase 1）、iOS Swift＋SwiftUI（Phase 2）；最低 Android 8／iOS 17；單一 repo `android/`、`ios/`、`shared/`、`tools/`（D-17、D-18）
- **資料庫**：SQLite，兩平台共用 `shared/db` 的 SQL；Android SQLDelight、iOS GRDB（D-16）
- **完全不用後端**（D-02）：唯一例外是使用者自己的 Google 雲端硬碟，只用於備份
- **資料模型**：只新增的流水帳；主鍵 UUID
- **身分**：父母 PIN 4 位數（Argon2id）、救援碼、孩子密碼選填；Google 登入只用於備份
- **備份**：手動為主、超過 30 天自動一次、雲端留 7 份、`.cacao` 檔、匯入整個覆蓋
- **主題與翻譯**：共用 JSON → codegen（D-19、D-20）
- **自動化測試**（D-23）：單元、資料庫、截圖、Maestro；共用測試案例保證兩平台計算一致
- **App**：名稱 Cacao、套件 ID `com.ruru.cacao`（D-24）

## Blockers

- 無
