module "network" {
  source = "./modules/network"
  cidr   = "10.0.0.0/16"
}

resource "aws_instance" "app" {
  ami           = "ami-12345678"
  instance_type = "t3.micro"
  subnet_id     = module.network.subnet_id
}
