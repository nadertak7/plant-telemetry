#include <time.h>

#include <ArduinoJson.h>
#include <ESP8266WiFi.h>
#include <PubSubClient.h>

#include "wifi_secrets.h"

WiFiClient g_wifi_client;
PubSubClient g_client(g_wifi_client);

// Sensor settings
const int kMoistureSensorPin = A0;
const char* kMqttClientId = "ESP8266_Sensor_01";
const char* kMqttTopic = "plant-monitoring/living-room/scarlet-star-1/telemetry";
const int kAdcValueDry = 666;
const int kAdcValueWet = 272;

// Wifi Secrets from wifi_secrets.h
const char* kWifiSsid = WIFI_SSID;
const char* kWifiPassword = WIFI_PASS;

// MQTT settings
const char* kMqttBrokerAddress = MQTT_BROKER_IP;
const char* kMqttUsername = MQTT_USERNAME;
const char* kMqttPassword = MQTT_PASSWORD;

// Retry settings
const int kMqttMaxRetries = 5;
const int kWifiMaxRetries = 5;
const int kTimeSyncMaxRetries = 5;

// Time settings
const int kSleepDurationSuccessSecs = 300;
const int kSleepDurationErrorSecs = 10;
const int kWifiRetryDelayMs = 10000;
const int kMqttRetryDelayMs = 10000;
const int kTimeSyncRetryDelayMs = 500;
// NTP specific time settings
const char* kNtpServer = "pool.ntp.org";
const char* kNtpTimezone = "UTC0";
const long kMinValidTimeUnix = 1735689600L; // 1st January 2025

void LogRetryAttempt(int attempt_index, const int max_retry, int error_code = -999) {
  Serial.printf("Failed attempt %d of %d...\n", attempt_index + 1, max_retry);
  if (error_code != -999) {
    Serial.printf("Error code %d\n", error_code);
  }
}

bool ConnectWifi() {
  Serial.println("\nConnecting to Wifi network...");
  WiFi.begin(kWifiSsid, kWifiPassword);
  for (int i = 0; i < kWifiMaxRetries; i++) {
    delay(kWifiRetryDelayMs);
    if (WiFi.status() == WL_CONNECTED) {
      Serial.println("WiFi connected...");
      return true;
    }
    LogRetryAttempt(i, kWifiMaxRetries, WiFi.status());
  }
  Serial.println("\nError: Failed to connect to wifi.");
  return false;
}

bool ConnectMqtt() {
  Serial.println("Connecting to MQTT...");
  g_client.setServer(kMqttBrokerAddress, 1883);
  for (int i = 0; i < kMqttMaxRetries; i++) {
    if (g_client.connect(kMqttClientId, kMqttUsername, kMqttPassword)) {
      Serial.println("Connected to MQTT....");
      return true;
    }
    LogRetryAttempt(i, kMqttMaxRetries, g_client.state());
    delay(kMqttRetryDelayMs);
  }
  Serial.println("Error: Failed to connect to MQTT.");
  return false;
}

bool SyncTime() {
  Serial.println("Syncing time from NTP server...");
  configTime(kNtpTimezone, kNtpServer);
  for (int i = 0; i < kTimeSyncMaxRetries; i++) {
    if (time(nullptr) > kMinValidTimeUnix) { // Later than 2025 (suggests successful sync)
      Serial.println("Time synced...");
      return true;
    }
    LogRetryAttempt(i, kTimeSyncMaxRetries);
    delay(kTimeSyncRetryDelayMs);
  }
  Serial.println("Error: Failed to sync time from NTP server.");
  return false;
}

String GetFormattedTimestamp() {
  char time_str[30];
  time_t now = time(nullptr);
  strftime(time_str, sizeof(time_str), "%Y-%m-%dT%H:%M:%SZ", gmtime(&now));
  return String(time_str);
}

String GetJsonPayload(int adc_value_reading) {
  int moisture_percentage = map(adc_value_reading, kAdcValueDry, kAdcValueWet, 0, 100);
  // In case moisture percentage falls outside of 0-100 range
  moisture_percentage = constrain(moisture_percentage, 0, 100);

  // Create json document
  StaticJsonDocument<256> json_doc;
  json_doc["timestamp"] = GetFormattedTimestamp();
  json_doc["adc_value"] = adc_value_reading;
  json_doc["dry_value"] = kAdcValueDry;
  json_doc["wet_value"] = kAdcValueWet;
  json_doc["moisture_perc"] = moisture_percentage;
  String json_payload;
  serializeJson(json_doc, json_payload);
  return json_payload;
}

void setup() {
  bool is_task_successful = false; // Determines how long ESP should sleep for
  Serial.begin(115200);
  while (!Serial) {} // Wait for serial to initialise

  // Take sensor reading
  int adc_value_reading = analogRead(kMoistureSensorPin);

  // If the sensor reading is significantly lower than the wet value,
  // it is likely that the sensor is not connected. A message should
  // not be sent to the MQTT broker
  if (adc_value_reading < kAdcValueWet - 50) {
    Serial.printf("\nInvalid sensor reading: %d\n", adc_value_reading);
    Serial.println("Sensor not likely connected. Sleeping indefinitely...");
    ESP.deepSleep(0); // Infinite
  }
  String payload = GetJsonPayload(adc_value_reading);

  if (ConnectWifi() && ConnectMqtt() && SyncTime()) {
    g_client.publish(kMqttTopic, payload.c_str(), true);
    Serial.println("Published message to MQTT broker...");
    is_task_successful = true;
  }

  // Prepare for deep sleep
  Serial.println("Sleeping...");
  g_client.disconnect();
  WiFi.disconnect();
  delay(100); // Give MQTT time to send before sleeping

  // Variably configure sleep time based on message being published successfully
  int sleepDurationSecs = is_task_successful ? kSleepDurationSuccessSecs : kSleepDurationErrorSecs;
  ESP.deepSleep(sleepDurationSecs * 1000000); // Time in microseconds
}

void loop() {
  // No need for loop as ESP resets after deep sleep
}
