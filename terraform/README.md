# AWS Deployment with Terraform

## Prerequisites

- Terraform >= 0.12
- AWS CLI configured with appropriate credentials
- An AWS account with permissions to create the resources

## Setup

Create a `terraform.tfvars` and `backend.tfvars` file in the environment directory you want to use (e.g., `environment/hml/terraform.tfvars` and `environment/hml/backend.tfvars`).

Template: `environment/ENV/terraform.tfvars`

```hcl
project_name = "MY_PROJECT_NAME"
region = "MY_REGION"
lambda_function_arn = "arn:aws:lambda:REGION:ACCOUNT_ID:function:FUNCTION_NAME"
lambda_function_name = "FUNCTION_NAME"
```

Template: `environment/ENV/backend.tfvars`

```hcl
bucket = "MY_BUCKET_NAME"
key    = "MY_KEY"
region = "MY_REGION"
```

## Usage

1. Initialize the Terraform configuration:

   ```shell
   terraform init -backend-config=environment/ENV/backend.tfvars
   ```

2. Set the required variables in a `terraform.tfvars` file or use the provided environment files:

   ```shell
   terraform plan -var-file=environment/ENV/terraform.tfvars
   ```

3. Apply the configuration:
   ```shell
   terraform apply -var-file=environment/ENV/terraform.tfvars
   ```
