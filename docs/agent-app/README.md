# Cacao App 開發指南

## 專案結構

Cacao 使用 **pnpm workspace** 管理 monorepo 結構：

```
Cacao/ (root)
├── pnpm-workspace.yaml  # Workspace 配置
├── pnpm-lock.yaml       # 統一 lock file
├── node_modules/        # 共享依賴（.pnpm virtual store）
├── package.json         # Root workspace scripts
├── apps/
│   ├── mobile/          # Expo React Native App
│   └── web-admin/       # React Web 管理後台
├── shared/
│   ├── api-schema/      # API 型別定義
│   ├── client-sdk/      # 共用 API 客戶端
│   └── ui-kit/          # 共用 UI 組件
└── server/              # Go backend API
```

## 套件管理規範

### 為什麼使用 pnpm workspace？

1. **節省磁碟空間**：使用 content-addressable store，相同版本的套件只儲存一次
2. **嚴格依賴管理**：避免 phantom dependencies（未宣告的依賴）
3. **快速安裝**：parallel + 硬連結機制，比 npm/yarn 快 2-3 倍
4. **CI 友善**：lock file 穩定，減少 diff 衝突

### 常用指令

#### 根目錄操作（推薦）

```bash
# 安裝所有 workspace 依賴
pnpm install

# 在特定 workspace 執行指令
pnpm --filter mobile start           # 啟動 Expo
pnpm --filter mobile add expo-camera # 在 mobile 安裝套件

# 在所有 workspace 執行指令（recursive）
pnpm -r run typecheck  # 全部 TypeScript 型別檢查
pnpm -r run lint       # 全部 ESLint 檢查
pnpm -r run test       # 執行所有測試

# Root scripts（已配置在 package.json）
pnpm dev:mobile   # = pnpm --filter mobile start
pnpm typecheck    # = pnpm -r run typecheck
pnpm lint         # = pnpm -r run lint

# Adnriod Debug
cd D:\Cacao\apps\mobile\android                                                        
.\gradlew.bat assembleDebug     
```

#### Workspace 內操作（不推薦）

```bash
# ❌ 避免在子目錄直接 pnpm install
cd apps/mobile
pnpm install  # 會破壞 workspace 結構!

# ✅ 應該在根目錄使用 --filter
cd ../..  # 回到根目錄
pnpm --filter mobile install
```

### 安裝新套件規則

1. **在根目錄操作**（強制）:
   ```bash
   # ✅ 正確
   pnpm --filter mobile add @nozbe/watermelondb
   
   # ❌ 錯誤
   cd apps/mobile
   pnpm add @nozbe/watermelondb  # 會產生獨立 lock file!
   ```

2. **devDependencies 安裝在哪？**
   - 專案專屬的放子 workspace（如 `eslint-config-expo` → mobile）
   - 通用工具放 root（如 `prettier`, `husky`）

3. **Peer dependencies 處理**：
   ```bash
   # pnpm 會自動提示 peer deps，照指示安裝即可
   pnpm --filter mobile add react-native-svg
   # 若提示缺 react，執行：
   pnpm --filter mobile add -D react@^19.1.0
   ```

### Workspace 依賴引用

在 `apps/mobile/package.json` 引用 shared workspace：

```json
{
  "dependencies": {
    "@cacao/client-sdk": "workspace:*",  // 引用 shared/client-sdk
    "@cacao/ui-kit": "workspace:^"       // 引用 shared/ui-kit
  }
}
```

- `workspace:*`：使用 workspace 內當前版本
- `workspace:^`：使用 workspace 內兼容版本（發布時轉換為 ^x.y.z）

## Expo 開發流程

### 啟動開發伺服器

```bash
# 根目錄執行（推薦）
pnpm dev:mobile

# 或使用 --filter
pnpm --filter mobile start

# 清除快取啟動
pnpm --filter mobile start -- -c
```

### 在實體裝置測試

1. 安裝 Expo Go app（iOS/Android）
2. 確保手機與電腦在同一 Wi-Fi
3. 掃描終端機顯示的 QR code

### 在模擬器測試

```bash
# iOS 模擬器（需要 Xcode）
pnpm --filter mobile run ios

# Android 模擬器（需要 Android Studio）
pnpm --filter mobile run android
```

### TypeScript 型別檢查

```bash
# 檢查 mobile app
pnpm --filter mobile typecheck

# 檢查所有 workspace
pnpm typecheck
```

### ESLint 檢查

```bash
# Lint mobile app
pnpm --filter mobile lint

# Lint 所有 workspace
pnpm lint
```

## 常見問題

### Q1: node_modules 要加入 .gitignore 嗎？

**A**: 是的，但只需在 `.gitignore` 加 root 的 `node_modules/`：

```gitignore
# Root
node_modules/
pnpm-lock.yaml  # ❌ 錯誤！lock file 要提交
```

正確版本：
```gitignore
node_modules/
# pnpm-lock.yaml 不應該被忽略!
```

### Q2: 為什麼不能在子目錄執行 pnpm install？

**A**: pnpm workspace 使用根目錄的 `pnpm-lock.yaml` 管理所有依賴。在子目錄執行會產生獨立的 lock file，破壞 workspace 結構。

### Q3: package-lock.json 還需要嗎？

**A**: 不需要！已刪除所有 `package-lock.json`，統一使用 `pnpm-lock.yaml`。

### Q4: 如何清理並重新安裝？

```bash
# 清理所有 node_modules 和 lock file
rm -rf node_modules apps/*/node_modules shared/*/node_modules

# 重新安裝（只在根目錄）
pnpm install
```

### Q5: Windows 長路徑問題怎麼辦？

如果遇到 "找不到路徑" 錯誤（通常是 React Native 深層目錄）：

```powershell
# 使用 robocopy 強制清理
$emptyDir = New-Item -Type Directory -Force ".\empty_temp"
robocopy $emptyDir node_modules /MIR /R:0 /W:0 | Out-Null
Remove-Item node_modules -Force
Remove-Item empty_temp -Force
```

### Q6: Expo 警告 peer dependency 不符怎麼辦？

```bash
# 範例：react-dom 要求 react@^19.2.1 但目前是 19.1.0
pnpm --filter mobile add react@^19.2.1

# 若 Expo SDK 54 不支援，可暫時忽略（Expo 會處理）
```

## 相關資源

- [pnpm workspace 文件](https://pnpm.io/workspaces)
- [Expo 官方文件](https://docs.expo.dev/)
- [Expo Router 文件](https://docs.expo.dev/router/introduction/)
- [專案 Commander 規則](../commander.md)
- [F 系列任務日誌](./agent-app-log.md)
