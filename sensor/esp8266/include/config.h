#pragma once

// Sensor settings
inline constexpr int MOISTURE_SENSOR_PIN = A0;
inline constexpr const char* MQTT_CLIENT_ID = "ESP8266_Sensor_01";
inline constexpr const char* MQTT_TOPIC = "plant-monitoring/living-room/scarlet-star-1/telemetry";
inline constexpr int ADC_VALUE_DRY = 666;
inline constexpr int ADC_VALUE_WET = 272;

// Retry settings
inline constexpr int MQTT_MAX_RETRIES = 5;
inline constexpr int WIFI_MAX_RETRIES = 5;
inline constexpr int TIME_SYNC_MAX_RETRIES = 5;

// Time settings
inline constexpr int SLEEP_DURATION_SUCCESS_SECS = 300;
inline constexpr int SLEEP_DURATION_ERROR_SECS = 10;
inline constexpr int WIFI_RETRY_DELAY_MS = 10000;
inline constexpr int MQTT_RETRY_DELAY_MS = 10000;
inline constexpr int TIME_SYNC_RETRY_DELAY_MS = 500;
// NTP specific time settings
inline constexpr const char* NTP_SERVER = "pool.ntp.org";
inline constexpr const char* NTP_TIMEZONE = "UTC0";
inline constexpr long MIN_VALID_TIME_UNIX = 1735689600L; // 1st January 2025
