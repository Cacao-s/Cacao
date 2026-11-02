# 技術決策

> 已定案的決策記錄在這裡，`/tech-research` 不再重新比較。
> 最後更新：2026-11-01（D-01 改為原生，D-16～D-23 定案）

> **2026-11-01 D-01 改為純原生（Swift＋Kotlin）**。依附 Flutter 的決策已由新決策取代：
> D-03 → D-16、D-09 → D-21、D-10 → D-19、D-12 → D-22、D-13 → D-20。新增 D-17 介面、D-18 repo 結構、D-23 自動化測試。
> D-04～D-08、D-11、D-14、D-15 的決策內容與框架無關，照舊有效；其中提到 Drift 的實作細節改由 D-16 承接。
>
> **第一版只實作 Android**（2026-11-01，使用者決定）：D-16～D-23 中 iOS 的部分（GRDB、SwiftUI、iOS 17、swift-snapshot-testing、macOS CI、跨平台還原測試）照規劃保留，在 Phase 2 才實作。第一版要做的是 `shared/` 與 Android。

---

## D-01 開發框架：純原生（Swift＋Kotlin）

**狀態**：已定案（2026-11-01，使用者決定；原本 2026-10-05 選 Flutter）

### 改變的理由（2026-11-01）

- 使用者比較 Flutter、Rust 核心＋原生介面、純原生之後，決定採用純原生：App 最小、操作手感與系統一致、系統新功能最快可用。
- 接受的代價：4 個主題與所有畫面要在兩個平台各做一次，商業邏輯（帳本、瀑布填充、餘額）也要寫兩次。
- 對策：規劃自動化測試，用兩個平台共用的測試案例確保計算結果一致（見 `docs/tech-qa.md`）。

### 原本的決策（2026-10-05，已被取代）

### 決策背景

- 要同時支援 iOS 與 Android
- App 體積要小
- 要能精準做出自訂的 UI 風格（見 UI 風格提案的三個主題）

### 方案比較

| | Flutter | React Native（Expo） | 原生 Swift + Kotlin |
|---|---|---|---|
| 多平台 | 單一程式碼 | 單一程式碼 | 兩套程式碼 |
| 自訂風格還原 | 自繪引擎，兩平台一致 | 依賴原生元件，陰影與字體在兩平台常有差異 | 可以，但要做兩次 |
| App 體積 | 小 | 與 Flutter 相近 | 最小 |

### 最終選擇與理由

選 **Flutter**。三個條件裡，自訂風格還原度是唯一明顯拉開差距的一項；多平台與 App 體積兩者相近。

### 影響

- `SPEC.md` 原本指定的 **WatermelonDB 只支援 React Native，已取消**。本地資料庫改在 Flutter 生態中選擇，待 `/tech-research` 決定。

### 風險評估

- App 體積的主要風險在中文字型，而不是框架：繁中字型每個字重可能好幾 MB。對策是中文用系統字型，只打包英文數字字型。

---

## D-02 完全不用後端

**狀態**：已定案（2026-10-05）

### 決策

不自建後端，也**不依賴任何第三方後端**（Supabase、Firebase、PowerSync 等雲端服務都不用）。所有資料只存在手機本機。

### 影響

- 裝置之間沒有伺服器可以交換資料。家人之間怎麼共用資料，見 `docs/product/discover-qa.md` WD-20。
- 「帳號密碼登入」沒有伺服器可以驗證，需要重新定義，見 WD-22。
- **例外：Google 登入＋Google 雲端硬碟備份第一版必做**，只當作使用者自己的備份空間，核心功能不依賴它。
- 本地資料庫候選排除 PowerSync（它依賴自己的雲端服務）。
- 若之後要做裝置之間的同步，只能走裝置直連（例如同一個 Wi-Fi 下互傳）。資料模型建議從第一版就設計成「只新增、不覆蓋」的流水帳，之後加同步才不用重構。

### 風險評估

- 手機遺失或壞掉、又沒有備份時，資料會全部消失。所以 Google 雲端硬碟備份列為第一版必做。

---

## D-03 本地資料庫套件：Drift

**狀態**：已被 D-16 取代（2026-11-01，D-01 改為原生）；原定案 2026-10-05，TQ-01

### 決策背景

流水帳要經常加總、篩選，資料結構會隨版本演進，還要支援整份資料庫的備份與還原。

### 方案比較

| | Drift | sqflite | sqlite3（直接用） |
| --- | --- | --- | --- |
| Schema migration | 內建，有遷移測試工具 | 自己寫 SQL | 自己寫 SQL |
| 型別安全 | 編譯期檢查 | 無 | 無 |
| 監聽資料變化 | 內建 | 無 | 無 |
| 之後要加密 | 不用換套件 | 要另找套件 | 不用換套件 |

### 最終選擇與理由

選 **Drift**。型別安全與監聽查詢（餘額變動，畫面自動更新）省下大量工作；之後要加密也不用換套件。

### 風險評估

- 需要程式碼產生（build_runner），開發流程多一步。

---

## D-04 本機資料庫第一版不加密

**狀態**：已定案（2026-10-05，TQ-02）

### 決策背景

「隱私優先」原則下，評估本機資料庫是否加密。

### 最終選擇與理由

**第一版不加密**。手機系統本來就隔離 App 資料並有全機加密，對零用錢資料已經足夠；加密的金鑰存在系統安全儲存區，重裝 App 就讀不到資料，徒增風險。

### 風險評估

- 之後若要加密，Drift 2.32 起可改用 SQLite3MultipleCiphers，不用換套件；既有資料需要一次性轉換。

---

## D-05 雲端備份位置：Google 雲端硬碟 appDataFolder

**狀態**：已定案（2026-10-05，TQ-03）

### 最終選擇與理由

- 套件：`google_sign_in`＋`googleapis`（Drive v3），以 `extension_google_sign_in_as_googleapis_auth` 銜接。
- 位置：**`appDataFolder`**，使用者在雲端硬碟裡看不到，只有 Cacao 讀得到；權限只需要 `drive.appdata`。
- 新版 `google_sign_in` 把登入和授權拆開，在「開啟雲端備份」時才請使用者授權。

### 風險評估

- 使用者無法從雲端硬碟直接下載備份；需要自己保存檔案時，改用匯出備份檔。

---

## D-06 備份檔格式：整個 SQLite 資料庫檔

**狀態**：已定案（2026-10-05，TQ-04）

### 方案比較

| | 整個 SQLite 資料庫檔 | JSON 匯出 |
| --- | --- | --- |
| 轉換程式 | 不需要 | 要寫匯出與匯入 |
| 舊版備份升級 | 由 Drift migration 處理 | 要自己處理 |

### 最終選擇與理由

選 **整個 SQLite 資料庫檔**。匯出、上傳、下載還原、匯入共用同一套流程：

- **產生備份**：`VACUUM INTO` 產生完整快照（App 執行中也能做，避開 `-wal` 造成的不完整），再壓縮成一個檔案。
- **匯出**：`file_picker`／`share_plus`。
- **上傳**：Drive API 上傳到 `appDataFolder`。
- **下載還原／匯入**：檢查 `PRAGMA integrity_check` 與 `PRAGMA user_version` → 關閉資料庫 → 取代檔案 → 重新開啟，舊版備份由 migration 自動升級。

### 風險評估

- 取代前一定要先關閉資料庫。
- 備份版本比 App 新時要擋下，提示先更新 App。
- 匯入是覆蓋還是合併，由 `/auth-design` 設計（建議第一版整份覆蓋）。

---

## D-07 備份加密：雲端不加密，匯出檔可選擇設密碼

**狀態**：已定案（2026-10-05，TQ-05、TQ-06）

### 方案比較

| | 不加密 | 用密碼加密（AES-256-GCM＋Argon2id） |
| --- | --- | --- |
| 換手機還原 | 登入 Google 就能還原 | 還要輸入備份密碼 |
| 忘記密碼 | 不影響 | 備份永遠打不開（沒有伺服器可以重設） |
| 檔案外流 | 內容可被讀取 | 看不到內容 |

### 最終選擇與理由

- **雲端備份不加密**：已經在使用者自己的 Google 帳號與 `appDataFolder` 裡，外人存取不到；「忘記密碼就救不回來」對家長的風險比資料外洩更大。
- **匯出的備份檔：匯出時讓使用者選擇要不要設密碼**。匯出檔可能被傳來傳去，給想保護的人一個選項。加密用 `cryptography` 套件：AES-256-GCM，金鑰由密碼經 Argon2id 產生。

### 風險評估

- 設了密碼的匯出檔，忘記密碼就無法還原；匯出時要清楚提醒。

---

## D-08 父母驗證碼：存在資料庫，Argon2id 加鹽雜湊

**狀態**：已定案（2026-10-05，TQ-07）

### 方案比較

| | 存在資料庫（加鹽雜湊） | 存在系統安全儲存區 |
| --- | --- | --- |
| 換手機還原後 | 跟著備份還原，沿用 | 不會跟著備份走，要重設 |
| 安全性 | Argon2id，抗暴力破解 | 更高 |

### 最終選擇與理由

選 **存在資料庫**。驗證碼的用途是區分父母和孩子，不是保護銀行帳戶；還原後能沿用比較符合使用情境。

### 風險評估

- 「忘記驗證碼怎麼辦」由 `/auth-design` 設計（沒有伺服器可以重設）。

---

## D-09 狀態管理：Riverpod 3

**狀態**：已被 D-21 取代（2026-11-01，D-01 改為原生）；原定案 2026-10-05，TQ-08

### 方案比較

| | Riverpod 3 | Bloc 9 |
| --- | --- | --- |
| 樣板程式碼 | 少 | 多 |
| 和 Drift 的搭配 | 監聽查詢可以直接接進來 | 要多包一層 |
| 適合 | 小團隊、一人開發 | 大團隊、需要嚴格追蹤狀態 |

### 最終選擇與理由

選 **Riverpod 3**。一人開發、畫面多半是「查詢結果直接顯示」，搭配 Drift 的監聽查詢最省力。

---

## D-10 主題實作：ThemeData＋ThemeExtension

**狀態**：已被 D-19 取代（2026-11-01，D-01 改為原生）；原定案 2026-10-05，TQ-09

### 最終選擇與理由

- 4 個主題各寫成一個 `ThemeData`，再加自訂 `ThemeExtension` 放內建沒有的 token（鎖錢色、硬陰影、粗框等）。
- 不用第三方主題套件：4 個主題差異很大，套件幫得上的地方有限。
- 主題設定存在手機本機的偏好設定，不放在家庭資料庫，不進備份。

---

## D-11 字型：英文數字字型打包，中文用系統字型

**狀態**：已定案（2026-10-05，TQ-10）

### 最終選擇與理由

- 英文數字字型（Fraunces、Rubik、Nunito、Sora）打包進 App，**關掉 `google_fonts` 的網路下載**（它預設執行時從網路下載，違反離線優先）。
- 中文一律用系統字型（iOS 蘋方、Android Noto Sans CJK）。
- App 只多數百 KB 到 1 MB 左右；打包中文字型一個字重就好幾 MB。

### 風險評估

- 極粗的中文標題在 Android 上可能比設計稿細一點。

---

## D-12 慶祝動畫與音效：Lottie＋內建動畫，audioplayers

**狀態**：已被 D-22 取代（2026-11-01，D-01 改為原生）；原定案 2026-10-05，TQ-11、TQ-12

### 方案比較

| | Flutter 內建動畫 | Lottie | Rive |
| --- | --- | --- | --- |
| App 大小 | 幾乎不增加 | 增加很少，動畫檔約幾十 KB | 原生函式庫，Android 合計約 12 MB |
| 製作 | 寫程式 | 可用 LottieFiles 現成素材 | 要用 Rive 編輯器 |

### 最終選擇與理由

- 動畫：**Lottie＋Flutter 內建動畫**。慶祝效果是播一段動畫，不需要 Rive 的複雜互動，而 Rive 對「App 要小」的代價太大。
- 音效：**`audioplayers` 播短音檔**，設定裡提供開關。

---

## D-13 多語系：Flutter 官方 gen-l10n

**狀態**：已被 D-20 取代（2026-11-01，D-01 改為原生）；原定案 2026-10-05，TQ-13

### 決策背景

第一版支援繁體中文與英文；預設跟著手機系統語言，設定裡可以切換，是手機層級的設定，不進備份。

### 方案比較

| | Flutter 官方 `gen-l10n` | `slang` | `easy_localization` |
| --- | --- | --- | --- |
| 型別安全 | 編譯期檢查 | 編譯期檢查 | 字串 key，執行時才發現打錯 |
| 額外依賴 | 無 | 第三方 | 第三方 |

### 最終選擇與理由

選 **Flutter 官方 `gen-l10n`**（`flutter_localizations`＋`intl`，ARB 翻譯檔）。只有兩種語言，官方方案就夠用，型別安全、不多依賴，App 內切換語言也做得到。

### 風險評估

- 英文字串通常比中文長，按鈕與卡片的版面要預留空間。

---

## D-14 英文介面的內文字型：系統字型

**狀態**：已定案（2026-10-05，TQ-15）

### 最終選擇與理由

- 英文介面的內文用**系統字型**（iOS SF Pro、Android Roboto），不增加 App 大小。
- 標題與數字仍用各主題打包的英文字型（D-11），主題感由它們呈現。

---

## D-15 App 自動產生的紀錄名稱：存類型代碼

**狀態**：已定案（2026-10-05，TQ-14）

### 決策背景

第一版支援繁體中文與英文，可以隨時切換。App 會自動產生一些紀錄名稱（例如「未記錄支出」「常態零用錢」「任務獎勵」）；如果直接存文字，切換語言後舊紀錄不會跟著變。

### 最終選擇與理由

- **App 自動產生的文字存類型代碼**（例如 `wallet_adjustment`），畫面顯示時再依目前語言翻譯。切到英文顯示 Unrecorded spending，切回中文顯示「未記錄支出」。
- **使用者自己輸入的內容**（任務名稱、目標名稱、長輩的一句話）照原文存、照原文顯示，不翻譯。

### 影響

- `/data-model` 的設計原則要納入：系統產生的名稱、類型、狀態一律存代碼，不存顯示文字。

---

## D-16 資料庫：共用 SQL schema，Android SQLDelight、iOS GRDB

**狀態**：已定案（2026-11-01，TQ-16～TQ-18；使用者授權依建議定案）

### 決策背景

- D-06 備份檔是整個 SQLite 檔；家長常在 Android 與 iPhone 之間換手機，備份必須能跨平台還原。
- 兩個平台各自用 ORM 時，資料表結構由框架決定，無法保證一致。

### 方案比較

| | SQLDelight＋GRDB（SQL 優先） | Room＋SwiftData | Room＋GRDB |
|---|---|---|---|
| 兩邊結構一致 | 共用同一份 `.sql` | 不可能 | Room 會加自己的管理表 |
| 監聽查詢 | 兩邊都有 | 有 | 有 |
| 跨平台還原 | 直接可用 | 不行 | 要另外處理 |

### 最終選擇與理由

- `shared/db/` 放唯一一份 schema 與 migration SQL；Android 用 **SQLDelight** 產生 Kotlin，iOS 用 **GRDB** 執行同一批 SQL。
- schema 版本用 `PRAGMA user_version`，兩邊一致；D-06 不用改。

### 風險評估

- 兩邊的查詢程式碼仍是各寫一份，靠 D-23 的共用測試案例保證結果一致。

---

## D-17 介面：Jetpack Compose、SwiftUI；最低 Android 8／iOS 17

**狀態**：已定案（2026-11-01，TQ-19～TQ-21）

- Android **Jetpack Compose**、iOS **SwiftUI**：都是官方主推的宣告式寫法，兩邊對照好維護，寫自訂主題最方便。
- 最低版本 **Android 8（API 26）／iOS 17**：iOS 17 起有 `@Observable`，資料監聽寫法簡單；Android 8 涵蓋幾乎所有還在用的手機。

---

## D-18 Repo 結構：單一 repo

**狀態**：已定案（2026-11-01，TQ-22）

```text
android/            Kotlin＋Compose（第一版）
ios/                Swift＋SwiftUI（Phase 2；第一版只放 README 佔位，目錄先建好）
shared/             兩平台共用的「資料」
  db/               schema 與 migration SQL
  tokens/           4 個主題的 design token JSON
  i18n/             翻譯來源檔
  test-cases/       計算的共用測試案例 JSON
  animations/       Lottie 動畫檔
tools/              兩平台共用的「工具」
  codegen/          tokens、i18n → Kotlin／Swift 的產生腳本
  e2e/              Maestro 端對端流程（同一份兩平台跑）
  ci/               CI 共用腳本（.github/workflows 呼叫這裡）
  scripts/          開發用指令（初始化、跑全部測試等）
.github/workflows/  GitHub Actions 規定的位置，只放觸發設定，邏輯在 tools/ci
docs/
```

- `shared/` 放資料、`tools/` 放工具；兩平台只讀不改，改一次兩邊都生效。
- 產生出來的 Kotlin／Swift 檔放在各平台目錄內（`android/…/generated`、`ios/…/Generated`），不進 `shared/`。
- `ios/` 第一版就建好目錄，`tools/` 的腳本與 CI 從一開始就以「有兩個平台」來寫，Phase 2 加 iOS 時只要補 `ios/` 內容並打開 CI 的 iOS 工作。

---

## D-19 主題：共用 design token JSON，自動產生 Kotlin／Swift

**狀態**：已定案（2026-11-01，TQ-23；取代 D-10）

- `shared/tokens/` 用一份 JSON 定義 4 個主題的顏色、字型、圓角、陰影等，用產生腳本輸出 Kotlin（Compose）與 Swift（SwiftUI）程式碼。
- 字型照 D-11：英文數字字型打包成 App 資源，中文用系統字型；每個主題的字型寫在 token 裡，換字型只改 JSON。
- 產生工具：`tools/codegen/tokens.py`（Python）。產生的 Kotlin 檔會 commit，CI 用 `--check` 確認它跟 JSON 一致；這樣 Android 建置不需要 Python（2026-11-02，AI 決定）。
- 字型檔：Fraunces、Rubik、Nunito、Sora 的可變字型（Google Fonts，OFL 授權）放在 `android/app/src/main/res/font/`，共約 1.1 MB。

---

## D-20 多語系：共用翻譯來源，產生 strings.xml 與 String Catalog

**狀態**：已定案（2026-11-01，TQ-24；取代 D-13）

- `shared/i18n/` 一份來源檔（繁中、英文），產生 Android `strings.xml` 與 iOS String Catalog（`.xcstrings`）。
- D-14（英文內文用系統字型）、D-15（系統產生的紀錄存類型代碼）照舊。

---

## D-21 狀態管理：各平台官方做法

**狀態**：已定案（2026-11-01，TQ-25；取代 D-09）

- Android：ViewModel＋StateFlow，接 SQLDelight 的 Flow 監聽查詢。
- iOS：`@Observable`，接 GRDB 的 ValueObservation。
- 一人開發，官方做法資料最多、依賴最少；不用第三方架構（MVI 套件、TCA）。

---

## D-22 慶祝動畫與音效：Lottie＋系統音效

**狀態**：已定案（2026-11-01，TQ-26；取代 D-12）

- 動畫：兩邊都用官方 Lottie 套件，動畫檔共用一份（`shared/animations/`）。
- 音效：Android SoundPool、iOS AVAudioPlayer 播短音檔；設定裡提供開關。

---

## D-23 自動化測試

**狀態**：已定案（2026-11-01，TQ-27～TQ-31）

### 測試層級

| 層級 | 範圍 | Android | iOS |
|---|---|---|---|
| 單元測試 | 帳本、瀑布填充、餘額、零用錢補記等計算 | JUnit | XCTest／Swift Testing |
| 資料庫測試 | migration、備份還原（含跨平台還原） | SQLDelight 測試驅動 | GRDB 記憶體資料庫 |
| 截圖測試 | 每個畫面 × 4 個主題 | Roborazzi（Robolectric） | swift-snapshot-testing |
| 端對端測試 | 首次設定、記收入、任務驗收、目標達成出金 | Maestro | Maestro |

### 截圖測試改用 Roborazzi（2026-11-02，AI 決定）

- 原定 Paparazzi，但 1.3.5 與 2.0.0-alpha02 都無法在 compileSdk 36 執行（讀取平台 build.prop 失敗）。
- 改用 **Roborazzi 1.77＋Robolectric 4.17**：同樣在 JVM 上算繪、不需要模擬器；Robolectric 可固定算繪用的 SDK（35），不受 compileSdk 影響。
- 基準圖放 `android/app/src/test/snapshots/`（要 commit）；`gradlew recordRoborazziDebug` 更新基準、`gradlew verifyRoborazziDebug` 比對。基準以繁中介面記錄。
- 不同作業系統的字型算繪可能有像素差，CI 只跑測試不做比對；比對在本機做。

### 兩個平台計算一致

- `shared/test-cases/` 用 JSON 寫「輸入的流水帳 → 預期的存款、錢包、已鎖金額、可出金額、各目標進度」，兩邊的單元測試都跑同一批案例；算錯的那邊直接失敗。
- 跨平台還原：用 Android 產生的備份檔在 iOS 測試中還原，反之亦然。

### 端對端測試

- **Maestro**：一份 YAML 流程兩邊都能跑；畫面元件加上相同的測試 ID（Android `testTag`、iOS `accessibilityIdentifier`）。

### CI

- **GitHub Actions**：每次 push 跑 Android 全部測試與 iOS 單元測試；iOS 截圖與端對端測試只在合併前跑，控制 macOS 機器用量。

### 風險評估

- 截圖測試在不同機器上可能有像素差異，需要固定字型與渲染環境。
- macOS CI 免費額度有限，超過要付費或改在本機跑。

---

## D-24 App 名稱與套件 ID

**狀態**：已定案（2026-11-01，使用者決定）

- App 名稱：**Cacao**
- 套件 ID：**`com.ruru.cacao`**（Android `applicationId`；Phase 2 的 iOS Bundle ID 用同一個）
- 上架 Google Play 後不能更改。
