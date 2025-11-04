#!/bin/bash

echo -e "\n* Stopping Localhost Docker Compose services..."

docker-compose down

docker ps

echo -e "\n* Localhost Docker Compose services stopped."