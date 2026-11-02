# 專案進度

> 最後更新：2026-11-02
> 這份只追蹤階段與進度；內容細節見下方〈文件〉。

- Current focus: Phase 1 開發，M5 常態零用錢完成
- Next action: M6 錢包、成長紀錄

## Phase 1 階段進度：91%

> 第一版，只做 Android。5 個階段平均；開發階段內 5a 基礎佔 20%，M1～M11 各約 7.3%。

- 階段 1 產品定義：[##########] 100%（`docs/product/`）
- 階段 2 技術選型：[##########] 100%（`docs/tech-decisions.md`）
- 階段 3 UI/UX 設計：[##########] 100%（`docs/design.md`）
- 階段 4 架構設計：[##########] 100%（`docs/data-model.md`、`docs/auth-design.md`）
- 階段 5 開發實作：[######----] 56%（`android/`、`shared/`、`tools/`）

### Phase 1 開發追蹤

- [x] 5a 基礎：repo 骨架、共用 schema、計算邏輯＋共用案例、主題、翻譯、CI、Maestro
- [x] M1 身分
- [x] M2 記帳
- [x] M3 目標
- [x] M4 任務
- [x] M5 常態零用錢
- [ ] M6 錢包、成長紀錄
- [ ] M7 設定
- [ ] M8 備份
- [ ] M9 慶祝動畫與音效
- [ ] M10 截圖測試＋主要流程
- [ ] M11 上架準備（人工上架）

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
- [ ] 預支／借款（待 PM 評估）

## 文件

### Phase 1

- 產品規格（建立）：`docs/product/`（overview／features／roadmap）
- 技術決策（建立）：`docs/tech-decisions.md`
- UI/UX 設計（建立）：`docs/design.md`（線框 <https://claude.ai/artifact/1hViDqnAG3cSEF4oLbudGg>）
- 資料模型（建立）：`docs/data-model.md`
- 身分與備份（建立）：`docs/auth-design.md`

### Phase 2

- 產品規格（修改）：`docs/product/`，範圍見 roadmap〈第二版以後〉
- 技術決策（修改）：`docs/tech-decisions.md`，加入裝置同步方案、iOS 細節、CD
- UI/UX 設計（修改）：`docs/design.md`，加入同步、進階任務、報表的畫面
- 資料模型（修改）：`docs/data-model.md`，加入同步需要的欄位與規則
- 身分與備份（修改）：`docs/auth-design.md`，加入 iOS 與跨平台還原

## Blockers

- 無
