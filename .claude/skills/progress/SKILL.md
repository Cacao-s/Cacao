---
name: progress
description: 進度管理與專案狀態記錄，確認目前做什麼、下一步要做什麼，以及對應的 skill 負責人。使用者可透過 `/progress` 或詢問「現在進度為何」時觸發。
allowed-tools: Read Edit WebSearch
---

# 進度管理

當需要檢視或更新專案進度時，請使用這個 skill。它會生成或更新 `docs/progress.md`，並說明目前工作、下一步與對應的 skill 負責人。

這個 skill 的目標是：
- 讓每次開專案時都能清楚知道目前正在做什麼
- 明確列出下一個要處理的工作
- 用進度條表示完成程度
- 標註每項工作對應的 skill 負責人
- 將結果寫入 `docs/progress.md`

## 任務內容

1. 讀取現有專案文件與 skill 結構，例如 `SPEC.md`、`docs/` 及 `.claude/skills/`
2. 判斷目前已完成的工作、正在進行中的工作、以及下一步要處理的工作
3. 以專案狀態報告形式產出：
   - 當下重點（Current focus）
   - 下一步（Next action）
   - 進度條（Progress bar）
   - 工作列表與對應 skill 負責人
4. 將進度寫入 `docs/progress.md`，並且如有必要更新報告內容

## 使用時機

- 這個 skill 可透過 `/progress` 或使用者直接問「現在進度為何」時觸發
- 若模型判斷出這個 skill 與問題相關，會自動載入並執行；若要手動執行，請使用 slash command
- 如果 `docs/progress.md` 尚未存在，請建立它並初始化
- 如果已有進度文件，請檢查是否需要更新內容或補上缺漏

> 注意：`SKILL.md` 本身不支援「每次開啟專案時自動執行」的 hook。若要在專案啟動時自動執行，需另外配置 VS Code agent hook 或專案啟動指令。

## 產出格式

報告應包含：
- `Current focus`：目前正在處理的核心任務
- `Next action`：下一個優先做的項目
- `Progress`：明確進度條與百分比
- `Task` / `Status` / `Skill owner` / `Notes` 表格
- `Blockers`（若有）

例子：

```md
# 專案進度

- Current focus: 進行 `progress` skill 的完善與自動觸發設計
- Next action: 定義進度文件格式並撰寫初始進度報告
- Progress: [#####-----] 50%

| Task | Status | Skill owner | Notes |
|------|--------|-------------|-------|
| 進度管理 skill | In progress | progress | 需要自動觸發與進度文件產生 |
| 認證方案研究 | Done | auth-design | 已完成初步選型 |
| 技術選型 | Next | tech-research | 需要確認框架與資料庫 |
```

## 完成後

- `docs/progress.md` 包含專案狀態摘要與工作對應表
- 每次啟動專案時能快速回顧當前進度
- skill 與專案工作之間的責任對應清楚
