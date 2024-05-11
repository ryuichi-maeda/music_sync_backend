resource "aws_ecs_service" "main" {
  name            = "${var.project_name}-ecs-service"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.main.arn

  desired_count = var.ecs_task_desired_count

  network_configuration {
    subnets         = var.ecs_subnets
    security_groups = var.ecs_sg_ids
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.main.arn
    container_name   = var.ecs_container_name
    container_port   = var.ecs_container_port
  }

  lifecycle {
    ignore_changes = [desired_count]
  }

  depends_on = [aws_ecs_cluster.main]
}
