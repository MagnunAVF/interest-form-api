variable "project_name" {
  description = "Name of the project that will be used as a prefix for all resources created in this infrastructure."
  type        = string
}

variable "region" {
  description = "AWS region where resources will be deployed (e.g., us-east-1, eu-west-1)."
  type        = string
}