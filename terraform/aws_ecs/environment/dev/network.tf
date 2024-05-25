module "network" {
  source = "../../modules/network"

  project_name = var.project_name

  vpc_cidr_block   = var.vpc_cidr_block
  route_cidr_block = var.route_cidr_block

  public_subnet1_cidr_block  = var.public_subnet1_cidr_block
  public_subnet2_cidr_block  = var.public_subnet2_cidr_block
  private_subnet1_cidr_block = var.private_subnet1_cidr_block
  private_subnet2_cidr_block = var.private_subnet2_cidr_block

  availability_zone1 = var.availability_zone1
  availability_zone2 = var.availability_zone2
}

# Variables
variable "vpc_cidr_block" {
  default = "10.0.0.0/16"
}

variable "route_cidr_block" {
  default = "0.0.0.0/0"
}

variable "public_subnet1_cidr_block" {
  default = "10.0.1.0/24"
}
variable "public_subnet2_cidr_block" {
  default = "10.0.2.0/24"
}
variable "private_subnet1_cidr_block" {
  default = "10.0.3.0/24"
}
variable "private_subnet2_cidr_block" {
  default = "10.0.4.0/24"
}

variable "availability_zone1" {
  default = "ap-northeast-1a"
}
variable "availability_zone2" {
  default = "ap-northeast-1c"
}
