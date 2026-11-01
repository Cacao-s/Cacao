---
name: tech-research
description: 技術研究 - 在已定案的 Flutter、無後端前提下，研究剩下的技術選型（本地資料庫、備份、裝置直連同步、狀態管理、主題）。當需要比較技術方案或做架構決策時使用。
disable-model-invocation: true
allowed-tools: Read Edit Write WebSearch WebFetch
---

# 技術研究

先讀取 `docs/tech-decisions.md` 確認已定案的決策，再讀 `SPEC.md` 與 `docs/product/` 確認需求，然後只研究**還沒定案**的技術點。

## 已定案（不再比較）

以 `docs/tech-decisions.md` 為準，目前包含：

- **D-01 開發框架：Flutter**（iOS／Android 單一程式碼）
- **D-02 完全不用後端**：不自建，也不依賴任何第三方後端（Supabase、Firebase、PowerSync 等都不列入）

已定案的項目不重新比較、不列入比較表。若新需求真的動搖了既有決策，先回報衝突與理由，由用戶決定是否重開，不要自行改寫。

## 研究範圍（皆以 Flutter、無後端為前提）

### 1. 本地資料庫
- Drift（SQLite）
- 其他 Flutter SQLite 方案
- 評估重點：離線、資料遷移（schema migration）、適合「只新增、不覆蓋」的流水帳模型

### 2. 備份與還原
- 匯出／匯入備份檔（使用者自己保存）
- Google 雲端硬碟（appDataFolder）——是否採用依 `docs/product/discover-qa.md` WD-21 的結論
- 評估重點：換手機還原、備份檔加密

### 3. 裝置之間同步（第二版，依 WD-20 的結論）
- 同一個 Wi-Fi 下裝置直連（區網探索＋直接連線）
- 評估重點：iOS／Android 互通、兩台各自記帳後的合併方式

### 4. 狀態管理
- Riverpod
- Bloc

### 5. 主題與 Design Token
- 用 `ThemeData`／`ThemeExtension` 承載 design token，支援多主題切換
- 字型策略：中文用系統字型，只打包英文數字字型，控制 App 大小

## 評估條件

每個方案都用同一組條件比較：

1. 離線運作：完全不需要網路
2. 不依賴任何第三方後端
3. 對 App 大小的影響
4. 維護狀態：最近是否還有更新、社群規模
5. 學習與實作成本

## 執行步驟

1. 讀取 `docs/tech-decisions.md`、`SPEC.md`、`docs/product/`
2. 只挑「研究範圍」中尚未定案的項目；依賴 discover 待確認題目（WD）的項目，若 WD 還沒回答就先略過並回報
3. 使用 WebSearch 查最新資訊（版本、維護狀態、已知限制）
4. 把每個待決定的技術點寫成題目，放進 `docs/tech-qa.md` 的「待確認」，**不在對話裡等回答**。格式照 `docs/product/discover-qa.md`：
   - 一題只問一件事，編號 `TQ-xx`，往後接、不重用
   - 需要比較時附比較表（方案 × 評估條件），再列編號選項
   - 每題都附建議與理由；需要用戶做產品取捨的題目標 ⚠
5. 對話只回報：新增了哪幾題、建議先回答哪一題
6. 用戶說「已回答」時：重新讀取 `docs/tech-qa.md`，把有明確答案的題目移到「已確認」並從「待確認」刪掉；同時把決策追加到 `docs/tech-decisions.md`，並更新 `SPEC.md` 中對應的技術規格。回答有疑慮或引出新問題時，重新查證後留在或新增到「待確認」，不要自行拍板

## 輸出格式

`docs/tech-decisions.md` 每項決策包含：

- 決策背景
- 方案比較表（已定案的方案不列）
- 最終選擇與理由
- 風險評估
