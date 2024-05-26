terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.30"
    }
  }

  required_version = "~> 1.8.0"
}

provider "google" {
  credentials = file(var.credentials_file)
  project     = var.project_name
  region      = var.region
}
