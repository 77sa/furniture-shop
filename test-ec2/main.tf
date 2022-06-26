resource "tls_private_key" "ssh_key" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

resource "aws_key_pair" "deployer" {
  key_name   = "vpc-with-instance-key"
  public_key = tls_private_key.ssh_key.public_key_openssh
}

resource "local_file" "private_key" {
    content  = tls_private_key.ssh_key.private_key_pem
    filename = "private_key.pem"
}

resource "null_resource" "chmod" {
  depends_on = [
    local_file.private_key
  ]

  provisioner "local-exec" {
    command = "chmod 400 private_key.pem"
  }
}

resource "aws_security_group" "public" {
  name        = "public-sg"
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

resource "aws_instance" "test" {
  ami                    = "ami-0d729d2846a86a9e7"
  instance_type          = "t2.micro"
  vpc_security_group_ids = ["${aws_security_group.public.id}"]
  subnet_id              = data.aws_subnets.default.ids[0]
  key_name               = aws_key_pair.deployer.key_name

  tags = {
    Name = "service-test"
  }
}

output "public_ip" {
  value = aws_instance.test.public_ip
}
