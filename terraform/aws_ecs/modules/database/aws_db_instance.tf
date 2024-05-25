resource "aws_db_subnet_group" "main" {
  name       = "${var.project_name}-db-subnet-group"
  subnet_ids = var.db_subnet_ids

  tags = {
    Name = "${var.project_name}-db-subnet-group"
  }
}

resource "aws_db_instance" "main" {
  identifier              = "${var.project_name}-db"
  engine                  = var.db_engine
  engine_version          = var.db_engine_version
  instance_class          = var.db_instance_class
  allocated_storage       = var.db_allocated_storage
  storage_type            = var.db_storage_type
  db_subnet_group_name    = aws_db_subnet_group.main.name
  vpc_security_group_ids  = var.db_security_group_ids
  skip_final_snapshot     = true
  backup_retention_period = var.db_backup_retention_period

  # TODO: Add db name
  username = var.db_username
  password = var.db_password

  tags = {
    Name = "${var.project_name}-db"
  }
}
