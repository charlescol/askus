resource "google_container_cluster" "gke_cluster" {
  name     = var.cluster_name
  location = var.zone
  project  = var.project_id

  deletion_protection      = false
  remove_default_node_pool = true
  initial_node_count       = 1

  network    = "projects/market-streaming-prod/global/networks/default"
  subnetwork = "projects/market-streaming-prod/regions/asia-northeast1/subnetworks/default"

  ip_allocation_policy {
    cluster_ipv4_cidr_block  = "10.20.0.0/16"
    services_ipv4_cidr_block = "10.30.0.0/20"
  }

  logging_service    = "none"
  monitoring_service = "none"

  lifecycle {
    ignore_changes = [initial_node_count]
  }
}

resource "google_container_node_pool" "ssd_pool" {
  name       = "ssd-node-pool"
  cluster    = google_container_cluster.gke_cluster.name
  location   = var.zone
  project    = var.project_id
  node_count = 3

  node_config {
    machine_type = "n2d-standard-8" # Check availability: gcloud compute machine-types describe {machine_type} --zone={zone}

    confidential_nodes {
      enabled = false
    }
    disk_size_gb                = 50
    disk_type                   = "pd-ssd"
    enable_confidential_storage = false
    local_ssd_count             = 2 #  375 GiB per disk


    oauth_scopes = [
      "https://www.googleapis.com/auth/cloud-platform",
    ]

    service_account = data.google_service_account.gke_nodes.email
    metadata = {
      disable-legacy-endpoints = "true"
    }
    tags   = ["gke-node", "ssd-node"]
    labels = { pool = "ssd" }

    taint {
      key    = "local-ssd"
      value  = "true"
      effect = "NO_SCHEDULE"
    }
  }

  management {
    auto_upgrade = true
    auto_repair  = true
  }
}

resource "google_container_node_pool" "standard_pool" {
  name       = "standard-node-pool"
  cluster    = google_container_cluster.gke_cluster.name
  location   = var.zone
  project    = var.project_id
  node_count = 5

  node_config {
    machine_type = "n2d-standard-16" # Check availability: gcloud compute machine-types describe {machine_type} --zone={zone}

    confidential_nodes {
      enabled = false
    }
    disk_size_gb                = 50
    disk_type                   = "pd-ssd"
    enable_confidential_storage = false

    oauth_scopes = [
      "https://www.googleapis.com/auth/cloud-platform",
    ]

    service_account = data.google_service_account.gke_nodes.email
    metadata = {
      disable-legacy-endpoints = "true"
    }
    tags   = ["gke-node", "standard-node"]
    labels = { pool = "standard" }
  }

  management {
    auto_upgrade = true
    auto_repair  = true
  }
}
