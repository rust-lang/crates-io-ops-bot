provider "aws" {
    region = var.aws_region
    profile = var.aws_profile
}

data "aws_availability_zones" "available" {
}

resource "aws_vpc" "main" {
  cidr_block = "172.17.0.0/16"
}

# Create var.az_count private subnets, each in a different AZ
resource "aws_subnet" "private" {
  count             = var.az_count
  cidr_block        = cidrsubnet(aws_vpc.main.cidr_block, 8, count.index)
  availability_zone = data.aws_availability_zones.available.names[count.index]
  vpc_id            = aws_vpc.main.id
}

# Create var.az_count public subnets, each in a different AZ
resource "aws_subnet" "public" {
  count                   = var.az_count
  cidr_block              = cidrsubnet(aws_vpc.main.cidr_block, 8, var.az_count + count.index)
  availability_zone       = data.aws_availability_zones.available.names[count.index]
  vpc_id                  = aws_vpc.main.id
  map_public_ip_on_launch = true
}

# Internet Gateway for the public subnet
resource "aws_internet_gateway" "gw" {
  vpc_id = aws_vpc.main.id
}

# Route the public subnet traffic through the IGW
resource "aws_route" "internet_access" {
  route_table_id         = aws_vpc.main.main_route_table_id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.gw.id
}

# Create a NAT gateway with an Elastic IP for each private subnet to get internet connectivity
resource "aws_eip" "gw" {
  count      = var.az_count
  vpc        = true
  depends_on = [aws_internet_gateway.gw]
}

resource "aws_nat_gateway" "gw" {
  count         = var.az_count
  subnet_id     = element(aws_subnet.public.*.id, count.index)
  allocation_id = element(aws_eip.gw.*.id, count.index)
}

# Create a new route table for the private subnets, make it route non-local traffic through the NAT gateway to the internet
resource "aws_route_table" "private" {
  count  = var.az_count
  vpc_id = aws_vpc.main.id

  route {
    cidr_block     = "0.0.0.0/0"
    nat_gateway_id = element(aws_nat_gateway.gw.*.id, count.index)
  }
}

# Explicitly associate the newly created route tables to the private subnets (so they don't default to the main route table)
resource "aws_route_table_association" "private" {
  count          = var.az_count
  subnet_id      = element(aws_subnet.private.*.id, count.index)
  route_table_id = element(aws_route_table.private.*.id, count.index)
}

resource "aws_security_group" "http" {
  name        = "http"
  description = "HTTP traffic"
  vpc_id      = aws_vpc.main.id

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
  vpc_id      = aws_vpc.main.id

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
  vpc_id      = aws_vpc.main.id

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
  vpc_id      = aws_vpc.main.id

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
  vpc_id = aws_vpc.main.id

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


  subnets         = aws_subnet.public.*.id

  security_groups = [
    aws_security_group.http.id,
    aws_security_group.https.id,
    aws_security_group.egress-all.id,
  ]

  depends_on = [aws_internet_gateway.gw]
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
        assign_public_ip = true

        security_groups = [
            aws_security_group.egress-all.id,
            aws_security_group.api-ingress.id,
        ]

        subnets          = aws_subnet.private.*.id
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
  value = aws_vpc.main.id
}

output "alb_url" {
  value = "http://${aws_alb.crates-io-ops-bot.dns_name}"
}
