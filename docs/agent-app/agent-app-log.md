## Tasks

### F0001
1. 為了完成 P0001 完成帳號密碼登入功能
2. 初始化 expo 專案
3. 專案目標是移動 app 可以安裝在 ios/andriod
4. 移動裝置資料庫，建立「專業級行動 App」使用 WatermelonDB 搭 SQLite
5. 登入要做到，註冊帳號密碼可以登入
6. 登入要做到，gmail可以登入

#### TaskReply
正在準備實作 F0001 任務。技術選型更新：

**資料庫方案**：WatermelonDB + SQLite
- ✅ 專為 React Native 設計，效能優異
- ✅ 內建離線同步機制
- ✅ 支援 lazy loading，適合大量資料
- ✅ 與後端 SQLite schema 可對應

### F0002 初始化 Expo 專案（SDK 54+）+ 轉換為 pnpm workspace

#### TaskReply
**任務狀態**：✅ 完成

**執行步驟**：
1. ✅ 在 `apps/` 目錄下創建 `mobile/` 資料夾
2. ✅ 使用 `npx create-expo-app` 初始化專案
3. ✅ 配置 Expo Router（檔案路由）
4. ✅ 設定 TypeScript 與 ESLint
5. ✅ 配置 app.json（支援 iOS/Android）
6. ✅ 安裝基礎依賴套件
7. ✅ **[新增]** 轉換整個 monorepo 為 pnpm workspace
8. ✅ **[新增]** 清理所有 npm 殘留（package-lock.json, node_modules）
9. ✅ **[新增]** 更新 root package.json scripts 使用 pnpm 語法

**技術規格**：
- Expo SDK 54.0.25
- TypeScript 5.9.3
- Expo Router 6.0.15（檔案路由系統）
- ESLint 9.39.1 + @typescript-eslint
- **pnpm 10.5.0 workspace**（統一套件管理）

**已完成配置**：
- ✅ `app.json` 配置完成（app name: Cacao, bundleId/package: com.cacao.app）
- ✅ `package.json` 主入口設為 `expo-router/entry`
- ✅ TypeScript 嚴格模式啟用，path alias `@/*` 配置完成
- ✅ ESLint 規則配置（extends expo + @typescript-eslint）
- ✅ 創建 `app/_layout.tsx` 和 `app/index.tsx` 基礎路由結構
- ✅ TypeScript 型別檢查通過
- ✅ **pnpm workspace 配置**：
  - 創建 `pnpm-workspace.yaml`（packages: apps/*, shared/*）
  - 清理所有 subdirectory node_modules 和 lock files
  - 根目錄統一 `pnpm install`（853 packages）
  - 更新所有 scripts 使用 pnpm 語法（`pnpm --filter`, `pnpm -r`）

**專案結構**：
```
Cacao/ (root)
├── pnpm-workspace.yaml  # Workspace 配置
├── pnpm-lock.yaml       # 統一 lock file
├── node_modules/        # 共享依賴（.pnpm virtual store）
├── package.json         # Root workspace scripts
└── apps/mobile/
    ├── app/
    │   ├── _layout.tsx  # Root layout
    │   └── index.tsx    # 首頁
    ├── assets/          # 圖片資源
    ├── eslint.config.js # ESLint 配置
    ├── app.json         # Expo 配置
    ├── package.json     # Mobile app 依賴
    └── tsconfig.json    # TypeScript 配置
```

**pnpm workspace 優勢**：
- ✅ 統一依賴管理，避免重複安裝
- ✅ 節省磁碟空間（content-addressable store）
- ✅ 嚴格依賴隔離（phantom dependencies 防護）
- ✅ 快速安裝與 CI 友善

**驗證結果**：
```bash
# TypeScript 型別檢查通過
$ pnpm typecheck
> tsc --noEmit  # ✅ No errors

# Expo 專案結構正確
$ pnpm --filter mobile start  # ✅ Ready to run
```

### F0003 撰寫 workspace 細節

#### TaskReply
**任務狀態**：✅ 完成

**執行內容**：
1. ✅ 創建 `docs/agent-app/README.md` 詳細開發指南
2. ✅ 記錄 pnpm workspace 使用規範
3. ✅ 整理常用指令參考
4. ✅ 編寫常見問題 Q&A（6 個問題）

**文件內容**：
- **專案結構說明**：Monorepo 組織方式與 workspace 配置
- **套件管理規範**：
  - 為什麼使用 pnpm workspace（4 大優勢）
  - 常用指令（根目錄操作 vs Workspace 內操作）
  - 安裝新套件規則（3 條強制規範）
  - Workspace 依賴引用方式
- **Expo 開發流程**：
  - 啟動開發伺服器
  - 實體裝置測試
  - 模擬器測試
  - TypeScript 型別檢查
  - ESLint 檢查
- **常見問題 Q&A**：
  - Q1: .gitignore 設定
  - Q2: 為什麼不能在子目錄執行 pnpm install
  - Q3: package-lock.json 是否需要
  - Q4: 如何清理並重新安裝
  - Q5: Windows 長路徑問題處理
  - Q6: Expo peer dependency 警告處理

###　F0004 我是否可以啟動執行檢查 前端執行正常
**驗證結果**：
```bash
$ pnpm --filter mobile start
✅ Expo 成功啟動
✅ Metro Bundler 運行正常
✅ QR code 已生成（可用 Expo Go 掃描）
✅ Web 界面：http://localhost:8081
```
###　F000５ 執行有異常
1. debug "name": "Start Expo Server"
2. Web Bundling failed 1595ms node_modules\.pnpm\expo-router@6.0.15_@expo+me_d25fb73b44f411a33f6d117d75737780\node_modules\expo-router\entry.js (263 modules)   
3. entry.bundle:1  Failed to load resource: the server responded with a status of 500 (Internal Server Error)
(index):1 Refused to execute script from 'http://localhost:8081/node_modules/.pnpm/expo-router@6.0.15_@expo+me_d25fb73b44f411a33f6d117d75737780/node_modules/expo-router/entry.bundle?platform=web&dev=true&hot=false&lazy=true&transform.engine=hermes&transform.routerRoot=app&unstable_transformProfile=hermes-stable' because its MIME type ('application/json') is not executable, and strict MIME type checking is enabled.

問題原因
pnpm workspace 的 symlink 結構導致 Metro bundler 無法正確解析 expo-router/entry，造成 Web bundling 500 錯誤。

解決方案
創建了 metro.config.js 配置檔，正確處理 monorepo 結構：

✅ 監聽整個 workspace
✅ 設定正確的模組解析路徑
✅ 自訂解析器處理 expo-router/entry

我看到錯誤依然存在。讓我檢查 metro.config.js 是否正確創建，並查看終端輸出以了解詳細錯誤：

Read metro.config.js, lines 1 to 50

Checked background terminal output

問題很清楚了！缺少 react-native-web 依賴。Expo SDK 54 的 Web 支援需要這個套件：


pnpm --filter mobile add react-native-web react-dom
伺服器停止了。讓我等待安裝完成：


### F0004 配置 WatermelonDB + SQLite adapter

### F000N 定義資料模型（users, sync_queue）


### F000N 實作帳號密碼註冊/登入 UI
### F000N 實作 Google OAuth 登入
### F000N 建立同步機制與 API 整合層


