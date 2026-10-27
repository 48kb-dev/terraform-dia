resource "aws_db_subnet_group" "main" {
  name       = "main"
  subnet_ids = [aws_subnet.private.id, aws_subnet.private_b.id]
}

resource "aws_db_instance" "main" {
  identifier           = "app-db"
  engine               = "postgres"
  instance_class       = "db.t3.micro"
  allocated_storage    = 20
  username             = "app"
  password             = var.db_password
  db_subnet_group_name = aws_db_subnet_group.main.name
  skip_final_snapshot  = true
}
