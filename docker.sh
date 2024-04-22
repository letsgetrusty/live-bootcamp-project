#!/bin/bash

# Define the location of the .env file (change if needed)
ENV_FILE="./auth-service/.env"

# Check if the .env file exists
if ! [[ -f "$ENV_FILE" ]]; then
  echo "Error: .env file not found!"
  exit 1
fi

# Read each line in the .env file (ignoring comments)
while IFS= read -r line; do
  # Skip blank lines and lines starting with #
  if [[ -n "$line" ]] && [[ "$line" != \#* ]]; then
    # Split the line into key and value
    key=$(echo "$line" | cut -d '=' -f1)
    value=$(echo "$line" | cut -d '=' -f2-)
    # Export the variable
    export "$key=$value"
  fi
done < <(grep -v '^#' "$ENV_FILE")

# Run docker-compose commands with exported variables
docker-compose build
docker-compose up