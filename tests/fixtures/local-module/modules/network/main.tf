variable "cidr" {
  type = string
}

resource "aws_vpc" "this" {
  cidr_block = var.cidr
}

resource "aws_subnet" "this" {
  vpc_id     = aws_vpc.this.id
  cidr_block = cidrsubnet(var.cidr, 8, 0)
}

output "subnet_id" {
  value = aws_subnet.this.id
}
