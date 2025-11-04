#!/bin/bash

echo -e "\n* Creating DynamoDB tables..."
aws dynamodb create-table \
    --table-name dev-Interests \
    --endpoint-url http://localhost:8000 \
    --attribute-definitions \
        AttributeName=id,AttributeType=S \
        AttributeName=email,AttributeType=S \
    --key-schema \
        AttributeName=id,KeyType=HASH \
        AttributeName=email,KeyType=RANGE \
    --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5 \
    --no-cli-pager --output json

echo -e "\n* Listing DynamoDB tables..."
aws dynamodb list-tables --endpoint-url http://localhost:8000 \
    --no-cli-pager --output json
