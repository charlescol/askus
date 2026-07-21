variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "Default region"
  type        = string
}

variable "zone" {
  description = "Default zone"
  type        = string
  sensitive   = true
}

variable "cluster_name" {
  type        = string
  description = "Name of the GKE cluster"
}

variable "services" {
  type        = list(string)
  description = "Services to deploy"
}
