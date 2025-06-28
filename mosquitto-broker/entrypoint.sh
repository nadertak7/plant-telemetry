#!/bin/sh

set -e # Exit if command status != 0

PWFILE="/mosquitto/config/pwfile"

if [ -z "$MQTT_USERNAME" ] || [ -z "$MQTT_PASSWORD" ]; then
  echo "Missing MQTT_USERNAME or MQTT_PASSWORD env vars."
  exit 1
fi

if [ ! -f "$PWFILE" ]; then
  echo "Password file not found. Creating one..."
  mosquitto_passwd -b -c "$PWFILE" "$MQTT_USERNAME" "$MQTT_PASSWORD"
else
  echo "Password file exists..."
  echo "Checking for user $MQTT_USERNAME..."
  if grep -q "^$MQTT_USERNAME:" "$PWFILE"; then
    echo "User already exists in pwfile. Skipping update."
  else
    echo "User not found in pwfile. Adding user..."
    mosquitto_passwd -b "$PWFILE" "$MQTT_USERNAME" "$MQTT_PASSWORD"
  fi
fi

# Run default CMD from image
exec "$@"