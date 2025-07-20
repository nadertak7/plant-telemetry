const int kMoistureSensorPin = A0;
const int kReadIntervalMs = 500; // Time between reads

void setup() {
  Serial.begin(115200);
  while (!Serial) {};
}

void loop() {
  int adc_value_reading = analogRead(kMoistureSensorPin);
  Serial.println(adc_value_reading);
  delay(kReadIntervalMs);
}
