module "database" {
  source = "../../modules/database"

  project_name = var.project_name

  db_subnet_ids = [
    module.network.private_subnet1_id,
    module.network.private_subnet2_id
  ]

  db_security_group_ids = [
    module.security.rds_sg_id
  ]

  db_engine                  = var.db_engine
  db_engine_version          = var.db_engine_version
  db_instance_class          = var.db_instance_class
  db_allocated_storage       = var.db_allocated_storage
  db_storage_type            = var.db_storage_type
  db_backup_retention_period = var.db_backup_retention_period

  db_username = var.db_username
  db_password = var.db_password
}

# Variables
variable "db_engine" {
  default = "mysql"
}
variable "db_engine_version" {
  default = "8.0.36"
}
variable "db_instance_class" {
  default = "db.t3.micro"
}
variable "db_allocated_storage" {
  default = 20
}
variable "db_storage_type" {
  default = "gp2"
}
variable "db_backup_retention_period" {
  default = 7
}

variable "db_username" {
}
variable "db_password" {
}
