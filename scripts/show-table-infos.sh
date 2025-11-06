#!/bin/bash

echo -e "\n* Showing DynamoDB table infos..."

TABLE_INFO=$(aws dynamodb describe-table \
    --table-name dev-Interests \
    --endpoint-url http://localhost:8000 \
    --region us-east-1)

ITEM_COUNT=$(echo "$TABLE_INFO" | jq -r '.Table.ItemCount')
TABLE_SIZE_BYTES=$(echo "$TABLE_INFO" | jq -r '.Table.TableSizeBytes')
TABLE_SIZE_KB=$(echo "scale=2; $TABLE_SIZE_BYTES / 1024" | bc)

echo -e "\n=== Table Structure ==="
echo "Key Schema:"
echo "$TABLE_INFO" | jq -r '.Table.KeySchema[] | "  - \(.AttributeName) (\(.KeyType))"'

echo -e "\nAttribute Definitions:"
echo "$TABLE_INFO" | jq -r '.Table.AttributeDefinitions[] | "  - \(.AttributeName): \(.AttributeType)"'

echo -e "\n=== Table Statistics ==="
echo "Total Items: $ITEM_COUNT"
echo "Table Size: ${TABLE_SIZE_KB} KB (${TABLE_SIZE_BYTES} bytes)"

echo -e "\n* DynamoDB table infos shown."
