resource "aws_autoscaling_group" "main" {
  name_prefix         = "${var.project_name}-ecs-as-"
  vpc_zone_identifier = var.ecs_subnets
  min_size            = var.ecs_as_min_size
  max_size            = var.ecs_as_max_size
  desired_capacity    = var.ecs_as_desired_capacity

  launch_template {
    id      = aws_launch_template.main.id
    version = "$Latest"
  }

  health_check_type         = "EC2"
  health_check_grace_period = 300
  force_delete              = true

  tag {
    key                 = "AmazonECSManaged"
    value               = true
    propagate_at_launch = true
  }
  tag {
    key                 = "Name"
    value               = "${var.project_name}-ecs-asg"
    propagate_at_launch = true
  }

  dynamic "tag" {
    for_each = data.aws_default_tags.provider.tags
    content {
      key                 = tag.key
      value               = tag.value
      propagate_at_launch = true
    }
  }
}

data "aws_default_tags" "provider" {}
