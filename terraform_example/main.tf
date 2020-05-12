provider "aws" {
    region = var.aws_region
    profile = var.aws_profile
}

resource "aws_vpc" "app-vpc" {
  cidr_block = "10.0.0.0/16"
}

resource "aws_subnet" "public" {
  vpc_id     = aws_vpc.app-vpc.id
  cidr_block = "10.0.1.0/24"
}

resource "aws_subnet" "private" {
  vpc_id     = aws_vpc.app-vpc.id
  cidr_block = "10.0.2.0/24"
}

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.app-vpc.id
}

resource "aws_route_table" "private" {
  vpc_id = aws_vpc.app-vpc.id
}

resource "aws_route_table_association" "public_subnet" {
  subnet_id      = aws_subnet.public.id
  route_table_id = aws_route_table.public.id
}

resource "aws_route_table_association" "private_subnet" {
  subnet_id      = aws_subnet.private.id
  route_table_id = aws_route_table.private.id
}

resource "aws_eip" "nat" {
  vpc = true
}

resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.app-vpc.id
}

resource "aws_nat_gateway" "ngw" {
  subnet_id     = aws_subnet.public.id
  allocation_id = aws_eip.nat.id

  depends_on = [
    aws_internet_gateway.igw
  ]
}

resource "aws_route" "public_igw" {
  route_table_id         = aws_route_table.public.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.igw.id
}

resource "aws_route" "private_ngw" {
  route_table_id         = aws_route_table.private.id
  destination_cidr_block = "0.0.0.0/0"
  nat_gateway_id         = aws_nat_gateway.ngw.id
}

resource "aws_security_group" "http" {
  name        = "http"
  description = "HTTP traffic"
  vpc_id      = aws_vpc.app-vpc.id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "https" {
  name        = "https"
  description = "HTTPS traffic"
  vpc_id      = aws_vpc.app-vpc.id

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "egress-all" {
  name        = "egress_all"
  description = "Allow all outbound traffic"
  vpc_id      = aws_vpc.app-vpc.id

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "api-ingress" {
  name        = "api_ingress"
  description = "Allow ingress to API"
  vpc_id      = aws_vpc.app-vpc.id

  ingress {
    from_port   = 3000
    to_port     = 3000
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_lb_target_group" "crates-io-ops-bot" {
  name = "crates-io-ops-bot"
  port = 3000
  protocol = "HTTP"
  target_type = "ip"
  vpc_id = aws_vpc.app-vpc.id

  health_check {
    enabled = true
    path = "/health"
  }

  depends_on = [
    aws_alb.crates-io-ops-bot
  ]
}

resource "aws_alb" "crates-io-ops-bot" {
  name = "crates-io-ops-bot-lb"
  internal = false
  load_balancer_type = "application"

  subnets = [
    aws_subnet.public.id,
    aws_subnet.private.id,
  ]

  security_groups = [
    aws_security_group.http.id,
    aws_security_group.https.id,
    aws_security_group.egress-all.id,
  ]

  depends_on = [aws_internet_gateway.igw]
}

resource "aws_alb_listener" "crates-io-ops-bot-http" {
  load_balancer_arn = aws_alb.crates-io-ops-bot.arn
  port = "80"
  protocol = "HTTP"

  default_action {
    type = "forward"
    target_group_arn = aws_lb_target_group.crates-io-ops-bot.arn
  }
}

resource "aws_iam_role" "crate-io-ops-bot-task-execution-role" {
  name = "crates-io-ops-bot-task-execution-role"

  assume_role_policy = data.aws_iam_policy_document.ecs-task-assume-role.json
}

data "aws_iam_policy_document" "ecs-task-assume-role" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type = "Service"
      identifiers = ["ecs-tasks.amazonaws.com"]
    }
  }
}

data "aws_iam_policy" "ecs-task-execution-role" {
  arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

# Attach the above policy to the execution role.
resource "aws_iam_role_policy_attachment" "ecs-task-execution-role" {
  role = aws_iam_role.crate-io-ops-bot-task-execution-role.name
  policy_arn = data.aws_iam_policy.ecs-task-execution-role.arn
}

resource "aws_ecs_cluster" "crates-io-ops-bot" {
    name = "crates-io-ops-bot"
}

resource "aws_ecs_service" "crates-io-ops-bot" {
    name = "crates-io-ops-bot"
    task_definition = aws_ecs_task_definition.crates-io-ops-bot.arn
    cluster = aws_ecs_cluster.crates-io-ops-bot.id
    launch_type = "FARGATE"

    network_configuration {
        assign_public_ip = false

        security_groups = [
            aws_security_group.egress-all.id,
            aws_security_group.api-ingress.id,
        ]

        subnets = [
            aws_subnet.private.id
        ]
    }

    load_balancer {
        target_group_arn = aws_lb_target_group.crates-io-ops-bot.arn
        container_name = "crates-io-ops-bot"
        container_port = var.app_port
    }

    desired_count = var.app_count
}

resource "aws_cloudwatch_log_group" "crates-io-ops-bot" {
    name = "/ecs/crates-io-ops-bot"
}

data "template_file" "crates-io-ops-bot" {
  template = file("./templates/crates-io-ops-bot.json.tpl")

  vars = {
    app_image      = var.app_image
    app_port       = var.app_port
    fargate_cpu    = var.fargate_cpu
    fargate_memory = var.fargate_memory
    aws_region     = var.aws_region
  }
}

resource "aws_ecs_task_definition" "crates-io-ops-bot" {
    family = "crates-io-ops-bot"
    execution_role_arn = aws_iam_role.crate-io-ops-bot-task-execution-role.arn
    container_definitions = data.template_file.crates-io-ops-bot.rendered 
    cpu = 256
    memory = 512
    requires_compatibilities = ["FARGATE"]
    network_mode = "awsvpc"
}

output "vpc_id" {
  value = aws_vpc.app-vpc.id
}

output "public_subnet_id" {
  value = aws_subnet.public.id
}

output "private_subnet_id" {
  value = aws_subnet.private.id
}

output "alb_url" {
  value = "http://${aws_alb.crates-io-ops-bot.dns_name}"
}
