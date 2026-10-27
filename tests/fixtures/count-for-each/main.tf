variable "azs" {
  default = ["eu-central-1a", "eu-central-1b"]
}

variable "buckets" {
  default = {
    logs    = "example-logs"
    backups = "example-backups"
  }
}

resource "aws_vpc" "main" {
  cidr_block = "10.0.0.0/16"
}

resource "aws_subnet" "public" {
  count             = length(var.azs)
  vpc_id            = aws_vpc.main.id
  availability_zone = var.azs[count.index]
  cidr_block        = cidrsubnet(aws_vpc.main.cidr_block, 8, count.index)
}

resource "aws_s3_bucket" "this" {
  for_each = var.buckets
  bucket   = each.value
}

resource "aws_instance" "web" {
  ami           = "ami-12345678"
  instance_type = "t3.micro"
  subnet_id     = aws_subnet.public[0].id

  depends_on = [aws_s3_bucket.this]
}
