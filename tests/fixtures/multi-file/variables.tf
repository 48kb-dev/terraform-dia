variable "vpc_cidr" {
  type    = string
  default = "10.0.0.0/16"
}

variable "db_password" {
  type      = string
  sensitive = true
}

output "db_endpoint" {
  value = aws_db_instance.main.endpoint
}
