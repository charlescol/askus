output "cloud_run_url" {
  description = "Cloud Run service URL"
  value       = google_cloud_run_v2_service.app.uri
}

output "cloud_run_runtime_service_account" {
  description = "Runtime service account"
  value       = data.google_service_account.cloud_run_runtime.email
}
