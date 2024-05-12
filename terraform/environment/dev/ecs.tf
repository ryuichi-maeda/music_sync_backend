module "ecs" {
  source = "../../modules/ecs"

  project_name = var.project_name

  ecs_task_cpu    = var.ecs_task_cpu
  ecs_task_memory = var.ecs_task_memory

  ecs_container_name = var.ecs_container_name
  ecs_container_port = var.ecs_container_port
  ecs_ec2_port       = var.ecs_ec2_port

  ecr_img                       = "${var.ecr_repo}:${var.ecr_repo_tag}"
  cloudwatch_log_retention_days = var.cloudwatch_log_retention_days
  ecs_logs_region               = var.ecs_logs_region
  ecs_logs_stream_prefix        = var.ecs_logs_stream_prefix

  ecs_ec2_instance_type = var.ecs_ec2_instance_type
  ecs_sg_ids = [
    module.security.ecs_sg_id
  ]

  alb_sg_ids = [
    module.security.alb_sg_id
  ]
  alb_subnets = [
    module.network.public_subnet1_id,
    module.network.public_subnet2_id
  ]

  vpc_id = module.network.vpc_id
  ecs_subnets = [
    module.network.public_subnet1_id,
    module.network.public_subnet2_id
  ]

  ecs_as_min_size         = var.ecs_as_min_size
  ecs_as_max_size         = var.ecs_as_max_size
  ecs_as_desired_capacity = var.ecs_as_desired_capacity

  maximum_scaling_step_size = var.maximum_scaling_step_size
  minimum_scaling_step_size = var.minimum_scaling_step_size
  scaling_target_capacity   = var.scaling_target_capacity

  ecs_task_desired_count = var.ecs_as_desired_capacity
}

# Variables
variable "ecs_task_cpu" {
  default = 256
}
variable "ecs_task_memory" {
  default = 512
}

variable "ecs_container_name" {
  default = "music-sync-ecs-container"
}
variable "ecs_container_port" {
  default = 80
}
variable "ecs_ec2_port" {
  default = 80
}

variable "ecr_repo" {
  default = "nginx"
}
variable "ecr_repo_tag" {
  default = "latest"
}

variable "cloudwatch_log_retention_days" {
  default = 30
}
variable "ecs_logs_region" {
  default = "ap-northeast-1"
}
variable "ecs_logs_stream_prefix" {
  default = "ecs"
}

variable "ecs_ec2_instance_type" {
  default = "t2.micro"
}

variable "ecs_as_min_size" {
  default = 1
}
variable "ecs_as_max_size" {
  default = 2
}
variable "ecs_as_desired_capacity" {
  default = 2
}

variable "maximum_scaling_step_size" {
  default = 10
}
variable "minimum_scaling_step_size" {
  default = 1
}
variable "scaling_target_capacity" {
  default = 100
}

variable "ecs_task_desired_count" {
  default = 2
}
