# Cacao 第一版上架步驟（Android，人工上架）

> Phase 1 用人工上架，讓使用者熟悉流程；CD 自動上架在 Phase 2。
> 👤＝使用者本人（帳號、付款、身分驗證、登入、密碼、要拍板的事）；🤖＝Claude。
> 原則：能用指令做的 Claude 用指令；只能在網頁後台設的，Claude 用 Chrome 擴充在使用者**已登入**的瀏覽器代點，使用者在旁邊看。使用者不需要給 Claude 任何 API key 或密碼。

## 總覽：使用者要做的事

1. 申請 Google Play 開發者帳號並付款、驗證身分。
2. 設定並保管簽名金鑰密碼。
3. 登入：Play Console、Google Cloud（`! gcloud auth login`）、Chrome。
4. 拍板：帳號類型、目標對象年齡；提供對外聯絡信箱（商店頁與隱私權政策要用，會公開）。
5. 封閉測試：找 12 位測試者。
6. 按下「送審／發布」。

其他都由 Claude 做。

## 1. 帳號

- 👤 申請 Google Play 開發者帳號（一次性 US$25，需要身分驗證，可能要等幾天）。
- 👤 拍板帳號類型：個人或機構。**個人帳號**正式上架前要先做封閉測試：至少 12 位測試者、連續 14 天。

## 2. 簽名金鑰

- 👤 想一組金鑰密碼，自己保管（建議存密碼管理器）。弄丟就無法更新 App。
- 🤖 用 `keytool` 產生上傳金鑰（upload key），放在 repo 外 `D:\git\Android\keys\`。
- 🤖 建 `keystore.properties`（只在本機，已 ignore，不 commit）；密碼由使用者自己填進去。
- 🤖 release 建置開啟 R8 縮小 App，跑完整測試確認 release 版正常。

## 3. 在 Play Console 建立 App、上傳內部測試

- 👤 在 Chrome 登入 Play Console，允許 Claude 擴充使用該網站。
- 🤖 Chrome 代點建立 App（名稱 Cacao、免費、App 類型）。
- 🤖 產生 release AAB（`gradlew bundleRelease`）。
- 🤖 Chrome 代點上傳到「內部測試」；Play App Signing 用預設（Google 保管 App 簽署金鑰）。
- 👤 從內部測試連結把 App 裝到自己手機試用。

## 4. Google 雲端備份（OAuth）

> 要等第 3 步做完，才拿得到 Play「App 簽署金鑰」的 SHA-1。

- 🤖 安裝 Google Cloud CLI（放 `D:\git\Android\`）。
- 👤 在終端機輸入 `! gcloud auth login`，用瀏覽器登入 Google 一次。
- 🤖 用 `gcloud` 建立專案、啟用 Drive API（`gcloud projects create`、`gcloud services enable drive.googleapis.com`）。
- 🤖 Chrome 代點設定 OAuth 同意畫面（gcloud 沒有對應指令）：外部使用者、App 名稱 Cacao、支援信箱、範圍只加 `drive.appdata`（非敏感範圍，不需要 Google 審核）。
- 🤖 Chrome 代點建立 Android OAuth 用戶端：套件名稱 `com.ruru.cacao`，SHA-1 加兩組：上傳金鑰的、Play Console「App 簽署金鑰」的。
- 🤖 在有 Google Play 的模擬器驗證：開啟雲端備份、立即備份、從 Google 還原、30 天自動備份。
- 👤 用手機上的內部測試版，實際登入自己的 Google 帳號備份一次。

## 5. 上架資料

- 🤖 隱私權政策：資料只存在手機、Google 雲端備份由使用者開啟且只存在使用者自己的雲端硬碟、不收集任何個人資料。放在公開網址（GitHub Pages）。
- 🤖 商店文案：名稱、短說明（80 字）、完整說明（繁中＋英文）。
- 🤖 圖像：App 圖示 512×512、主題圖 1024×500、手機截圖至少 2 張（模擬器截圖）。
- 👤 ⚠ 拍板**目標對象**：App 給父母與孩子一起用。若勾 13 歲以下，要遵守「Designed for Families」政策。
- 🤖 Chrome 代填 Play Console 表單：商店資訊、資料安全性（不收集資料；雲端備份由使用者控制）、內容分級問卷、廣告（無）、目標對象（照使用者決定）。
- 👤 填完逐頁看過，確認無誤。

## 6. 發布

- 🤖 Chrome 代點建立「封閉測試」軌道（個人帳號才需要）。
- 👤 邀請至少 12 位測試者（提供他們的 Google 帳號 email），讓他們連續 14 天保持參與。
- 🤖 期間收到的問題由 Claude 修，重新產生 AAB 上傳。
- 👤 14 天後申請正式版存取權、按下送審與發布。
