# AI 工具使用指南

本專案使用 Claude Code 作為 AI 開發助手。以下是可用的 skills 和使用方法。

## 快速開始

在 Claude Code 中輸入 `/` 可以看到所有可用的 skills。

## 可用的 Skills

### `/discover` - 產品探索

**用途**：釐清產品需求、問對問題

**使用時機**：
- 開始新專案時
- 不確定要做什麼功能時
- 需要整理用戶需求時

**範例**：
```
/discover
```

Claude 會讀取 SPEC.md，然後引導你回答一系列問題來釐清需求。

---

### `/tech-research` - 技術研究

**用途**：技術選型與架構決策（框架已定案為 Flutter）

**使用時機**：
- 需要比較不同技術方案時
- 做架構決策時

**範例**：
```
/tech-research
```

Claude 會研究各種技術方案並產出比較報告。

---

### `/design` - UI/UX 設計

**用途**：整理畫面清單、導覽結構、主要流程與低保真線框

**使用時機**：
- 產品規格定案後、設計資料模型之前
- 需要規劃畫面或使用流程時

**範例**：
```
/design
```

Claude 會把有疑問的地方寫進 `docs/design-qa.md`，等你回答後再完成設計。

---

### `/data-model` - 資料模型設計

**用途**：設計資料庫 Schema

**使用時機**：
- 定義資料結構時
- 設計本地資料庫 Schema 時

**範例**：
```
/data-model
```

---

### `/auth-design` - 認證架構設計

**用途**：設計登入和備份流程

**使用時機**：
- 規劃認證系統時
- 設計 Google 登入流程時

**範例**：
```
/auth-design
```

---

## 文件輸出位置

各 skill 產出的文件會放在：

| Skill | 輸出檔案 |
|-------|----------|
| `/discover` | 更新 `SPEC.md` |
| `/tech-research` | `docs/tech-decisions.md` |
| `/design` | `docs/design.md` |
| `/data-model` | `docs/data-model.md` |
| `/auth-design` | `docs/auth-design.md` |

---

## 建議的開發流程

1. **產品定義** → `/discover`
2. **技術選型** → `/tech-research`
3. **UI/UX 設計** → `/design`
4. **架構設計** → `/data-model` + `/auth-design`
5. **開發實作** → 開始寫 code

---

## 參考資料

- [Claude Code Skills 官方文檔](https://code.claude.com/docs/en/skills)
- [CLAUDE.md 最佳實踐](https://www.builder.io/blog/claude-md-guide)
