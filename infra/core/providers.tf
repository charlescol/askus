terraform {
  required_version = ">= 1.7.5"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 6.0" 
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "~> 6.0"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}


variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "Region for bucket"
  type        = string
}