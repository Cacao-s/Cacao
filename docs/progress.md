# 專案進度

> 最後更新：2026-11-04
> 這份只追蹤階段與進度；內容細節見下方〈文件〉。

- Current focus: M11 上架準備；Play 帳號驗證全部完成，可以建 App
- Next action: 👤 收集測試者 Gmail；🤖 補 OAuth 品牌頁與測試使用者後實測雲端備份

## Phase 1 階段進度：99%

> 第一版，只做 Android。5 個階段平均；開發階段內 5a 基礎佔 20%，M1～M11 各約 7.3%。

- 階段 1 產品定義：[##########] 100%（`docs/product/`）
- 階段 2 技術選型：[##########] 100%（`docs/tech-decisions.md`）
- 階段 3 UI/UX 設計：[##########] 100%（`docs/design.md`）
- 階段 4 架構設計：[##########] 100%（`docs/data-model.md`、`docs/auth-design.md`）
- 階段 5 開發實作：[##########] 97%（`android/`、`shared/`、`tools/`）

### Phase 1 開發追蹤

- [x] 5a 基礎：repo 骨架、共用 schema、計算邏輯＋共用案例、主題、翻譯、CI、Maestro
- [x] M1 身分
- [x] M2 記帳
- [x] M3 目標
- [x] M4 任務
- [x] M5 常態零用錢
- [x] M6 錢包、成長紀錄
- [x] M7 設定
- [x] M8 備份（匯出／匯入已驗證；Google 雲端程式完成，待 M11 設好 OAuth 後在有 Google Play 的裝置驗證）
- [x] M9 慶祝動畫與音效
- [x] M10 截圖測試＋主要流程（Roborazzi 12 張基準圖；Maestro 9 條流程，`tools/scripts/e2e.ps1`）
- [ ] M11 上架準備（人工上架，約 50%；`docs/release-guide.md`、`docs/store/`、`docs/privacy/`、`infra/`）
  - 已完成
    - [x] release 版：R8 縮小（E2E 9/9 通過）、上傳金鑰簽章、AAB 已產生
    - [x] App 圖示：④ 發芽的可可豆
    - [x] 商店素材：中英文案、512 圖示、主題圖、中英截圖各 6 張
    - [x] 隱私權政策上線：https://cacao-s.github.io/Cacao/privacy/
    - [x] 決策：個人帳戶、包含 13 歲以下、聯絡信箱 amanda1991910@gmail.com
    - [x] Play 開發者帳號：已付款、身分文件已送審（2026-11-04）
    - [x] GCP：專案 `cacao-ruru-app`＋Drive API（Terraform `infra/gcp`）、OAuth 同意畫面、Android 用戶端 debug／upload 兩組
    - [x] 工具：gcloud、Terraform、有 Google Play 的模擬器 `play34`（已登入 Google）
  - 待辦（依順序）
    - [x] 👤 Google 身分驗證通過（2026-11-04）
    - [x] 👤 簡訊驗證聯絡電話；聲明交易商身分：非交易商（2026-11-05；公開姓名、信箱、國家）
    - [ ] 🤖 OAuth 品牌頁補隱私權網址、授權網域 `cacao-s.github.io`；加測試使用者（11/04 Console 輸入無反應，改天再試）
    - [ ] 🤖 雲端備份實測：開啟、立即備份、從 Google 還原（目前 403，等測試使用者加好）
    - [ ] 🤖 Play Console 建 App、上傳 AAB、填商店資訊／資料安全性／內容分級／目標對象
    - [ ] 🤖 加第三組 OAuth 用戶端（Play 簽署金鑰 SHA-1）、OAuth 發布為正式版
    - [ ] 👤 收集 15 位以上測試者 Gmail（Android 手機）
    - [ ] 🤖 建封閉測試、匯入名單、送審；👤 傳邀請連結給測試者
    - [ ] 👤 封閉測試連續 14 天（12 人以上不退出）
    - [ ] 👤 申請正式版、送審、發布

## Phase 2 階段進度：0%

> 下一個迭代，不計入 Phase 1。範圍見 `docs/product/roadmap.md`〈第二版以後〉。

- 階段 1 範圍定義：[----------] 0%（`docs/product/`）
- 階段 2 技術選型：[----------] 0%（`docs/tech-decisions.md`）
- 階段 3 UI/UX 設計：[----------] 0%（`docs/design.md`）
- 階段 4 架構設計：[----------] 0%（`docs/data-model.md`、`docs/auth-design.md`）
- 階段 5 開發實作：[----------] 0%（`ios/`、`android/`、`shared/`、`tools/`）

### Phase 2 開發追蹤

- [ ] iOS 版
- [ ] CD 自動上架
- [ ] 裝置之間同步
- [ ] 進階任務
- [ ] 回顧報表完整版
- [ ] 優化：目標排序改拖曳
- [ ] 預支／借款（待 PM 評估）

## 文件

### Phase 1

- 產品規格（建立）：`docs/product/`（overview／features／roadmap）
- 技術決策（建立）：`docs/tech-decisions.md`
- UI/UX 設計（建立）：`docs/design.md`（線框 <https://claude.ai/artifact/1hViDqnAG3cSEF4oLbudGg>）
- 資料模型（建立）：`docs/data-model.md`
- 身分與備份（建立）：`docs/auth-design.md`
- 上架步驟（建立）：`docs/release-guide.md`
- 商店素材（建立）：`docs/store/`；隱私權政策（建立）：`docs/privacy/`
- 專案連結（建立）：`docs/links.md`
- 雲端資源（建立）：`infra/`（Terraform＋指令操作紀錄）

### Phase 2

- 產品規格（修改）：`docs/product/`，範圍見 roadmap〈第二版以後〉
- 技術決策（修改）：`docs/tech-decisions.md`，加入裝置同步方案、iOS 細節、CD
- UI/UX 設計（修改）：`docs/design.md`，加入同步、進階任務、報表的畫面
- 資料模型（修改）：`docs/data-model.md`，加入同步需要的欄位與規則
- 身分與備份（修改）：`docs/auth-design.md`，加入 iOS 與跨平台還原

## Blockers

- 無（Play 帳號已完成所有驗證，可以建 App）
- GCP Console OAuth 頁面輸入無反應：卡測試使用者 → 雲端備份實測
