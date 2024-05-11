variable "project_name" {}

variable "ecs_task_cpu" {}
variable "ecs_task_memory" {}

variable "ecs_container_name" {}
variable "ecs_container_port" {}
variable "ecs_ec2_port" {}

variable "ecr_img" {}

variable "cloudwatch_log_retention_days" {}
variable "ecs_logs_region" {}
variable "ecs_logs_stream_prefix" {}

variable "ecs_ec2_instance_type" {}

variable "ecs_sg_ids" {}

variable "alb_sg_ids" {}
variable "alb_subnets" {}

variable "vpc_id" {}
variable "ecs_subnets" {}

variable "ecs_as_min_size" {}
variable "ecs_as_max_size" {}
variable "ecs_as_desired_capacity" {}

variable "maximum_scaling_step_size" {}
variable "minimum_scaling_step_size" {}
variable "scaling_target_capacity" {}

variable "ecs_task_desired_count" {}
