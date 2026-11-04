# Cacao 的 GCP 專案只做一件事：讓 App 用 Google 登入，把備份存到使用者自己的雲端硬碟（drive.appdata）。
# 沒有後端、沒有帳單、不放任何使用者資料。OAuth 同意畫面與 Android 用戶端沒有可用的 Terraform 資源，在 Console 畫面設定。

resource "google_project" "cacao" {
  project_id      = var.project_id
  name            = var.project_name
  deletion_policy = "PREVENT"
}

resource "google_project_service" "drive" {
  project            = google_project.cacao.project_id
  service            = "drive.googleapis.com"
  disable_on_destroy = false
}
