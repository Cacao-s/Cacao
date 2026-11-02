# shared

兩個平台共用的「資料」，只有一份。Android 與 iOS 只讀不改；要改就改這裡，再用 `tools/` 重新產生各平台的程式碼。

- `db/`：SQLite schema 與 migration（`001_init.sql` 起依序編號）。規則見 `docs/data-model.md`。
- `tokens/`：4 個主題的 design token JSON（D-19）。
- `i18n/`：繁中、英文翻譯來源（D-20）。
- `test-cases/`：計算的共用測試案例 JSON，兩平台單元測試都跑（D-23）。
- `animations/`：Lottie 動畫檔（D-22）。
