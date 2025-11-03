# Cacao

Developing a Family Allowance Management System

打造家庭零用錢管理App — Nuxt 3 + Go 全端實作

## 專案介紹：Nuxt3 + GO API + MSSQL 專案初始化與環境建立

### Git Repo 準備

新增 github Organizations Cacao's 

新增 github Repositories Cacao

### Ide 準備

Vscode 開發 Cacao Nuxt Web 與 Go API

dbever 連線 mysql

docker Desktop 運行 docker container

### 資料庫設計

Cacao\Cacao\db\CacaoInit.sql

### 初始化 cacaoapi GO web

```powershell
# 初始化 module
go mod init cacaoapi
# 安裝 Gin 套件
go get -u github.com/gin-gonic/gin
go get -u gorm.io/gorm
go get -u gorm.io/driver/mysql
```

### 初始化 cacaoweb Nuxt app

```
npx nuxi init cacaoweb

pnpm add pinia @pinia/nuxt
pnpm add pinia-plugin-persistedstate
pnpm add dayjs
pnpm add @ant-design/icons-vue
npx nuxi@latest module add ionic
npx nuxi@latest module add tailwindcss

pnpm add -D eslint @nuxt/eslint-config eslint-plugin-nuxt eslint-plugin-vue eslint-config-prettier
pnpm add -D @ant-design-vue/nuxt
pnpm add -D dotenv
pnpm add -D cross-env
```

update nuxt.config.ts

```typescript
export default defineNuxtConfig({
  devServer: {
    port: 7726,
  },
  runtimeConfig: {
    public: {
      storeApi: 'cacaoapi',
    }
  },
  compatibilityDate: '2025-06-20',
  devtools: { enabled: true },
  nitro: {
    devProxy: {
      '/cacaoapi': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        secure: false,
      },
    },
  },
  modules: [
    '@ant-design-vue/nuxt',
    '@pinia/nuxt',
    'pinia-plugin-persistedstate/nuxt',
    '@nuxt/eslint',
    '@nuxtjs/tailwindcss',
    '@nuxtjs/ionic'
  ],
  antd: {
    // Options
  },
  app: {
    head: {
      title: 'LineCRM.CarCare',
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
      ]
    }
  },
  sourcemap: {
    server: true,
    client: true
  },
  ssr: false,
})
```

2. 安裝 Capacitor

```
pnpm install --save @capacitor/core @capacitor/cli
npx cap init
```

初始化時選擇：

- App name: `YourAppName`
- App id: `com.yourdomain.appname`

3. 設定 `capacitor.config.ts`

設定 build output 為 Nuxt 輸出的目錄：

```
tsCopyEditimport { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.yourdomain.appname',
  appName: 'YourAppName',
  webDir: 'dist',
  bundledWebRuntime: false
};

export default config;
```

4. 加入 Android 平台

```
npx cap add android
```

5. 建立 Nuxt 的 Production Build

```
npm run build
```

確保你在 `nuxt.config.ts` 中使用的是 `ssr: false`，以便生成為 SPA：

```
tsCopyEditexport default defineNuxtConfig({
  ssr: false
})
```

6. 同步到 Android 專案

```
npx cap copy android
```

#### 7. 使用 Android Studio 開啟專案

```
npx cap open android
```

在 Android Studio 中編譯與部署到模擬器或實體裝置。

### 附加：安裝 Ionic UI 元件

你可以使用 Ionic 的核心元件：

```
npm install @ionic/vue @ionic/core
```

並在 `app.vue` 中初始化：

```
<script setup lang="ts">
import { IonApp, IonContent } from '@ionic/vue'
</script>

<template>
  <IonApp>
    <IonContent>
      <NuxtPage />
    </IonContent>
  </IonApp>
</template>
```

### 

### Gmail OAuth2 登入流程（前端登入拿 token，後端驗證）



### 完成 Nuxt3 登入流程，登入後自動建立 User 資料（Giver / Baby）



### Nuxt3 建立 Pinia 狀態管理（userStore, walletStore）



### 建立 GO API：User Profile CRUD

### 前端完成 Profile 頁面（個人資料管理）

### 建立 GO API：查詢餘額、發錢（Giver 發錢功能）

### 前端實作 Giver 發錢頁面（指定 Baby 加錢）

### 建立 GO API：提款申請（Baby 發起提款請求）

### 前端實作 Baby 提款申請頁面

### 建立 GO API：提款審核（Giver同意/拒絕）

### 前端實作 Giver 提款審核頁面

### 建立 GO API：支出紀錄（Baby新增支出）

### 前端實作 Baby 支出登記頁面

### 前端實作 Baby 支出歷史紀錄頁面

### 建立 GO API：Giver查看 Baby 消費紀錄

### 前端實作 Giver 消費紀錄頁面（含搜尋/篩選）

------

### Day 21 加入交易時間軸頁面（Timeline 顯示交易歷程）

### Day 22 建立 GO API：上傳使用者頭像（接收 Multipart / 上傳雲端或本地）

### Day 23 Nuxt3 實作頭像上傳功能

### Day 24 加上前端通知功能（Polling 拿提款申請提醒）

------

### Day 25 完成多 Giver 支援單一 Baby 的資料結構與API調整

### Day 26 建立 GO 排程服務（HostedService）自動定期加零用錢

### Day 27 加入交易分類系統（食物、玩具等），前端後端同步支援分類

### Day 28 前端實作每月支出統計圖表（Pie Chart、Bar Chart）

------

### Day 29 服務容器化（Docker化 Nuxt3 與 GO API，連接 MSSQL）

```
# 基底 Node.js image
FROM node:20-slim

# 建立工作資料夾
WORKDIR /app

# 複製 package.json 和 package-lock.json
COPY package.json ./

# 安裝依賴
RUN npm install

# 複製其他所有檔案
COPY . .

# Build Nuxt (如果是 SPA/SSG模式)
RUN npm run build

# 開放 port
EXPOSE 3000

# 啟動 Nitro server
CMD ["npm", "run", "start"]
```

SQLite 檔案（例如 `data.db`）直接存放在 `/app/server/db/` 裡。

Docker 容器內部會啟一個 Node server (`npm run start`)。

外部只需要連到 3000 port 就能使用你的前端+後端。

| 功能              | 說明                                               |
| :---------------- | :------------------------------------------------- |
| Volume掛載        | 把 SQLite 資料夾掛到宿主機上，避免容器重建資料消失 |
| Multi-stage build | build 階段跟 run 階段拆開，減少 image 大小         |
| Docker Compose    | 之後可以方便多環境（測試、正式）部署切換           |

```
version: '3'
services:
  pocketmoney-app:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./server/db:/app/server/db  # 掛載資料夾，保留SQLite資料
    environment:
      - NODE_ENV=production
```

### 5. 如果想打包成 APK 上架 Google Play（選項）

- 使用 Capacitor 封裝 Nuxt3 SPA 網址
- 指定 `server.url = 'https://your-webapp-domain.com'`
- 用 Android Studio 打包 APK
- 必須申請 Google Play 開發者帳號（$25一次性）
- 上傳、填資料、審核、上架



### 最佳化注意事項



| 項目                                     | 建議                                 |
| ---------------------------------------- | ------------------------------------ |
| icon尺寸要齊全                           | 最好準備 192x192, 512x512, 1024x1024 |
| 要有 start_url、scope                    | 不然 PWA 會認不出首頁                |
| display 設定成 standalone                | 這樣加到桌面時沒瀏覽器邊框           |
| theme_color, background_color 要配色好看 | 不然安裝畫面很醜                     |
| registerType 設 autoUpdate               | 確保有新版自動更新 Service Worker    |

### 完成後效果

- 使用者可以「加到主畫面」
- 打開像原生 App，無瀏覽器 UI
- 支援離線瀏覽（支援緩存頁面）
- 新版自動提示更新
- 可以上 Google Play Store（Web App型）



### 測試本地 App（SQLite資料儲存、API呼叫正常），修正所有 Bugs

### 打包 APK，申請 Google Play 開發者帳號，上傳 Google Play Console



### 其他功能規劃

### 意外之財，我要先存起來



產品定位與市場：屬「親子理財教育」利基市場，中文圈競品少；以家庭互動 + 交易一致性為核心價值，先推 Android MVP 驗證再擴 iOS。

資料層原則：MSSQL/MySQL 為唯一真實帳本；SQLite 只作前端離線快取/暫存（可清除），避免並發鎖檔、不同步與多副本資料分裂。

架構：前後端完全解耦（Nuxt 與 Go 各自容器/CI/CD）；加 API Gateway/反向代理；Repository 介面抽象，支援 SQLite↔MSSQL 切換。

同步與一致性：App 離線寫入（SQLite）→ 上線即雙向同步到雲端 DB；所有金流操作以伺服器原子交易為準，防重入/重複提交。

身份驗證：Gmail OAuth2 登入；後端簽發 JWT + Refresh Token（HttpOnly Cookie），統一錯誤格式與 401 自動換證。

通知：Polling 只限初期；正式版採 FCM/Line Notify/WebSocket 即時通知（提款申請、審核結果、餘額變動）。

前端實務：Nuxt devProxy 需加 `rewrite`；建立 `useApi()` 封裝攔截/錯誤處理；Pinia 狀態持久化；.env 分環境。

部署：Docker multi-stage 瘦身；前後端獨立映像與發布；DB 掛載/備援；未上雲前可本地 SQLite Demo，但不可作正式後端。

MVP 取捨：先做「發錢/提款審核/支出記錄/時間軸/分類/統計」與即時通知；再做頭像上傳與多 Giver-Baby 關聯。







