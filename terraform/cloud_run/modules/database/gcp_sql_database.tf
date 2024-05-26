resource "google_sql_database_instance" "main" {
  name             = "${var.project_name}-database"
  region           = var.region
  database_version = var.database_version
  settings {
    tier = var.database_tier
  }

  deletion_protection = "true"
}

resource "google_sql_database" "main" {
  name     = var.database_name
  instance = google_sql_database_instance.main.name
}
