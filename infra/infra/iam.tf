

data "google_service_account" "terraform" {
  account_id = "terraform"
  project    = var.project_id
}


data "google_service_account" "cloud_run_runtime" {
  account_id = "cloud-run-runtime"
  project    = var.project_id
}
