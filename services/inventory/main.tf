# ECR
resource "aws_ecr_repository" "repo" {
  name = "${local.service}/runner"
}

resource "aws_ecr_lifecycle_policy" "repo-policy" {
  repository = aws_ecr_repository.repo.name
  policy     = file("ecr.json")
}

# Service discovery:
resource "aws_service_discovery_private_dns_namespace" "inventory" {
  name = "inventory"
  vpc  = data.aws_vpc.default.id
}

resource "aws_service_discovery_service" "inventory" {
  name = "inventory"

  dns_config {
    namespace_id = aws_service_discovery_private_dns_namespace.inventory.id
    routing_policy = "MULTIVALUE"
    dns_records {
      ttl  = 10
      type = "A"
    }
  }

  # health_check_custom_config {
  #   resource_path     = "/health"
  #   failure_threshold = 5
  #   type              = "HTTP"
  # }
}

# Task & service:
resource "aws_ecs_task_definition" "inventory" {
  family                   = "${local.service}-task-family"
  network_mode             = "awsvpc"
  execution_role_arn       = data.aws_iam_role.ecs.arn
  cpu                      = 256
  memory                   = 2048
  requires_compatibilities = ["FARGATE"]

  container_definitions = templatefile("./app.json.tpl", {
    aws_ecr_repository = aws_ecr_repository.repo.repository_url
    tag                = "latest"
    region             = "eu-west-2"
    service            = local.service
    envvars            = var.envvars
    port               = 8000
  })

  tags = {
    Application = local.app
    Service     = local.service
  }
}

resource "aws_ecs_service" "inventory" {
  name            = "${local.service}-service"
  cluster         = data.aws_ecs_cluster.ecs.id
  task_definition = aws_ecs_task_definition.inventory.arn
  desired_count   = 1
  launch_type     = "FARGATE"

  network_configuration {
    security_groups = [aws_security_group.ecs_tasks.id]
    subnets         = [data.aws_subnets.default.ids[2]]
  }

  service_registries {
    registry_arn = aws_service_discovery_service.inventory.arn
  }
  tags = {
    Application = "${local.service}"
  }
}

# Security groups
resource "aws_security_group" "ecs_tasks" {
  name        = "${local.app}-${local.service}-sg"
  description = "Allow all"

  ingress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

# Push image:
resource "null_resource" "push" {
  provisioner "local-exec" {
    command     = "./push.sh . ${aws_ecr_repository.repo.repository_url} latest ${data.aws_caller_identity.current.account_id} ${local.service}"
    interpreter = ["bash", "-c"]
  }
}

resource "aws_cloudwatch_log_group" "comments" {
  name = "${local.service}-log-group"

  tags = {
    Application = "${local.service}"
  }
}
