# tools

兩個平台共用的「工具」。

- `codegen/`：把 `shared/tokens`、`shared/i18n` 產生成 Kotlin（Compose）與 Swift（SwiftUI）程式碼。
- `e2e/`：Maestro 端對端流程，同一份 Android 與 iOS 都跑。
- `ci/`：CI 共用腳本；`.github/workflows/` 只放觸發設定，呼叫這裡。
- `scripts/`：開發用指令（初始化、跑全部測試等）。
