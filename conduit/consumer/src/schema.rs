use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::error::HandlerError;

#[derive(Deserialize, Debug)]
pub struct SensorPayload {
    pub adc: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct SensorRecord {
    pub id: i32,
    pub plant_id: Option<i32>,
    pub dry_adc: i32,
    pub wet_adc: i32,
}

pub struct Sensor {
    pub id: i32,
    pub plant_id: i32,
    pub dry_adc: i32,
    pub wet_adc: i32,
}

impl TryFrom<&SensorRecord> for Sensor {
    type Error = HandlerError;

    fn try_from(sensor_row: &SensorRecord) -> Result<Self, HandlerError> {
        let plant_id = sensor_row
            .plant_id
            .ok_or(HandlerError::PlantNotRegistered)?;

        if sensor_row.dry_adc <= sensor_row.wet_adc {
            return Err(HandlerError::InvalidSensorCalibration);
        }

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

    pub fn calculate_moisture_perc(&self, adc: i32) -> f64 {
        let adc = adc as f64;
        let dry_adc = self.dry_adc as f64;
        let wet_adc = self.wet_adc as f64;
        100.0 * ((adc - wet_adc) / (dry_adc - wet_adc))
    }
}
