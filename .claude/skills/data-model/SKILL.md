---
name: data-model
description: 資料模型設計 - 設計離線優先的資料庫 Schema。當需要設計資料結構時使用。
disable-model-invocation: true
allowed-tools: Read Edit
---

# 資料模型設計

先讀取 SPEC.md 確認實體和關係，然後設計 Schema。

## 設計原則

1. **離線優先** - 所有資料先存本地，網路可用時再同步
2. **衝突處理** - 多設備同步時的衝突解決策略
3. **隱私安全** - 敏感資料加密存儲

## 執行步驟

### 第一步：識別實體
根據 SPEC.md 識別核心實體：
- User（用戶帳號）
- Family（家庭/家族）
- Parent（父母 - 可出金的長輩）
- Elder（長輩 - 可設任務但不能出金）
- Child（孩子）
- Task（任務）
- Allowance（零用錢記錄）
- Goal（儲蓄目標）
- Transaction（交易記錄）

### 第二步：定義關係
```
Family 1:N Parent
Family 1:N Elder
Family 1:N Child
Parent/Elder 1:N Task
Child N:N Task
Child 1:N Goal
Child 1:N Transaction
Task 1:1 Allowance (獎勵型)
```

### 第三步：設計 Schema
使用 WatermelonDB 格式：

```javascript
tableSchema({
  name: 'children',
  columns: [
    { name: 'name', type: 'string' },
    { name: 'family_id', type: 'string', isIndexed: true },
    { name: 'balance', type: 'number' },  // 目前餘額
    { name: 'created_at', type: 'number' },
    { name: 'updated_at', type: 'number' },
  ]
})

tableSchema({
  name: 'tasks',
  columns: [
    { name: 'title', type: 'string' },
    { name: 'reward_amount', type: 'number' },
    { name: 'creator_id', type: 'string' },  // Parent 或 Elder
    { name: 'child_id', type: 'string' },
    { name: 'verify_password_hash', type: 'string' },  // 驗證密碼
    { name: 'status', type: 'string' },  // pending/completed/verified
    { name: 'created_at', type: 'number' },
  ]
})
```

### 第四步：同步策略
- 哪些資料需要雲端備份？
- 備份頻率？
- 衝突解決規則？

## 完成後

將設計寫入 `docs/data-model.md`，包含：
- ER Diagram（文字描述）
- WatermelonDB Schema 定義
- 同步策略說明
