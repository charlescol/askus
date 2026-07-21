

data "google_service_account" "terraform" {
  account_id = "terraform"
  project    = var.project_id
}

data "google_service_account" "gke_nodes" {
  account_id = "gke-node"
  project    = var.project_id
}
