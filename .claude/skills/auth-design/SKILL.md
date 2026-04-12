---
name: auth-design
description: 認證架構設計 - 設計登入和備份流程。當需要規劃認證系統時使用。
disable-model-invocation: true
allowed-tools: Read Edit WebSearch
---

# 認證架構設計

先讀取 SPEC.md 確認認證需求，然後設計流程。

## 設計目標

根據 SPEC.md：
1. 帳號密碼註冊/登入
2. Google 帳號登入
3. 資料備份到 Google 帳號
4. 換機時資料恢復

## 需要確認的問題

向用戶確認：
1. 是否允許匿名使用？（不登入直接用）
2. 登入後才能備份？還是本地也能用？
3. 多設備登入同一帳號的行為？

## 核心問題分析

### 問題 1：無後端如何認證？
- **Supabase Auth** - 免費額度大、整合簡單
- **Firebase Auth** - Google 生態、穩定

### 問題 2：認證 vs 備份
- 認證：驗證用戶身份
- 備份：存儲用戶資料到 Google Drive
- 這兩者是否需要同一個 Google 帳號？

### 問題 3：離線場景
- 用戶離線時能否使用 app？
- Token 過期時的處理？

## 執行步驟

1. 讀取 SPEC.md
2. 向用戶確認上述問題
3. 比較 Supabase vs Firebase
4. 設計流程圖
5. 將設計寫入 `docs/auth-design.md`

## 完成後

產出文件包含：
- 認證服務選擇與理由
- 流程圖（Mermaid 格式）
- 安全考量
- 實作注意事項
