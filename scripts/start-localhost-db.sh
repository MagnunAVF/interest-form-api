#!/bin/bash

echo -e "\n* Starting Localhost Docker Compose services..."

docker-compose up -d

docker ps

echo -e "\n* Localhost Docker Compose services started."