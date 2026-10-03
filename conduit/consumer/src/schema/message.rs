use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The expected message format from a sensor.
///
/// Convert the incoming unix timestamp to UTC.
#[derive(Deserialize, Serialize, Debug)]
pub struct SensorMessage {
    pub adc: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}
