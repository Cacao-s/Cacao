# 查看 WatermelonDB SQLite 資料庫

## 資料庫位置

WatermelonDB 在 Android 上會將 SQLite 資料庫儲存在:
```
/data/data/com.cacao.app/databases/cacao.db
```

## 方法 1: 使用 ADB 導出資料庫 (推薦)

### 1. 確認裝置連接
```bash
adb devices
```

### 2. 導出資料庫檔案
```bash
# Android (Expo Go)
adb exec-out run-as host.exp.exponent cat /data/data/host.exp.exponent/databases/cacao.db > cacao.db

# Android (Development Build)
adb exec-out run-as com.cacao.app cat /data/data/com.cacao.app/databases/cacao.db > cacao.db
```

### 3. 使用 SQLite 工具查看

**DB Browser for SQLite** (推薦)
- 下載: https://sqlitebrowser.org/
- 開啟匯出的 `cacao.db` 檔案
- 可以直接查看所有 table 和資料

**命令列**:
```bash
sqlite3 cacao.db
.tables
.schema users
SELECT * FROM users;
```

## 方法 2: 在 App 內建查詢工具

我可以建立一個內建的資料庫檢視器,讓你直接在 App 內查看資料!

## 方法 3: Reactotron (開發工具)

安裝 Reactotron 可以在開發時即時查看 WatermelonDB 資料。

## 當前測試資料結構

執行「建立測試資料」後會有:

### users table
```
id | email              | display_name    | role  | password_hash (SHA256)
---+--------------------+----------------+-------+------------------------
1  | giver@example.com  | Primary Giver  | giver | <hash>
2  | baby@example.com   | Little Baby    | baby  | <hash>
3  | parent@example.com | Caring Parent  | giver | <hash>
```

### families table
```
id | name        | currency | timezone     | created_by
---+-------------+----------+--------------+-----------
1  | Demo Family | TWD      | Asia/Taipei  | 1
```

### wallets table
```
id | name               | type | balance_cents | currency
---+--------------------+------+--------------+----------
1  | Baby's Cash        | cash | 50000        | TWD
2  | Baby's Bank Account| bank | 1000000      | TWD
```

## 我建議的解決方案

你想要哪一種?
1. **快速方案**: 我教你用 ADB 導出並用 DB Browser 查看
2. **開發方案**: 我在 App 內建立資料庫檢視器畫面
3. **專業方案**: 設定 Reactotron 整合

選哪一個?
