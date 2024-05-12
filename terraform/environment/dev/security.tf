module "security" {
  source = "../../modules/security"

  project_name   = var.project_name
  vpc_id         = module.network.vpc_id
  vpc_cidr_block = var.vpc_cidr_block
}
