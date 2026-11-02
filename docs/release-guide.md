# Cacao 第一版上架步驟（Android，人工上架）

> Phase 1 用人工上架，讓使用者熟悉流程；CD 自動上架在 Phase 2。
> 標 👤 的步驟要使用者本人操作（帳號、付款、身分驗證）；標 🤖 的由 Claude 處理。

## 1. 帳號

- 👤 申請 Google Play 開發者帳號（一次性 US$25，需要身分驗證，可能要等幾天）。
- 👤 決定帳號類型：個人或機構。**個人帳號**正式上架前要先做封閉測試：至少 12 位測試者、連續 14 天。

## 2. 簽名金鑰

- 🤖 用 `keytool` 產生上傳金鑰（upload key），放在 repo 外（例如 `D:\git\Android\keys\`）。
- 👤 設定並保管金鑰密碼；金鑰與密碼**不能進 git**，弄丟就無法更新 App。
- 🤖 release 建置讀取本機的 `keystore.properties`（不 commit），並開啟 R8 縮小 App；跑完整測試確認 release 版正常。
- Google Play 會再用自己的「App 簽署金鑰」簽一次（Play App Signing），這是預設做法。

## 3. Google 雲端備份（OAuth）

- 👤 在 Google Cloud Console 建立專案。
- 👤 設定 OAuth 同意畫面：外部使用者、App 名稱 Cacao、支援信箱、範圍只加 `drive.appdata`（非敏感範圍，不需要 Google 審核）。
- 🤖👤 建立 Android OAuth 用戶端：套件名稱 `com.ruru.cacao`，SHA-1 要加兩組：上傳金鑰的、以及 Play Console 裡「App 簽署金鑰」的。
- 🤖 在有 Google Play 的模擬器或手機上驗證：開啟雲端備份、立即備份、從 Google 還原、30 天自動備份。

## 4. 上架資料

- 🤖 隱私權政策：說明資料只存在手機、Google 雲端備份由使用者開啟且只存在使用者自己的雲端硬碟、不收集任何個人資料。需要一個公開網址（例如 GitHub Pages）。
- 🤖 商店文案：名稱、短說明（80 字）、完整說明（繁中＋英文）。
- 🤖 圖像：App 圖示 512×512、主題圖 1024×500、手機截圖至少 2 張（用模擬器截圖）。
- 👤 Play Console 表單：資料安全性（不收集資料；雲端備份由使用者控制）、內容分級問卷、廣告（無）。
- 👤 ⚠ **目標對象**：App 給父母與孩子一起用。若勾選 13 歲以下，要遵守「Designed for Families」政策；需要使用者決定。

## 5. 發布

- 🤖 產生 release AAB（`gradlew bundleRelease`）。
- 👤 第一版在 Play Console 手動上傳到「內部測試」，自己先裝來試。
- 👤 個人帳號：建立「封閉測試」，邀請至少 12 位測試者，連續 14 天。
- 👤 申請正式版、送審、發布。
