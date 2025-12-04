# Notes
## Go Api

## Expo App

### Q0001
為什麼 package-lock.json 在子目錄 即便我所有的前端城市 都寫在apps apps\mobile apps\web-admin 我直覺應該要在 apps\ 項下

原因：npm 不是 monorepo-aware。哪個資料夾有 package.json，你在那裡跑 npm install 就會產生自己的 package-lock.json。

想集中在 apps/
→ 必須用 workspace（npm workspaces / pnpm / yarn）。
→ 不然永遠會在子目錄產生。

如果你願意，我可以幫你：
提供 完整範例：workspace、scripts、Turbo、EAS（若是 Expo）、Nuxt 3 架構

### Q0002
解析我的專案細節， package-lock.json 在 apps 好還是不好

**當前狀態分析**：

你的專案根目錄 `package.json` 已經定義了 workspace：
```json
{
  "workspaces": ["apps/*", "shared/*"],
  "packageManager": "pnpm@10.5.0+..."
}
```

**目前的問題**：
1. ❌ 根目錄有 `package-lock.json`（npm 的 lock file）
2. ❌ 根目錄有 `node_modules/`
3. ✅ 已宣告使用 `pnpm@10.5.0` 作為 packageManager
4. ✅ `apps/mobile/` 使用 `pnpm-lock.yaml`（正確）
5. ⚠️ 混用 npm 和 pnpm，狀態不一致

**建議方案：統一使用 pnpm workspace**

### 為什麼應該在根目錄統一管理？

#### ✅ 優點（推薦）：
1. **單一 lock file**：只有一個 `pnpm-lock.yaml` 在根目錄
2. **共享依賴**：重複的套件（如 TypeScript、React）只安裝一次
3. **統一版本**：避免 mobile/web-admin 使用不同版本的相同套件
4. **更快安裝**：pnpm 的 symlink 機制，節省磁碟空間和時間
5. **統一指令**：在根目錄一次執行所有子專案的指令

#### ❌ 子目錄各自管理的缺點：
1. 每個子專案都有自己的 `node_modules/`（重複浪費）
2. 多個 lock file 難以同步
3. 版本衝突風險高
4. CI/CD 需要分別安裝多次

### 建議執行步驟：

**1. 清理根目錄的 npm 殘留**
```bash
# 刪除根目錄的 npm 產物
rm -rf node_modules package-lock.json
```

**2. 創建 `pnpm-workspace.yaml`**
```yaml
packages:
  - 'apps/*'
  - 'shared/*'
```

**3. 清理所有子專案的 node_modules**
```bash
rm -rf apps/*/node_modules apps/*/package-lock.json
```

**4. 在根目錄執行 pnpm install**
```bash
pnpm install
```

**5. 更新根目錄的 package.json scripts**
```json
{
  "scripts": {
    "dev:mobile": "pnpm --filter mobile start",
    "dev:web": "pnpm --filter web-admin dev",
    "lint": "pnpm -r run lint",
    "typecheck": "pnpm -r run typecheck",
    "build:mobile": "pnpm --filter mobile run build"
  }
}
```

### 最終結構：

```
Cacao/
├── pnpm-workspace.yaml        # pnpm workspace 配置
├── package.json               # 根目錄統一管理
├── pnpm-lock.yaml            # ✅ 唯一的 lock file
├── node_modules/             # ✅ 統一的依賴目錄
│   └── .pnpm/               # pnpm 的 virtual store
├── apps/
│   ├── mobile/
│   │   ├── package.json     # 宣告依賴
│   │   └── node_modules/    # ❌ 刪除（使用根目錄的）
│   └── web-admin/
│       ├── package.json
│       └── node_modules/    # ❌ 刪除（使用根目錄的）
└── shared/
    └── ...
```

### 結論：

**強烈建議在根目錄統一管理**，因為：
1. 你已經宣告了 workspace 和 pnpm
2. 可以享受 pnpm workspace 的所有優勢
3. 符合現代 monorepo 最佳實踐
4. 避免目前 npm/pnpm 混用的混亂狀態

### Q0003 Metro 是什麼？

Metro = React Native / Expo 專用的 JavaScript Bundler。

作用類似：

Web：Vite / Webpack

iOS/Android 原生：無類似系統，Metro 就是唯一 bundler

Expo App：100% 依賴 Metro

Metro 的職責：

把所有 JS/TS 檔案打包成一個 bundle（給手機執行）

處理 React Native 的特殊轉換（JS → 原生 Bridge）

watch 檔案變化

處理路徑解析（resolve）

特點：
Metro 不像 Vite 或 Webpack 那麼聰明。對 monorepo、symlink 敏感。
如果解析不到某個檔案，就直接報 500 bundle error。

你的 500 錯誤就是 Metro 找不到 expo-router/entry。

### Q0004 symlink 是什麼？

symlink = 符號連結（Symbolic Link），類似捷徑。

作用：
讓某個路徑 指向 另一個真實路徑。

例如：

node_modules/react  →  /pnpm/.pnpm/react@18.3.0/node_modules/react


在檔案系統看起來像：

node_modules/react/


但實際上只是指到

pnpm/.pnpm/react@18.3.0/...


pnpm 的核心機制就是：

只下載套件一次

建 symlink 給各專案重用

比起 npm / yarn：

npm = 每個專案一份完整 node_modules
pnpm = 共享唯一套件，其他都用 symlink 指過去


優點：超快速、省硬碟、省時間
缺點：某些 bundler（如 Metro）不會順著 symlink 找到真正的檔案。

為什麼 Metro + pnpm symlink 容易出狀況？

Metro 路徑解析模式較原始，不會自動：

apps/mobile/node_modules/react → symlink → /pnpm/.pnpm/react@x/node_modules/react


它只看 node_modules，如果裡面是 "指向別處"，它就會解析失敗。

所以才需要加：

config.watchFolders = [workspaceRoot];
config.resolver.nodeModulesPaths = [
  path.resolve(workspaceRoot, 'node_modules'),
];


來告訴 Metro：

「嘿，真實的套件都在 workspace 根目錄，不是在 apps/mobile 下面。」

### Q0005 Gradle 是什麼？

cd apps/mobile/android; .\gradlew.bat assembleDebug --stacktrace

### Q0006 Gradle 版本依賴重新安裝

看到問題了！Gradle 建置需要 Java 11+，但系統使用的是 Java 8。讓我幫您設定正確的 Java 版本：



java -version



$env:JAVA_HOME

確認是 Java 8 且 JAVA_HOME 未設定。Android Studio 通常包含 Java 17。讓我找到並設定它：



Get-ChildItem "C:\Program Files\Android\Android Studio" -Recurse -Directory -Filter "jbr" -ErrorAction SilentlyContinue | Select-Object -First 1 -ExpandProperty FullName

完美！找到了 JBR（JetBrains Runtime）。讓我設定 JAVA_HOME：



[System.Environment]::SetEnvironmentVariable('JAVA_HOME', 'C:\Program Files\Android\Android Studio\jbr', [System.EnvironmentVariableTarget]::User); $env:JAVA_HOME = 'C:\Program Files\Android\Android Studio\jbr'; & "$env:JAVA_HOME\bin\java.exe" -version

**安裝狀態確認：**

- ✅ JAVA_HOME 已設定為 Android Studio 的 JDK
- ✅ Java 版本：OpenJDK **21.0.8**（符合 Gradle 8.14.3 需求）

**接下來需要重新啟動 VS Code 讓環境變數生效：**

1. 請**完全關閉 VS Code**
2. 重新開啟 VS Code
3. 再次執行 "Run Android" debug 配置

重啟後 Gradle 應該會使用 Java 21，可以通過 JVM 版本檢查。如果還有其他錯誤（例如找不到 Android 裝置），我們再繼續處理。

### Q0007 路徑太長

你的真正錯誤現在非常明確：

**Windows 路徑太長 → CMake + Ninja 無法編譯 Expo Native 模組**

錯誤核心：

```
full path 有 227 characters  
CMAKE_OBJECT_PATH_MAX = 250  
The build may not work correctly.

ninja: error: manifest 'build.ninja' still dirty after 100 tries
```

這是 Windows 的經典問題（尤其是 pnpm + monorepo）：

**pnpm node_modules 路徑非常深 → Windows 原生 CMake/Ninja 超過路徑上限 → 編譯失敗**

Expo SDK 50 / RN 0.75 原生模組（expo-modules-core）有 C++ 檔案，必須使用 CMake/Ninja 編譯。
 但 Windows 預設最大完整路徑長度仍然偏低（即使你啟用了長路徑支持，CMake/Ninja 自己也有限制）。

```
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

### Q0006
你遇到的是「CMake + pnpm + Windows MAX_PATH 限制」的典型組合爆炸。

最有效順序是：

移到更短的專案路徑（立刻有效）

改用 npm/yarn，不使用 pnpm

啟用 Windows LongPath

必要時關掉 Fabric

# 以管理員身份執行 PowerShell,然後執行:
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force

# 或透過 regedit:
# 1. Win+R → regedit
# 2. 前往: HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\FileSystem
# 3. 新增 DWORD: LongPathsEnabled = 1
# 4. 重啟電腦