#!/bin/bash

# This script prevents interference by checking if a container of a service
# is still running on the host machine before the devcontainer starts.

# If this does not correctly identify the container name, check if container_name
# is set on the docker compose service.

# Handle cases when container name is not passed in as an argument.
if [ -z "$1" ]; then
  echo "Error: No container name provided."
  echo "Usage in initializeCommand: $0 <container_name>"
  exit 1
fi

CONTAINER_NAME_TO_CHECK=$1

echo "Checking if container '$CONTAINER_NAME_TO_CHECK' is running..."
if [ -n "$(docker ps -q -f name=^/"${CONTAINER_NAME_TO_CHECK}"$)" ]; then
  echo "Error: Service ('$CONTAINER_NAME_TO_CHECK') is already running."
  echo "Run 'docker compose down' before starting dev container."
  exit 1
fi
