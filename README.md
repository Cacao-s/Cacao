# Cacao

Developing a Family Allowance Management System

打造家庭零用錢管理App — Nuxt 3 + .NET 8全端實作

### 專案介紹：Nuxt3 + .NET8 API + MSSQL 專案初始化與環境建立

新增 Git Cacao's

新增 Repo CacaoApi 存放 net 8 後端 Api 專案

新增 Repo Cacao 存放 Nuxt 前端專案

#### Ide 準備

Vscode 開發 Cacao Nuxt  專案

vs 2022 開發 net CacaoApi

dbever 連線 mysql

docker Desktop 運行 docker container

### 資料庫設計

Cacao\Cacao\db\CacaoInit.sql

#### 專案初始化

新增專案 CacaoApi  

```powershell
# 初始化 module
go mod init cacaoapi
# 安裝 Gin 套件
go get -u github.com/gin-gonic/gin
go get -u gorm.io/gorm
go get -u gorm.io/driver/mysql
```

新增專案 Cacao 

```

```



### 初始化 Nuxt3 專案，設定 SPA 模式 (`ssr: false`)

1. 安裝 Nuxt 3 專案

如果你還沒建立專案，先建立一個：

```
nuxi init cacaoweb
cd cacaoweb
pnpm install
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

### 初始化 .NET8 API 專案，連接 MSSQL，建置 EF Core Model（DB First）

### Gmail OAuth2 登入流程（前端登入拿 token，後端驗證）

### 完成 Nuxt3 登入流程，登入後自動建立 User 資料（Giver / Baby）

### Nuxt3 建立 Pinia 狀態管理（userStore, walletStore）

### 建立 .NET8 API：User Profile CRUD

### 前端完成 Profile 頁面（個人資料管理）

### 建立 .NET8 API：查詢餘額、發錢（Giver 發錢功能）

### 前端實作 Giver 發錢頁面（指定 Baby 加錢）

### 建立 .NET8 API：提款申請（Baby 發起提款請求）

### 前端實作 Baby 提款申請頁面

### 建立 .NET8 API：提款審核（Giver同意/拒絕）

### 前端實作 Giver 提款審核頁面

### 建立 .NET8 API：支出紀錄（Baby新增支出）

### 前端實作 Baby 支出登記頁面

### 前端實作 Baby 支出歷史紀錄頁面

### 建立 .NET8 API：Giver查看 Baby 消費紀錄

### 前端實作 Giver 消費紀錄頁面（含搜尋/篩選）

------

### Day 21 加入交易時間軸頁面（Timeline 顯示交易歷程）

### Day 22 建立 .NET8 API：上傳使用者頭像（接收 Multipart / 上傳雲端或本地）

### Day 23 Nuxt3 實作頭像上傳功能

### Day 24 加上前端通知功能（Polling 拿提款申請提醒）

------

### Day 25 完成多 Giver 支援單一 Baby 的資料結構與API調整

### Day 26 建立 .NET8 排程服務（HostedService）自動定期加零用錢

### Day 27 加入交易分類系統（食物、玩具等），前端後端同步支援分類

### Day 28 前端實作每月支出統計圖表（Pie Chart、Bar Chart）

------

### Day 29 服務容器化（Docker化 Nuxt3 與 .NET8 API，連接 MSSQL）

```
# 基底 Node.js image
FROM node:20-slim

# 建立工作資料夾
WORKDIR /app

# 複製 package.json 和 package-lock.json
COPY package*.json ./

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

### Day 30 整合測試、修正 Bug、打包正式版、撰寫發表心得

## 1. 整合測試與最後修正

- 全面測試登入、發錢、提款申請、支出記錄、排程加零用錢等功能。
- 確認前端 Nuxt3 與後端 .NET8 API 正常連線，MSSQL 資料一致。

------

## 2. 容器化部署

- 使用 Dockerfile 打包 Nuxt3 前端與 .NET8 API。
- 使用 Docker-Compose 統一管理（Web + API + MSSQL）。
- 本地或雲端（如AWS、Azure、Vercel）部署完整環境。

------

## 3. 選擇正式釋出方式



| 方式                | 說明                                                         |
| ------------------- | ------------------------------------------------------------ |
| Web App             | ✅ 將 Nuxt3 部署成正式網站，供瀏覽器使用                      |
| PWA（推薦）         | ✅ 把 Nuxt3設成PWA，可安裝到手機桌面，像App體驗               |
| WebView App（次選） | ✅ 用 Capacitor/Firebase/WebView 封裝 Nuxt3 網站，打包成 Android APK 上架 Google Play |

------

## 4. PWA 設定（推薦）

- 使用 `@nuxtjs/pwa` 模組
- 設定 manifest.json
- 加入 Service Worker 離線快取
- 支援「加到主畫面」功能（Add to HomeScreen）
- 可透過 Chrome、Safari 安裝成應用程式

範例設定：

```
bash


CopyEdit
npm install @nuxtjs/pwa
```

`nuxt.config.ts` 加上：

```
tsCopyEditexport default defineNuxtConfig({
  modules: ['@nuxtjs/pwa'],
  pwa: {
    manifest: {
      name: 'PocketMoneyApp',
      short_name: 'PocketMoney',
      start_url: '/',
      display: 'standalone',
      background_color: '#ffffff',
      theme_color: '#4F46E5',
      icons: [
        {
          src: '/icon.png',
          sizes: '512x512',
          type: 'image/png',
        }
      ]
    }
  }
})
```

------

## 5. 如果想打包成 APK 上架 Google Play（選項）

- 使用 Capacitor 封裝 Nuxt3 SPA 網址
- 指定 `server.url = 'https://your-webapp-domain.com'`
- 用 Android Studio 打包 APK
- 必須申請 Google Play 開發者帳號（$25一次性）
- 上傳、填資料、審核、上架

------

## 6. 最後提交成果

- 專案原始碼 GitHub Repo
- Demo影片（介紹功能流程）
- 開發心得文章（整理遇到的問題與學到的事）

## 1. 安裝 PWA 模組

```
npm install @vite-pwa/nuxt
```

------

## 2. 在 `nuxt.config.ts` 加上 PWA 設定

```
tsCopyEditimport { defineNuxtConfig } from 'nuxt/config'

export default defineNuxtConfig({
  modules: [
    '@vite-pwa/nuxt',
  ],
  pwa: {
    registerType: 'autoUpdate', // 自動更新新版 Service Worker
    manifest: {
      id: '/',
      name: 'PocketMoney 家庭零用錢管理',
      short_name: 'PocketMoney',
      description: '管理家庭成員的零用錢收支，支援Giver與Baby角色',
      theme_color: '#4F46E5',
      background_color: '#ffffff',
      display: 'standalone',
      start_url: '/',
      scope: '/',
      orientation: 'portrait-primary',
      icons: [
        {
          src: '/icon-192x192.png',
          sizes: '192x192',
          type: 'image/png',
        },
        {
          src: '/icon-512x512.png',
          sizes: '512x512',
          type: 'image/png',
        }
      ]
    },
    workbox: {
      cleanupOutdatedCaches: true,
      navigateFallback: '/',
      runtimeCaching: [
        {
          urlPattern: /^https:\/\/your-api-domain\.com\/api\//,
          handler: 'NetworkFirst',
          options: {
            cacheName: 'api-cache',
            expiration: {
              maxEntries: 50,
              maxAgeSeconds: 300, // 5 minutes
            },
          },
        },
        {
          urlPattern: ({ request }) => request.destination === 'image',
          handler: 'CacheFirst',
          options: {
            cacheName: 'image-cache',
            expiration: {
              maxEntries: 100,
              maxAgeSeconds: 60 * 60 * 24 * 7, // 1 week
            },
          },
        }
      ],
    }
  }
})
```

------

## 3. PWA 必備圖示（放在 `/public`）



| 檔案                       | 尺寸    |
| -------------------------- | ------- |
| `/public/icon-192x192.png` | 192x192 |
| `/public/icon-512x512.png` | 512x512 |

✅ 記得自己產生高解析度 icon，讓裝置加到桌面時好看。

------

## 4. 實現自動更新提示 (Optional)

如果你想讓使用者知道有新版本，可以自己在 Nuxt3 加一個提示：

```
tsCopyEdit// 在 app.vue 加上

<script setup>
import { useRegisterSW } from 'virtual:pwa-register/vue'

const { needRefresh, updateServiceWorker } = useRegisterSW()

const refreshApp = () => {
  updateServiceWorker(true)
}
</script>

<template>
  <div v-if="needRefresh" class="fixed bottom-0 left-0 right-0 bg-blue-600 text-white p-2 text-center z-50">
    發現新版本，<button @click="refreshApp" class="underline">點我更新</button>
  </div>
</template>
```

✅ 這樣當你部署新版本，使用者就會看到「發現新版本」提示！

------

# 最佳化注意事項



| 項目                                     | 建議                                 |
| ---------------------------------------- | ------------------------------------ |
| icon尺寸要齊全                           | 最好準備 192x192, 512x512, 1024x1024 |
| 要有 start_url、scope                    | 不然 PWA 會認不出首頁                |
| display 設定成 standalone                | 這樣加到桌面時沒瀏覽器邊框           |
| theme_color, background_color 要配色好看 | 不然安裝畫面很醜                     |
| registerType 設 autoUpdate               | 確保有新版自動更新 Service Worker    |

# 完成後效果

- 使用者可以「加到主畫面」
- 打開像原生 App，無瀏覽器 UI
- 支援離線瀏覽（支援緩存頁面）
- 新版自動提示更新
- 可以上 Google Play Store（Web App型）

# 總結一句話

> **這樣設定後，你的 Nuxt3 App 安裝到手機上，會幾乎跟原生 App 一模一樣體驗。**

------

✅ 完整 production-ready。
 ✅ 上線後馬上能讓人「加到主畫面」，正式運作！

------

要不要我再順便補一版 👉「專門針對 Android WebAPK最佳化的小技巧（例如隱藏狀態列、全屏設定）」？
 要的話直接說：「要 WebAPK最佳化」，我馬上附上。🚀
 要嗎？

### Nuxt3 專案轉入 Capacitor 專案，設定 Android專案環境



### 測試本地 App（SQLite資料儲存、API呼叫正常），修正所有 Bugs

### 打包 APK，申請 Google Play 開發者帳號，上傳 Google Play Console

## Day ?? 

### 其他功能規劃

### 意外之財，我要先存起來





