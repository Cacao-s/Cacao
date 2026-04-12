---
name: tech-research
description: 技術研究 - 框架選型、技術決策。當需要比較技術方案或做架構決策時使用。
disable-model-invocation: true
allowed-tools: Read Edit WebSearch WebFetch
---

# 技術研究

先讀取 SPEC.md 確認需求，然後針對技術點進行研究。

## 研究範圍

### 1. 開發框架
- React Native
- Expo
- Flutter

### 2. 資料庫方案
- WatermelonDB + SQLite
- Realm
- PouchDB

### 3. 認證方案（無後端）
- Supabase Auth
- Firebase Auth

### 4. 雲端備份
- Google Drive API
- Supabase Storage

## 執行步驟

1. 讀取 SPEC.md 確認功能需求
2. 使用 WebSearch 研究各方案的最新資訊
3. 產出比較表
4. 給出建議並說明理由
5. 將決策寫入 `docs/tech-decisions.md`

## 輸出格式

產出的文件應包含：
- 決策背景
- 方案比較表
- 最終選擇與理由
- 風險評估
