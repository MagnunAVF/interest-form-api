resource "aws_dynamodb_table" "interests" {
  name           = "${var.project_name}-Interests"
  billing_mode   = "PROVISIONED"
  read_capacity  = 5
  write_capacity = 5
  hash_key       = "id"
  range_key      = "email"

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "email"
    type = "S"
  }

  tags = {
    Name        = "${var.project_name}-Interests"
    Environment = var.project_name
  }
}