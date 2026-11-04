# 2026-11-04 先用 gcloud 建立，這裡把既有資源收進 state；import 完成後可保留（重跑無作用）
import {
  to = google_project.cacao
  id = "cacao-ruru-app"
}

import {
  to = google_project_service.drive
  id = "cacao-ruru-app/drive.googleapis.com"
}
