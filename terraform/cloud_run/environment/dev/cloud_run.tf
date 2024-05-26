module "cloud_run" {
  source = "../../modules/cloud_run"

  project_name = var.project_name
  location     = var.region
  image        = var.container_image

  max_scale = var.max_scale

  database_connection_name = module.database.database_connection_name
}

# Variables
variable "container_image" {
  default = "us-docker.pkg.dev/cloudrun/container/hello"
}

variable "max_scale" {
  default = 5
}
