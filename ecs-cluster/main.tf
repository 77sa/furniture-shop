resource "aws_ecs_cluster" "comments" {
  name = "${local.app}-cluster"
}

resource "aws_iam_role" "ecs" {
  name = "${local.app}-execution-role"
  assume_role_policy = file("role-policy.json")
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution" {
  role       = aws_iam_role.ecs.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

# NAT Gateway for services in private subnets:
resource "aws_eip" "lb" {
  vpc      = true
}

resource "aws_nat_gateway" "natgw" {
  allocation_id = aws_eip.lb.id
  subnet_id     = data.aws_subnets.default.ids[0]

  tags = {
    Name = "NATGW"
  }
}

resource "aws_route_table" "private" {
  vpc_id = data.aws_vpc.default.id

  route {
    cidr_block     = "0.0.0.0/0"
    nat_gateway_id = aws_nat_gateway.natgw.id
  }

  tags = {
    Name = "PRIVATE RT"
  }
}

resource "aws_route_table_association" "private" {
  subnet_id      = data.aws_subnets.default.ids[2]
  route_table_id = aws_route_table.private.id
}
