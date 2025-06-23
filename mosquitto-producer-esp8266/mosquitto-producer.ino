#include <ESP8266WiFi.h>
#include <PubSubClient.h>
#include "wifi_secrets.h"
#include <time.h>

WiFiClient wifiClient;
PubSubClient client(wifiClient);

// Sensor settings
const int MOISTURE_SENSOR_PIN = A0;
const int DRY_VALUE = 666;
const int WET_VALUE = 272;

// Wifi Secrets from wifi_secrets.h
const char* WIFI_SSID_VALUE = WIFI_SSID;
const char* WIFI_PASS_VALUE = WIFI_PASS;

// MQTT settings
const char* MQTT_BROKER_IP_VALUE = MQTT_BROKER_IP;
const char* MQTT_USERNAME_VALUE = MQTT_USERNAME;
const char* MQTT_PASSWORD_VALUE = MQTT_PASSWORD;
const char* MQTT_CLIENT_ID_VALUE = "ESP8266_Sensor_01";
const char* MQTT_PUBLISH_TOPIC = "livingroom/plant1";

// Time settings
const int WIFI_RETRY_DELAY = 500;
const int MQTT_RETRY_DELAY = 500;
const int SEND_INTERVAL = 500;
unsigned long lastSendTime = 0;
// NTP specific time settings
const char* NTP_SERVER = "pool.ntp.org";
const char* TIMEZONE = "UTC0";

void connect_wifi() {
  Serial.println();
  Serial.print("Connecting to Wifi network ");
  Serial.println(WIFI_SSID_VALUE);

  WiFi.begin(WIFI_SSID_VALUE, WIFI_PASS_VALUE);

  while (WiFi.status() != WL_CONNECTED) {
    delay(WIFI_RETRY_DELAY);
    Serial.print(".");
  }
  Serial.println("WiFi connected...");
}

void connect_mqtt() {
  while (!client.connected()) {
    Serial.print("Attempting MQTT connection...");
    if (client.connect(MQTT_CLIENT_ID_VALUE, MQTT_USERNAME_VALUE, MQTT_PASSWORD_VALUE)) {
      Serial.println("Connected to MQTT Broker...");
    } else {
      Serial.print("MQTT connection failed. Error code = ");
      Serial.println(client.state());
      Serial.println("Retrying...");
      delay(MQTT_RETRY_DELAY);
    }
  }
}

String get_formatted_timestamp() {
  char time_str[30];
  time_t now = time(nullptr);
  strftime(time_str, sizeof(time_str), "%Y-%m-%dT%H:%M:%SZ", gmtime(&now));
  return String(time_str);
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
  Serial.begin(115200);
  while (!Serial) {
    ;
  }

  connect_wifi();

  configTime(TIMEZONE, NTP_SERVER);
  while (time(nullptr) < 8 * 3600 * 2) {
    delay(500);
    Serial.print(".");
  }

  client.setServer(MQTT_BROKER_IP_VALUE, 1883);
  connect_mqtt();
}

void loop() {
  // In case mqtt connection is no longer active
  if (!client.connected()) {
    connect_mqtt();
  }

  unsigned long now = millis();
  if (now - lastSendTime > SEND_INTERVAL) {
    lastSendTime = now;
    String payload = get_moisture_reading();
    client.publish(MQTT_PUBLISH_TOPIC, payload.c_str(), true);  // 'true' arg ensures message retention
  }
}
