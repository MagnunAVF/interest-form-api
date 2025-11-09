#!/bin/bash

N=${1:-10}

for i in $(seq 1 $N); do
    echo "Creating interest $i"
    curl -X POST http://localhost:9000/interests -H "Content-Type: application/json" -d '{"name": "Test User '$i'", "email": "testuser'$i'@example.com"}'
done
