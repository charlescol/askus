resource "google_compute_disk" "gce-pv-disk" {
  name                      = "gce-pv-disk"
  physical_block_size_bytes = 4096
  size                      = 10
  type                      = "pd-ssd"
  zone                      = var.zone
  project                   = var.project_id

  lifecycle {
    prevent_destroy = true
    ignore_changes = [
      snapshot,
      source_snapshot_id,
      zone,
    ]
  }
}


resource "google_compute_resource_policy" "gce_pv_disk_backup" {
  name   = "gce-pv-disk-backup"
  region = var.region

  snapshot_schedule_policy {
    schedule {
      daily_schedule {
        days_in_cycle = 1
        start_time    = "01:00"
      }
    }
    retention_policy {
      max_retention_days    = 3
      on_source_disk_delete = "APPLY_RETENTION_POLICY"
    }
    snapshot_properties {
      guest_flush = true
      labels = {
        application = "grafana"
        db_type     = "sqlite"
      }
    }
  }
}

resource "google_compute_disk_resource_policy_attachment" "gce_pv_disk_attachment" {
  name    = google_compute_resource_policy.gce_pv_disk_backup.name
  disk    = google_compute_disk.gce-pv-disk.name
  zone    = google_compute_disk.gce-pv-disk.zone
  project = var.project_id
}
