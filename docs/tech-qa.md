# Cacao — 技術選型 Q&A

> **Phase 1 定案文件**：本文件內容 P1 已實作 ✅；標「第二版」的段落為 ⏭ 延到 P2。Phase 2 的修改會另外標示，不覆蓋 P1 結論。

> 用途：`/tech-research` 的提問與回答紀錄。手機上一題一題在對話中確認，由 Claude 寫回本檔。
> 已定案的內容移到「已確認」並追加到 `docs/tech-decisions.md`；「待確認」只留下還沒定案的題目。
> TQ-01～TQ-15 已在 2026-10-05 定案；本檔從 TQ-16 接續。
>
> 最後更新：2026-11-04（Phase 2 題目 TQ-32～TQ-42）

---

# Phase 1（已定案）

## Phase 1：已確認

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

# Phase 2（收斂中）

> 2026-11-04 開始。依 `docs/product/discover-qa.md`〈Phase 2〉與 `docs/product/roadmap.md`〈第二版以後〉，把技術上要決定的事收斂成題目。
> 記號：🚩 牴觸已定案前提（D-02 完全不用後端），要先由使用者決定前提要不要動｜⚠ 需要產品取捨｜❓ 依賴還沒回答的 discover 題目（WD）｜✅ 疑慮低，可照建議定案

## Phase 2：已確認

目前沒有。

## Phase 2：待確認

### A. 前提衝突（先決定，會影響後面所有題目）

#### TQ-32 🚩 不在同一個 Wi-Fi 時，家人之間要不要能互動？

背景：WD-30（⏸ 暫緩）、WD-31 裡的「在公司核准任務」「另一半記帳我馬上看到」「孩子回報、父母手機收到」都要求手機不在一起時也能溝通。D-02 規定不能有任何後端，手機之間沒有中間人可以傳話。

| 方案 | 不在家也能互動 | 牴觸 D-02 | 隱私 | 說明 |
| --- | --- | --- | --- | --- |
| 1. 只在同一個 Wi-Fi 同步 | ✗ | 否 | 最好 | 回家連上 Wi-Fi 才合併；在外面各記各的 |
| 2. 用家人自己的 Google 雲端硬碟當交換站 | △ 有延遲（幾分鐘） | 灰色：沒有我們的伺服器，但依賴 Google Drive 當中繼 | 好：資料在使用者自己的雲端 | 要開共用資料夾給家人，權限比 `drive.appdata` 大；不是即時 |
| 3. 第三方即時後端（Firebase、Supabase） | ✓ 即時 | **是** | 資料放在第三方 | 推播、即時同步都容易做 |
| 4. 自建後端 | ✓ 即時 | **是** | 我們要保管使用者資料 | 要維運伺服器、要錢 |

1. 方案 1：維持 D-02，只做同 Wi-Fi 同步
2. 方案 2：放寬成「可以用使用者自己的雲端硬碟交換」
3. 方案 1＋2 都做：在家即時、在外面透過雲端硬碟慢慢同步
4. 方案 3 或 4：重開 D-02

- 建議：**3**。保住「沒有我們的伺服器、資料在使用者自己手上」這個核心承諾，又能滿足大部分不在家的情境；真正即時（3、4）代價是推翻產品原則。⚠ 這題是產品原則，請你拍板。

回答：

#### TQ-33 🚩 要不要推播通知？

背景：Phase 1 ✖ 不做推播（WD 已確認）。WD-30 選項 6（孩子達成目標、任務待驗證時提醒）需要推播。Android 遠端推播要經過 Google 的 FCM（Firebase），屬於第三方後端。

1. 不做，維持 Phase 1
2. 只做本機提醒：手機自己排程（例如「零用錢今天發了」「超過 30 天沒備份」），不需要任何伺服器
3. 做遠端推播（FCM／APNs）：重開 D-02

- 建議：**2**。「零用錢發了」「該備份了」這類提醒用本機就做得到；「孩子剛回報任務」這種跨手機的即時提醒要等 TQ-32 決定。

回答：

### B. 同步怎麼做（TQ-32 選 1 或 3 才需要）

#### TQ-34 ❓ 同步的合併方式

背景：Phase 1 已照「只新增、不覆蓋」的流水帳、UUID 主鍵設計（roadmap〈預留設計〉✅）。會衝突的只剩可修改的資料（目標排序、零用錢設定、主題以外的設定）。

| 方案 | 維護狀態 | iOS／Android | 改資料表結構 | App 大小 |
| --- | --- | --- | --- | --- |
| cr-sqlite（CRDT 擴充） | 一人主力維護，仍有更新 | 要自己編譯原生擴充；行動版資料少 | 要走專用流程 | 增加原生函式庫 |
| 自己寫：流水帳聯集＋可修改資料以最後修改為準 | 自己維護 | 兩邊照同一份共用規則實作 | 照現有 migration | 幾乎不變 |

1. cr-sqlite
2. 自己寫

- 建議：**2**。我們的資料大部分本來就不會衝突，自己寫的規則簡單、可以放進 `shared/test-cases` 兩平台一起驗；cr-sqlite 寫入慢 2.5 倍、行動版整合資料少。

回答：

#### TQ-35 ❓ 同一個 Wi-Fi 下手機怎麼找到彼此？

| 方案 | Android | iOS | 跨平台 |
| --- | --- | --- | --- |
| mDNS／DNS-SD（Android NSD＋iOS Bonjour） | 內建 | 內建，要宣告區網權限 | 同一套標準，要真機驗證 |
| Google Nearby Connections | 好 | 支援度差 | 弱 |
| Apple Multipeer Connectivity | ✗ | 好 | ✗ |

1. mDNS（NSD＋Bonjour）＋自己的 TCP 連線
2. Nearby Connections
3. 掃 QR code 配對，之後用 mDNS 找人

- 建議：**3**。第一次用 QR code 配對最直覺也最安全（確認是自己家人），之後自動用 mDNS 找到彼此；模擬器測不了區網探索，要準備真機。

回答：

#### TQ-36 ⚠ 新手機怎麼加入家庭？

1. 掃家裡現有手機上的 QR code
2. 從 Google 雲端備份還原後自動加入
3. 兩種都可以

- 建議：**1**。不依賴 Google 帳號；也和 TQ-35 的配對同一個流程。

回答：

### C. 其他 Phase 2 項目（疑慮低）

#### TQ-37 ✅ 目標排序拖曳（只能上下移動）怎麼做？

1. 用 Reorderable 套件（`sh.calvin.reorderable`，支援 LazyColumn，有按住把手才拖的模式）
2. 自己用 Compose 手勢寫

- 建議：**1**。維護中、支援 Compose Multiplatform，按住把手拖動剛好符合「只能上下」；iOS 用 SwiftUI 內建的 `.onMove`。

回答：

#### TQ-38 ✅ CD 自動上架用什麼？

1. Gradle Play Publisher（Gradle 外掛，直接 `publishBundle`）
2. fastlane supply（Ruby，iOS 也能用同一套）

- 建議：**2**。Phase 2 iOS 也要上 TestFlight，fastlane 一套工具兩平台都用；服務帳號金鑰放 GitHub Secrets，GCP 服務帳號照規則寫進 `infra/`。

回答：

#### TQ-39 ❓ 回顧報表的圖表

背景：等 WD-36 決定要看什麼。

1. Android 用 Vico（Compose 圖表套件）、iOS 用內建 Swift Charts
2. 兩邊都自己用 Canvas 畫

- 建議：**2**。報表圖表種類少（長條、圓餅），自己畫可以完全照 4 個主題的風格，兩平台長得一樣；需要的話再換套件。

回答：

#### TQ-40 ✅ 成長卡片、紀念冊分享成圖片／PDF

背景：WD-33、WD-34 的建議選項。不需要任何後端。

1. 畫面轉圖片＋系統分享（Android Sharesheet、iOS ShareLink）；紀念冊用系統 PDF（Android `PdfDocument`、iOS `UIGraphicsPDFRenderer`）
2. 用第三方 PDF 套件

- 建議：**1**。系統內建就夠，不增加 App 大小。

回答：

#### TQ-41 🚩❓ 跨家庭的互動、長輩裝 App

背景：WD-38 選項 4、5（排行榜、交換點子）與 WD-33 選項 3、4（長輩裝 App、長輩直接出任務）需要不同家庭、不同住處的手機互相溝通。

1. 不做（和 D-02 衝突）
2. 只做「分享成圖片」這類單向、不需要連線的版本
3. 等 TQ-32 決定前提後再談

- 建議：**2**。保住前提，又給長輩看得到成果；真正的互動要等 TQ-32。

回答：

#### TQ-42 ⚠❓ 收費要怎麼實作？

背景：WD-39 還沒回答。沒有後端時只能靠 Google Play／App Store 自己驗證購買。

1. 不收費，不做
2. Play Billing／StoreKit 一次買斷，在手機上驗證（不需要伺服器，但比較容易被破解）
3. 訂閱制（官方建議要有伺服器驗證，和 D-02 衝突）

- 建議：**依 WD-39**；若要收費選 **2**。家庭類 App 被破解的風險可以接受，不值得為此加後端。

回答：
