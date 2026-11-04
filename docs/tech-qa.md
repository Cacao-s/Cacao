# Cacao — 技術選型 Q&A

> **Phase 1 定案文件**：本文件內容 P1 已實作 ✅；標「第二版」的段落為 ⏭ 延到 P2。Phase 2 的修改會另外標示，不覆蓋 P1 結論。

> 用途：`/tech-research` 的提問與回答紀錄。手機上一題一題在對話中確認，由 Claude 寫回本檔。
> 已定案的內容移到「已確認」並追加到 `docs/tech-decisions.md`；「待確認」只留下還沒定案的題目。
> TQ-01～TQ-15 已在 2026-10-05 定案；本檔從 TQ-16 接續。
>
> 最後更新：2026-11-01

---

## 已確認

2026-11-01 使用者決定 D-01 改為純原生（Swift＋Kotlin）並規劃自動化測試；TQ-16～TQ-31 由使用者授權「照建議全部定案」。細節見 `tech-decisions.md`。

### 跨平台資料

- **Android 與 iPhone 的備份可以互相還原**（TQ-16）：兩邊共用 `shared/db/` 同一份 schema 與 migration SQL。→ D-16
- **Android 資料庫用 SQLDelight**（TQ-17）、**iOS 用 GRDB**（TQ-18）：都是 SQL 優先，能直接用共用 SQL 檔。→ D-16

### 介面與架構

- **Jetpack Compose**（TQ-19）、**SwiftUI**（TQ-20）。→ D-17
- **最低支援 Android 8（API 26）／iOS 17**（TQ-21）。→ D-17
- **單一 repo**：`android/`、`ios/`（Phase 2，先佔位）、`shared/`（共用資料）、`tools/`（codegen、端對端、CI 腳本）（TQ-22，2026-11-01 補上 `tools/`）。→ D-18
- **主題用一份 design token JSON 產生 Kotlin／Swift**（TQ-23）。→ D-19
- **翻譯用一份來源產生 strings.xml 與 String Catalog**（TQ-24）。→ D-20
- **狀態管理用各平台官方做法**：ViewModel＋StateFlow、`@Observable`（TQ-25）。→ D-21
- **動畫兩邊都用 Lottie，音效用系統內建**（TQ-26）。→ D-22

### 自動化測試

- **四層都做**：單元、資料庫、截圖（每個畫面 × 4 個主題）、端對端（只涵蓋主要流程）（TQ-27）。→ D-23
- **共用測試案例**：`shared/test-cases/` 的 JSON 兩邊單元測試都跑，保證計算結果一致（TQ-28）。→ D-23
- **端對端用 Maestro**（TQ-29）、**截圖用 Paparazzi＋swift-snapshot-testing**（TQ-30；2026-11-02 Android 改用 Roborazzi，見 D-23）。→ D-23
- **CI 用 GitHub Actions**，iOS 截圖與端對端只在合併前跑（TQ-31）。→ D-23

---

## 待確認

（目前沒有待確認的題目）
