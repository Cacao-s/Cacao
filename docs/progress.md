# 專案進度

> 最後更新：2026-11-01

- Current focus: 開發 5a：Android 專案骨架完成（共用 schema 接 SQLDelight、計算邏輯＋7 個共用測試案例、DB 測試），模擬器 poc34 跑起來
- Next action: 5a 剩下：主題 token＋codegen、翻譯來源、CI；接著 5b 畫面實作
- Progress: [########--] 82%（五個階段平均）

## 階段進度

- 階段 1 產品定義：[##########] 100%
- 階段 2 技術選型：[##########] 100%（2026-11-01 D-01 改為純原生 Swift＋Kotlin，D-16～D-23 定案，含自動化測試）
- 階段 3 UI/UX 設計：[##########] 100%（21 個線框、導覽與流程圖、交給下一階段清單完成；UQ-01～09 定案）
- 階段 4 架構設計：[##########] 100%（`docs/data-model.md`、`docs/auth-design.md` 定稿）
- 階段 5 開發實作（第一版只做 Android）：[#---------] 10%（Android 專案可編譯、單元測試通過、模擬器跑起來）
  - 5a repo 骨架（`android/`、`ios/` 佔位、`shared/` 資料、`tools/` 工具）＋ codegen ＋ CI
  - 5b Android 實作＋自動化測試到通
- Phase 2（下一個迭代，不計入第一版進度）：iOS 版，共用 `shared/`；驗證 Android ↔ iPhone 備份互相還原

| Task | Status | Skill owner | Notes |
| --- | --- | --- | --- |
| 產品規格初稿 `SPEC.md` | Done | — | 技術規格已隨決策更新 |
| 產品探索問答 `docs/product/discover-qa.md` | Done | discover | 待確認已清空（WD-29 語言：預設跟系統、設定可切換） |
| 產品規格 `docs/product/`（overview／features／roadmap） | Done | discover | 正式規格；`SPEC.md` 降為歷史初稿 |
| UI 風格與 Logo | Done | — | 主題選定 4 個（手機層級設定切換）、Logo 選定「可可豆存錢筒」 |
| 技術選型 `docs/tech-decisions.md` | Done | tech-research | D-01 改為純原生，D-16～D-23 定案（含自動化測試）；第一版只實作 Android，iOS 在 Phase 2 |
| UI/UX 設計 `docs/design.md` | Done | design | 21 個線框、導覽與流程圖、交給 `/data-model`／`/auth-design` 的清單；UQ-01～09 定案 |
| 資料模型 `docs/data-model.md` | Done | data-model | 共用 SQL schema v1（8 張表）、餘額／瀑布填充／零用錢補記／成長紀錄推算規則、第二版同步預留；DQ-13 指定的錢不能一般出金（使用者決定）；其餘 4 項 AI 決定見文件末 |
| 認證與備份 `docs/auth-design.md` | Done | auth-design | PIN（驗證點、輸錯等待、救援碼、自動鎖回）、Google 授權 drive.appdata、手動＋30 天自動備份、雲端留 7 份、`.cacao` 備份檔格式、匯入覆蓋、換手機還原；5 項 AI 決定見文件末；線框 S01 待補「從備份還原」 |
| 開發實作（Android） | Pending | — | 依賴階段 1–4；先做 `shared/` 與 CI，再做 Android；高保真視覺邊開發邊調 |
| iOS 版 | Phase 2 | — | 下一個迭代；共用 `shared/`，不計入第一版進度 |
| 進度管理 `docs/progress.md` | Done | progress | 本文件 |

## UI 提案

- 畫布：<https://claude.ai/artifact/XkWynXKhLPrwvBhwCYXcC4>
- **選定主題（4 個）**：A 可可牛奶、B 彩色積木、F 棉花糖、D 星夜金幣。未採用：C 晨霧藍綠、E 黑白大字
- **選定 Logo**：第一組 ① 可可豆存錢筒（可可棕底、奶茶色豆莢、投幣孔、金幣；字標 Fraunces）。延伸 6 款未採用
- 主題用法：一台手機一個設定，在設定裡切換，不綁帳號、不綁孩子

## 技術選型總覽

### 已定案

- **開發框架：純原生 Swift＋Kotlin**（`docs/tech-decisions.md` D-01，2026-11-01 由 Flutter 改定）：Compose／SwiftUI、最低 Android 8／iOS 17、單一 repo（D-17、D-18）
- **原生版技術選型（D-16～D-23）**：共用 schema SQL＋SQLDelight／GRDB（備份可跨平台還原）、共用 design token 與翻譯來源自動產生兩邊程式碼、官方狀態管理、Lottie＋系統音效
- **自動化測試（D-23）**：單元、資料庫、截圖（每畫面 × 4 主題）、端對端 Maestro；共用 JSON 測試案例保證兩平台計算一致；GitHub Actions
- **完全不用後端**（D-02）：不自建，也不依賴任何第三方後端；唯一例外是使用者自己的 Google 雲端硬碟，只用於備份
- **資料庫引擎：SQLite**（手機本機）
- **資料模型原則**：只新增、不覆蓋的流水帳；主鍵用 UUID、不用自動遞增、主鍵不可為空
- **登入**：只有 Google 登入，用途是雲端硬碟備份；不做帳號密碼登入。沒登入也能用 App
- **身分區分**：父母驗證碼（PIN），每位父母一組
- **備份**：Google 雲端硬碟（第一版必做）＋匯出／匯入備份檔
- **裝置模式**：第一版全家共用一台手機，不做同步
- **主題**：4 個主題，用 design token 實作；一台手機一個設定，不屬於家庭資料，也不進備份
- **多語系（D-20、D-14、D-15）**：第一版繁體中文＋英文；預設跟系統、設定可切換；英文內文用系統字型；App 自動產生的紀錄名稱存類型代碼
- **與框架無關的決策（D-04～D-08、D-11）**：本機不加密、雲端備份放 `appDataFolder`（不加密）、備份檔是整個 SQLite 檔、匯出檔可選擇設密碼、驗證碼 Argon2id 雜湊存資料庫、英文數字字型打包＋中文系統字型

### 第二版再決定

- **裝置同步方案**：cr-sqlite，或自己寫「流水帳聯集＋可修改資料以最後修改為準」（細節見下方「別忘了」）
- **同步傳輸**：同一個 Wi-Fi 下手機直連的探索與連線方式

## ⚠ 別忘了：第二版待辦

- **裝置同步要比較兩條路**（第一版不做，但資料表設計要保留選擇權）：
  1. **cr-sqlite**：SQLite 擴充套件，用 CRDT 合併兩台手機的資料。原生可直接載入擴充套件；缺點是 iOS 要靜態連結、一人維護、改資料表結構要走專用流程。
  2. **自己寫**：流水帳聯集（依 UUID 合併）＋少數可修改的資料（目標排序、設定）以最後修改為準。
  - 傳輸方式兩條路都要自己寫：同一個 Wi-Fi 下手機直連。
  - 另一個候選：同一個 Google 帳號的手機透過雲端硬碟交換資料（要求父母共用帳號）。
  - 第一版的資料表要先符合：主鍵用 UUID、不用自動遞增、主鍵不可為空。
- 進階任務（重複、有期限、部分完成）
- 定期回顧報表完整版（第一版只做簡單版的成長紀錄）
- 父母在自己的手機上直接核准任務（需要裝置同步）

## Blockers

- 無
