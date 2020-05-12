provider "aws" {
    region = var.aws_region
    profile = var.aws_profile
}

resource "aws_ecs_service" "crates-io-ops-bot" {
    name = "crates-io-ops-bot2"
    task_definition = aws_ecs_task_definition.crates-io-ops-bot.arn
    launch_type = "FARGATE"
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
    container_definitions = data.template_file.crates-io-ops-bot.rendered 
    cpu = 256
    memory = 512
    requires_compatibilities = ["FARGATE"]
    network_mode = "awsvpc"
}