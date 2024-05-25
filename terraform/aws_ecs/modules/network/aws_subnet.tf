# Public Subnet
resource "aws_subnet" "public_subnet1" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = var.public_subnet1_cidr_block
  availability_zone       = var.availability_zone1
  map_public_ip_on_launch = true
  tags = {
    Name = "${var.project_name}-public-subnet1"
  }
}

resource "aws_subnet" "public_subnet2" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = var.public_subnet2_cidr_block
  availability_zone       = var.availability_zone2
  map_public_ip_on_launch = true
  tags = {
    Name = "${var.project_name}-public-subnet2"
  }
}

# Private Subnet
resource "aws_subnet" "private_subnet1" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = var.private_subnet1_cidr_block
  availability_zone = var.availability_zone1
  tags = {
    Name = "${var.project_name}-private-subnet1"
  }
}

resource "aws_subnet" "private_subnet2" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = var.private_subnet2_cidr_block
  availability_zone = var.availability_zone2
  tags = {
    Name = "${var.project_name}-private-subnet2"
  }
}
