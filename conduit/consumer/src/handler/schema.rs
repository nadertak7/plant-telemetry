use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SensorPayload {
    adc: u16,
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Sensor {
    pub id: i32,
    pub plant_id: Option<i32>,
    pub dry_adc: i32,
    pub wet_adc: i32,
}
