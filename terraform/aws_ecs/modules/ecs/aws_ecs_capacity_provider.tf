resource "aws_ecs_capacity_provider" "main" {
  name = "${var.project_name}-ecs-capacity-provider"

  auto_scaling_group_provider {
    auto_scaling_group_arn = aws_autoscaling_group.main.arn
    managed_scaling {
      maximum_scaling_step_size = var.maximum_scaling_step_size
      minimum_scaling_step_size = var.minimum_scaling_step_size
      status                    = "ENABLED"
      # This is the target value of ((number of instances needed) / (number of running instances) * 100)
      target_capacity = var.scaling_target_capacity
    }
  }

  tags = {
    Name = "${var.project_name}-ecs-capacity-provider"
  }
}

resource "aws_ecs_cluster_capacity_providers" "main" {
  cluster_name       = aws_ecs_cluster.main.name
  capacity_providers = [aws_ecs_capacity_provider.main.name]

  # TODO: Strategy will be changed to use the CPU and memory requirements of the tasks
  default_capacity_provider_strategy {
    capacity_provider = aws_ecs_capacity_provider.main.name
    base              = 1
    weight            = 100
  }
}
