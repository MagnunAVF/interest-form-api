# Interest Form API

A Rust-based AWS Lambda function that handles interest form submissions and stores them in DynamoDB.

## Overview

This API provides a single endpoint `POST /interests` that accepts user interest form data (name and email) and stores it in a DynamoDB table. The function is designed to run on AWS Lambda with support for local development using DynamoDB Local.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)
- [Docker](https://www.docker.com/) (for local DynamoDB)
- [AWS CLI](https://aws.amazon.com/cli/) (for deployment)

## Project Structure

```
src/
├── main.rs          # Entry point and Lambda runtime setup
├── http_handler.rs  # HTTP request handler and validation
├── db.rs           # DynamoDB client and operations
├── models.rs       # Data models (Interest, InterestFormData)
└── aws.rs          # AWS SDK configuration
```

## Environment Variables

The following environment variables are required:

- `ENV` - Environment name (`dev` for local, `prod` for production)
- `INTERESTS_TABLE_NAME` - DynamoDB table name for storing interests
- `CUSTOM_AWS_REGION` - AWS region (optional, defaults to SDK config)
- `RUST_LOG` - Logging level (optional, e.g., `debug`, `info`)
- `RUST_BACKTRACE` - Enable backtraces (optional, `1` or `full`)

## API Specification

### POST /interests

Accepts interest form submissions.

**Request Body:**

```json
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Response:**

- `201 Created` - Interest successfully recorded
- `400 Bad Request` - Invalid request body or missing fields
- `405 Method Not Allowed` - Invalid HTTP method or path
- `500 Internal Server Error` - Database or server error

## Local Development

### 1. Start Local DynamoDB

Use the provided Makefile command to start DynamoDB Local and create the required table:

```bash
make start-localhost-db
```

This will:

- Start a DynamoDB Local container on port 8000
- Create the interests table with the proper schema

### 2. Run the Lambda Function Locally

Start the local Lambda server with hot-reload:

```bash
make run
```

Or manually:

```bash
cargo lambda watch
```

The function will be available at `http://localhost:9000`.

### 3. Test the Function

**Using cURL:**

```bash
curl -X POST http://localhost:9000/interests \
  -H "Content-Type: application/json" \
  -d '{"name": "Jane Doe", "email": "jane@example.com"}'
```

**Using cargo lambda invoke:**

```bash
cargo lambda invoke --data-file tests/data-mock.json
```

### 4. View Database Items

Check stored interests in the local DynamoDB:

```bash
./scripts/show-db-items.sh
```

### 5. Stop Local DynamoDB

```bash
make stop-localhost-db
```

## Building

Build for production (x86_64):

```bash
make build
```

Build for ARM64:

```bash
make build-arm
```

Or use cargo lambda directly:

```bash
cargo lambda build --release
```

## Testing

Run unit tests:

```bash
make test
```

Or:

```bash
cargo test
```

## Deployment

### Prerequisites

- AWS credentials configured
- IAM role with DynamoDB and CloudWatch permissions
- Environment files (`.env.hml`, `.env.prod`) with required variables
- Terraform infrastructure provisioned (see [terraform/README.md](terraform/README.md))

### Deploy to Homologation

```bash
make deploy-hml
```

### Deploy to Production

```bash
make deploy-prod
```

### Test Remote Function

```bash
cargo lambda invoke -R arn:aws:lambda:us-east-1:ACCOUNT_ID:function:prod-interest-form-api \
    --data-file tests/data-mock.json \
    --region us-east-1
```

### Destroy Deployment

Remove the Lambda function from AWS:

```bash
make destroy-hml  # Remove homologation
make destroy-prod # Remove production
```
