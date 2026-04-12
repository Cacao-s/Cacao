# Cacao 專案指南

## 專案簡介

Cacao 是一個**家庭零用錢管理**行動應用程式，支援 iOS 和 Android。
讓家庭/家族成員共同管理孩子的零用錢、任務獎勵和儲蓄目標。

## 專案原則

1. **離線優先** - 所有功能都要能離線運作
2. **無後端** - 盡量不依賴自建後端，必要時用 Supabase
3. **隱私優先** - 用戶資料存本地，備份由用戶控制

## 文件結構

```
Cacao/
├── SPEC.md              # 產品規格書
├── CLAUDE.md            # AI 指南（本檔案）
├── docs/
│   ├── ai-guide.md      # 人類可讀的 AI 工具教學
│   ├── tech-decisions.md
│   ├── data-model.md
│   └── auth-design.md
└── .claude/
    └── skills/          # Claude Code Skills
        ├── discover/
        ├── tech-research/
        ├── data-model/
        └── auth-design/
```

## 開發階段

- [ ] 階段 1：產品定義（/discover）
- [ ] 階段 2：技術選型（/tech-research）
- [ ] 階段 3：架構設計（/data-model, /auth-design）
- [ ] 階段 4：開發實作
