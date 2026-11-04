# infra — Cacao 的雲端資源（Infrastructure as Code）

Cacao 沒有後端。雲端資源只有一個 GCP 專案，用途只有一個：讓 App 用 Google 登入，把備份存到**使用者自己的**雲端硬碟（`drive.appdata`）。

## 規則

- 用 API／指令建立或修改的雲端資源，一律寫進 `infra/`（能用 Terraform 就用 Terraform），並在下方〈操作紀錄〉記一筆。
- 只能在 Console 畫面設定的（OAuth 同意畫面、Android OAuth 用戶端、Play Console）不在這裡記錄。
- 不放任何密碼、金鑰、token；state 只在本機（已 gitignore）。

## 結構

```
infra/
├── README.md        # 本檔：規則、用法、操作紀錄
└── gcp/             # GCP 專案（Terraform root module，一個 root＝一份 state）
    ├── versions.tf  # Terraform／provider 版本
    ├── variables.tf
    ├── terraform.tfvars   # 非機密的值（專案 ID）
    ├── main.tf      # 專案、啟用的 API
    ├── imports.tf   # 把先用 gcloud 建的資源收進 state
    └── outputs.tf
```

之後有新的雲端（例如 Phase 2 的 iOS／CI 需要的資源）就在 `infra/` 下開平行資料夾（`infra/<平台>/`），各自一份 state；共用的寫成 `infra/modules/`。

## 用法

工具裝在 `D:\git\Android\`（`terraform\`、`gcloud\`）。

```bash
# 登入一次（瀏覽器）
gcloud auth login
# Terraform 直接借用 gcloud 的 token，不需要另外的金鑰檔
export GOOGLE_OAUTH_ACCESS_TOKEN=$(gcloud auth print-access-token)

cd infra/gcp
terraform init
terraform plan      # 應該是 No changes
terraform apply
```

## State

- 目前用**本機 state**（`infra/gcp/terraform.tfstate`，已 gitignore）。這台電腦以外的人要接手，先 `terraform import`（`imports.tf` 已寫好）就能重建 state。
- 遠端 state（GCS bucket）需要綁帳單帳戶，Phase 2 有 CI 時再搬。

## 操作紀錄

只記用 API／指令做的事。

- 2026-11-04 `gcloud auth login`：以 amanda1991910@gmail.com 登入。
- 2026-11-04 `gcloud projects create cacao-ruru-app --name=Cacao`：建立專案（專案編號 622558914718），附帶自動啟用 `cloudapis.googleapis.com`。
- 2026-11-04 `gcloud services enable drive.googleapis.com`：啟用 Google Drive API。
- 2026-11-04 `terraform apply`（只有 import）：把上面兩項收進 Terraform 管理，`plan` 無差異。
