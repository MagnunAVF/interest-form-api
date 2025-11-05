output "lambda_execution_role_arn" {
  description = "ARN of the IAM role for Lambda execution"
  value       = aws_iam_role.lambda_execution_role.arn
}

output "dynamodb_table_arn" {
  description = "ARN of the DynamoDB Interests table"
  value       = aws_dynamodb_table.interests.arn
}
