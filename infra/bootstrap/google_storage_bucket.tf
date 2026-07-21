resource "google_storage_bucket" "tfstate" {
  name                        = "tfstate-${var.project_id}"
  location                    = var.region
  uniform_bucket_level_access = true

  force_destroy = true
  versioning { enabled = true }
}

output "bucket_name" {
  value = google_storage_bucket.tfstate.name
}
