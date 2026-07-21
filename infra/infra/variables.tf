variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "zone" {
  description = "Default zone"
  type        = string
  sensitive   = true
}

variable "region" {
  description = "Google Cloud region"
  type        = string
}

variable "cloud_run_name" {
  description = "Cloud Run service name"
  type        = string
  default     = "app"
}

variable "cloud_run_image" {
  description = "Public Docker Hub image"
  type        = string
}

