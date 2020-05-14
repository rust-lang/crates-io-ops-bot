[
  {
    "name": "crates-io-ops-bot",
    "image": "${app_image}",
    "cpu": ${fargate_cpu},
    "memory": ${fargate_memory},
    "networkMode": "awsvpc",
    "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/crates-io-ops-bot",
          "awslogs-region": "${aws_region}",
          "awslogs-stream-prefix": "ecs"
        }
    },
    "secrets": [
      {
        "name": "DISCORD_TOKEN",
        "valueFrom": "${discord_token}"
      },
      {
        "name": "AUTHORIZED_USERS",
        "valueFrom": "${authorized_users}"
      },
      {
        "name": "HEROKU_API_KEY",
        "valueFrom": "${heroku_api_key}"
      },
      {
        "name": "BUILD_CHECK_INTERVAL",
        "valueFrom": "${build_check_interval}"
      },
      {
        "name": "GITHUB_ORG",
        "valueFrom": "${github_org}"
      },
      {
        "name": "GITHUB_REPO",
        "valueFrom": "${github_repo}"
      },
      {
        "name": "GITHUB_TOKEN",
        "valueFrom": "${github_token}"
      }
    ],
    "portMappings": [
      {
        "containerPort": ${app_port},
        "hostPort": ${app_port}
      }
    ]
  }
]