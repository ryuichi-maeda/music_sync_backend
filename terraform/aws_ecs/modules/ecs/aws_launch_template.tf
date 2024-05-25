resource "aws_launch_template" "main" {
  name_prefix   = "${var.project_name}-ecs-launch-template-"
  image_id      = data.aws_ami.ecs_ami.id
  instance_type = var.ecs_ec2_instance_type

  vpc_security_group_ids = var.ecs_sg_ids

  iam_instance_profile {
    name = aws_iam_instance_profile.ecs_ec2_instance_profile.name
  }
  monitoring {
    enabled = true
  }

  tags = {
    Name = "${var.project_name}-ecs-launch-template"
  }

  user_data = base64encode(<<-EOF
    #!/bin/bash
    echo ECS_CLUSTER=${var.project_name}-ecs-cluster >> /etc/ecs/ecs.config
    EOF
  )
}

# Most recent Amazon ECS-optimized Amazon Linux 2 AMI
data "aws_ami" "ecs_ami" {
  most_recent = true

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  filter {
    name   = "owner-alias"
    values = ["amazon"]
  }

  filter {
    name   = "name"
    values = ["amzn2-ami-ecs-hvm-*-x86_64-ebs"]
  }

  owners = ["amazon"]
}

# IAM Role for ECS EC2 Instance
data "aws_iam_policy_document" "ecs_ec2_instance_assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"

    principals {
      type = "Service"
      identifiers = [
        "ec2.amazonaws.com"
      ]
    }
  }
}

resource "aws_iam_role" "ecs_ec2_instance_role" {
  name               = "${var.project_name}-ecs-ec2-instance-role"
  assume_role_policy = data.aws_iam_policy_document.ecs_ec2_instance_assume_role_policy.json
}

resource "aws_iam_role_policy_attachment" "ecs_ec2_instance_role_policy" {
  role       = aws_iam_role.ecs_ec2_instance_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceforEC2Role"
}

resource "aws_iam_instance_profile" "ecs_ec2_instance_profile" {
  name = "${var.project_name}-ecs-ec2-instance-profile"
  role = aws_iam_role.ecs_ec2_instance_role.name
}

# If additional policies are needed, they can be attached here
