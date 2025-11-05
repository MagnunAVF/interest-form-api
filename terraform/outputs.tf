output "lambda_execution_role_arn" {
  description = "ARN of the IAM role for Lambda execution"
  value       = aws_iam_role.lambda_execution_role.arn
}

output "dynamodb_table_arn" {
  description = "ARN of the DynamoDB Interests table"
  value       = aws_dynamodb_table.interests.arn
}

output "api_gateway_url" {
  description = "URL of the API Gateway endpoint"
  value       = aws_apigatewayv2_api.lambda_api.api_endpoint
}
