terraform {
  required_version = ">= 1.9"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 6.0"
    }
  }

  # 第一版用本機 state（已 gitignore）；GCS 後端需要綁帳單，Phase 2 再搬
}

provider "google" {
  project = var.project_id
}
