#include <ESP8266WiFi.h>
#include <PubSubClient.h>
#include "wifi_secrets.h"
#include <time.h>

WiFiClient wifiClient;
PubSubClient client(wifiClient);

// Sensor settings
const int MOISTURE_SENSOR_PIN = A0;
const char* MQTT_CLIENT_ID_VALUE = "ESP8266_Sensor_01";
const char* MQTT_PUBLISH_TOPIC = "plant-monitoring/living-room/scarlet-star-1/telemetry";
const int DRY_VALUE = 666;
const int WET_VALUE = 272;

// Wifi Secrets from wifi_secrets.h
const char* WIFI_SSID_VALUE = WIFI_SSID;
const char* WIFI_PASS_VALUE = WIFI_PASS;

// MQTT settings
const char* MQTT_BROKER_IP_VALUE = MQTT_BROKER_IP;
const char* MQTT_USERNAME_VALUE = MQTT_USERNAME;
const char* MQTT_PASSWORD_VALUE = MQTT_PASSWORD;

// Retry settings
const int MQTT_MAX_RETRIES = 5;
const int WIFI_MAX_RETRIES = 5;
const int TIME_SYNC_MAX_RETRIES = 5;

// Time settings
const int WIFI_RETRY_DELAY_MS = 1000;
const int MQTT_RETRY_DELAY_MS = 500;
const int TIME_SYNC_RETRY_DELAY_MS = 500;
// NTP specific time settings
const char* NTP_SERVER = "pool.ntp.org";
const char* TIMEZONE = "UTC0";
const long MIN_VALID_TIME_SYNC = 1735689600L; // 1st January 2025
const int SLEEP_DURATION_ERROR_SECS = 10;
const int SLEEP_DURATION_SUCCESS_SECS = 60;

void log_retry_attempt(int iterator, const int max_retry) {
  Serial.printf("Failed attempt %d of %d...\n", iterator + 1, max_retry);
}

bool connect_wifi() {
  Serial.print("\nConnecting to Wifi network...");
  WiFi.begin(WIFI_SSID_VALUE, WIFI_PASS_VALUE);
  for (int i = 0; i < WIFI_MAX_RETRIES; i++) {
    if (WiFi.status() == WL_CONNECTED) {
      Serial.println("\nWiFi connected...");
      return true;
    }
    else {
      log_retry_attempt(i, WIFI_MAX_RETRIES);
      delay(WIFI_RETRY_DELAY_MS);
    }
  }
  Serial.println("\nError: Failed to connect to wifi.");
  return false;
}

bool connect_mqtt() {
  Serial.print("Connecting to MQTT...");
  client.setServer(MQTT_BROKER_IP_VALUE, 1883);
  for (int i = 0; i < MQTT_MAX_RETRIES; i++) {
    if (client.connect(MQTT_CLIENT_ID_VALUE, MQTT_USERNAME_VALUE, MQTT_PASSWORD_VALUE)) {
      Serial.println("\nConnected to MQTT....");
      return true;
    }
    else {
      log_retry_attempt(i, MQTT_MAX_RETRIES);
      delay(MQTT_RETRY_DELAY_MS);
    }
  }
  Serial.println("\nError: Failed to connect to MQTT.");
  return false;
}

bool sync_time() {
  Serial.print("Syncing time from NTP server...");
  configTime(TIMEZONE, NTP_SERVER);
  for (int i = 0; i < TIME_SYNC_MAX_RETRIES; i++) {
    if (time(nullptr) > MIN_VALID_TIME_SYNC) { // Later than 2025 (suggests successful sync)
      Serial.println("\nTime synced...");
      return true;
    }
    else {
      log_retry_attempt(i, TIME_SYNC_MAX_RETRIES);
      delay(TIME_SYNC_RETRY_DELAY_MS);
    }
  }
  Serial.println("\nError: Failed to sync time from NTP server.");
  return false;
}

String get_formatted_timestamp() {
  char timeStr[30];
  time_t now = time(nullptr);
  strftime(timeStr, sizeof(timeStr), "%Y-%m-%dT%H:%M:%SZ", gmtime(&now));
  return String(timeStr);
}

String get_moisture_reading() {
  // Get timestamp from function
  String timestamp = get_formatted_timestamp();

  // Take sensor reading
  int rawMoistureValue = analogRead(MOISTURE_SENSOR_PIN);
  int moisturePercentage = map(rawMoistureValue, DRY_VALUE, WET_VALUE, 0, 100);
  // In case moisture percentage falls outside of 0-100 range
  moisturePercentage = constrain(moisturePercentage, 0, 100);

  // Compile into json
  String moistureData = "{";
  moistureData += "\"timestamp\":\"" + timestamp + "\",";
  moistureData += "\"raw\":" + String(rawMoistureValue) + ",";
  moistureData += "\"percentage\":" + String(moisturePercentage);
  moistureData += "}";
  return moistureData;
}

void setup() {
  bool success = false; // Determines how long ESP should sleep for
  Serial.begin(115200);
  while (!Serial) {} // Wait for serial to initialise

  if (connect_wifi() && connect_mqtt() && sync_time()) {
    String payload = get_moisture_reading();
    client.publish(MQTT_PUBLISH_TOPIC, payload.c_str(), true);
    Serial.println("Published message to MQTT broker...");
    success = true;
  }

  // Prepare for deep sleep
  Serial.println("Sleeping...");
  client.disconnect();
  WiFi.disconnect();
  delay(100); // Give MQTT time to send before sleeping

  // Variably configure sleep time based on message being published successfully
  int sleep_duration_secs = success ? SLEEP_DURATION_SUCCESS_SECS : SLEEP_DURATION_ERROR_SECS;
  ESP.deepSleep(sleep_duration_secs * 1000000); // Time in microseconds
}

void loop() {
  // No need for loop as ESP resets after deep sleep
}
