# Agent CLI 說明

位置：`infra/Agent/`

長期規則：`infra/Agent/AGENTS.md`

## Node 版 (快速互動)
- 指令：`npm run agent:node`
- 依賴：Node 20+、環境變數 `OPENAI_API_KEY`（可選 `OPENAI_MODEL`，預設 `gpt-4o-mini`）。
- 路徑：`infra/Agent/agent-node.js`
- 退出：輸入 `:q`

## Go 版 (獨立於產品)
- 路徑：`infra/Agent/go/`
- 檔案：`infra/Agent/go/main.go`，`go.mod`
- 執行：
  ```bash
  set OPENAI_API_KEY=your_key_here
  cd infra/Agent/go
  go run .
  ```
  或建置：`go build -o agent-go .`，再執行 `./agent-go`
- 可選旗標：
  - `-prompt <path>` 自訂 system prompt 路徑（預設 `../agent-prompt.md`）
  - `-model <name>`、`-url <endpoint>` 覆寫模型與 API endpoint

## 說明
- 兩個版本共享同一份長期規則檔，彼此路徑獨立，不影響產品程式碼。
- 如需搬移規則檔，請同步更新上述路徑或啟動旗標。
