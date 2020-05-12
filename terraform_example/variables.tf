variable "aws_region" {
    default = "us-west-2"
    description = "aws region to deploy to"
}

variable "aws_profile" {}

variable "availability_zone" {
    default = "us-west-1b"
}

variable "ecs_task_execution_role_name" {
  description = "ECS task execution role name"
  default = "cratesioopsbotEcsTaskExecutionRole"
}

variable "ecs_auto_scale_role_name" {
  description = "ECS auto scale role Name"
  default = "myEcsAutoScaleRole"
}

variable "app_image" {
    description = "Docker image to run in the ECS cluster"
    default = "https://hub.docker.com/repository/docker/nellshamrell/crates-io-ops-bot"
}

variable "app_port" {
    description = "Port exposed by the docker image to redirect traffic to"
    default = 8888
}

variable "app_count" {
  description = "Number of docker containers to run"
  default     = 3
}

variable "health_check_path" {
  default = "/"
}

variable "fargate_cpu" {
   description = "Fargate instance CPU units to provision (1 vCPU = 1024 CPU units)" 
   default = "256"
}


variable "fargate_memory" {
  description = "Fargate instance memory to provision (in MiB)"
  default     = "512"
}