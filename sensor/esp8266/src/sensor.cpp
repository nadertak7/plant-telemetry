#include <time.h>

#include <ArduinoJson.h>
#include <ESP8266WiFi.h>
#include <PubSubClient.h>

#include "config.h"
#include "secrets.h"

WiFiClient wifiClient;
PubSubClient mqttClient(wifiClient);

void logRetryAttempt(int attemptIndex, const int maxRetry, int errorCode = -999) {
    Serial.printf("Failed attempt %d of %d...\n", attemptIndex + 1, maxRetry);
    if (errorCode != -999) {
        Serial.printf("Error code %d\n", errorCode);
    }
}

bool connectWifi() {
    Serial.println("\nConnecting to Wifi network...");
    WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
    for (int i{}; i < WIFI_MAX_RETRIES; i++) {
        delay(WIFI_RETRY_DELAY_MS);
        if (WiFi.status() == WL_CONNECTED) {
            Serial.println("WiFi connected...");
            return true;
        }
        logRetryAttempt(i, WIFI_MAX_RETRIES, WiFi.status());
    }
    Serial.println("\nError: Failed to connect to wifi.");
    return false;
}

bool connectMqtt() {
    Serial.println("Connecting to MQTT...");
    mqttClient.setServer(MQTT_BROKER_ADDRESS, 1883);
    for (int i{}; i < MQTT_MAX_RETRIES; i++) {
        if (mqttClient.connect(MQTT_CLIENT_ID, MQTT_USERNAME, MQTT_PASSWORD)) {
            Serial.println("Connected to MQTT....");
            return true;
        }
        logRetryAttempt(i, MQTT_MAX_RETRIES, mqttClient.state());
        delay(MQTT_RETRY_DELAY_MS);
    }
    Serial.println("Error: Failed to connect to MQTT.");
    return false;
}

bool syncTime() {
    Serial.println("Syncing time from NTP server...");
    configTime(NTP_TIMEZONE, NTP_SERVER);
    for (int i{}; i < TIME_SYNC_MAX_RETRIES; i++) {
        if (time(nullptr) > MIN_VALID_TIME_UNIX) { // Later than 2025 (suggests successful sync)
            Serial.println("Time synced...");
            return true;
        }
        logRetryAttempt(i, TIME_SYNC_MAX_RETRIES);
        delay(TIME_SYNC_RETRY_DELAY_MS);
    }
    Serial.println("Error: Failed to sync time from NTP server.");
    return false;
}

String getFormattedTimestamp() {
    char timeStr[30];
    time_t now = time(nullptr);
    strftime(timeStr, sizeof(timeStr), "%Y-%m-%dT%H:%M:%SZ", gmtime(&now));
    return String(timeStr);
}

String getJsonPayload(int adcValueReading) {
    int moisturePercentage = map(adcValueReading, ADC_VALUE_DRY, ADC_VALUE_WET, 0, 100);
    // In case moisture percentage falls outside of 0-100 range
    moisturePercentage = constrain(moisturePercentage, 0, 100);

    // Create json document
    StaticJsonDocument<256> jsonDoc;
    jsonDoc["timestamp"] = getFormattedTimestamp();
    jsonDoc["adc_value"] = adcValueReading;
    jsonDoc["dry_value"] = ADC_VALUE_DRY;
    jsonDoc["wet_value"] = ADC_VALUE_WET;
    jsonDoc["moisture_perc"] = moisturePercentage;
    String jsonPayload;
    serializeJson(jsonDoc, jsonPayload);
    return jsonPayload;
}

void setup() {
    bool isTaskSuccessful = false; // Determines how long ESP should sleep for
    Serial.begin(115200);
    while (!Serial) {} // Wait for serial to initialise

    // Take sensor reading
    int adcValueReading = analogRead(MOISTURE_SENSOR_PIN);

    // If the sensor reading is significantly lower than the wet value,
    // it is likely that the sensor is not connected. A message should
    // not be sent to the MQTT broker
    if (adcValueReading < ADC_VALUE_WET - 50) {
        Serial.printf("\nInvalid sensor reading: %d\n", adcValueReading);
        Serial.println("Sensor not likely connected. Sleeping indefinitely...");
        ESP.deepSleep(0); // Infinite
    }
    Serial.printf("\nLogged moisture reading: %d", adcValueReading);
    String payload = getJsonPayload(adcValueReading);

    if (connectWifi() && connectMqtt() && syncTime()) {
        mqttClient.publish(MQTT_TOPIC, payload.c_str(), true);
        Serial.println("Published message to MQTT broker...");
        isTaskSuccessful = true;
    }

    // Prepare for deep sleep
    Serial.println("Sleeping...");
    mqttClient.disconnect();
    WiFi.disconnect();
    delay(100); // Give MQTT time to send before sleeping

    // Variably configure sleep time based on message being published successfully
    int sleepDurationSecs = isTaskSuccessful ? SLEEP_DURATION_SUCCESS_SECS : SLEEP_DURATION_ERROR_SECS;
    ESP.deepSleep(sleepDurationSecs * 1000000); // Time in microseconds
}

void loop() {
    // No need for loop as ESP resets after deep sleep
}
