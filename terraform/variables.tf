variable "project_name" {
  description = "Name of the project that will be used as a prefix for all resources created in this infrastructure."
  type        = string
}

variable "region" {
  description = "AWS region where resources will be deployed (e.g., us-east-1, eu-west-1)."
  type        = string
}

variable "lambda_function_arn" {
  description = "ARN of the Lambda function to integrate with API Gateway."
  type        = string
}

variable "lambda_function_name" {
  description = "Name of the Lambda function to integrate with API Gateway."
  type        = string
}