module "database" {
  source = "../../modules/database"

  project_name     = var.project_name
  region           = var.region
  database_version = var.database_version
  database_tier    = var.database_tier

  database_name     = var.database_name
  database_user     = var.database_user
  database_password = var.database_password
}

# Variables
variable "database_version" {
  default = "MySQL_8_0"
}
variable "database_tier" {
  default = "db-f1-micro"
}

variable "database_name" {}
variable "database_user" {}
variable "database_password" {}
