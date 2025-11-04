#!/bin/bash

echo -e "\n* Listing DynamoDB items..."

aws dynamodb scan \
    --table-name dev-Interests \
    --endpoint-url http://localhost:8000 \
    --region us-east-1 \
    --query 'Items[*].{id: id, name: name, email: email}'

echo -e "\n* DynamoDB items listed."
