//  gcloud compute addresses describe grafana-ip --region=asia-northeast1 --project=market-streaming-prod --format="get(address)" 

resource "google_compute_global_address" "grafana_global_ip" {
  name    = "grafana-global-ip"
  project = var.project_id
}

output "grafana_global_ip" {
  value = google_compute_global_address.grafana_global_ip.address
}
