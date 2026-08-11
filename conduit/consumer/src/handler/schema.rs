use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::handler::error::HandlerError;

#[derive(Deserialize, Debug)]
pub struct SensorPayload {
    pub adc: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct SensorRecord {
    pub id: i32,
    pub plant_id: Option<i32>,
    pub dry_adc: i32,
    pub wet_adc: i32,
}

pub struct Sensor {
    id: i32,
    plant_id: i32,
    pub dry_adc: i32,
    pub wet_adc: i32,
}

impl TryFrom<&SensorRecord> for Sensor {
    type Error = HandlerError;

    fn try_from(sensor_row: &SensorRecord) -> Result<Self, HandlerError> {
        let plant_id = sensor_row
            .plant_id
            .ok_or(HandlerError::PlantNotRegistered)?;

        Ok(Self {
            id: sensor_row.id,
            plant_id,
            dry_adc: sensor_row.dry_adc,
            wet_adc: sensor_row.wet_adc,
        })
    }
}

impl Sensor {
    pub fn check_adc(&self, adc: i32) -> Result<(), HandlerError> {
        if !(self.wet_adc..=self.dry_adc).contains(&adc) {
            return Err(HandlerError::AdcNotInRange);
        }
        Ok(())
    }
}
