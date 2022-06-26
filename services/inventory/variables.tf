locals {
  app     = "furniture-shop"
  service = "inventory"
}

variable "envvars" {
  type        = map(string)
  description = "container environment variables"
}

data "aws_iam_role" "ecs" {
  name = "${local.app}-execution-role"
}

data "aws_ecs_cluster" "ecs" {
  cluster_name = "${local.app}-cluster"
}

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

data "aws_caller_identity" "current" {}
