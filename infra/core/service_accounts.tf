resource "google_service_account" "terraform" {
  account_id   = "terraform"
  display_name = "Terraform Bootstrap / IaC"
  description  = "Service account for Terraform IaC"
}

resource "google_service_account_key" "tf_key" {
  service_account_id = google_service_account.terraform.name
  keepers = {
    generation = 1
  }
}

resource "google_service_account" "container_deployer" {
  account_id   = "container-deployer"
  display_name = "Container Deployer"
  description  = "Service account for deploying containers"
}
