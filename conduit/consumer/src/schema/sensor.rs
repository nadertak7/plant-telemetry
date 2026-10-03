use crate::error::HandlerError;

/// Sensor data retrieved from the database, including plant archival status.
///
/// Convert to [`Sensor`] to validate it before processing readings.
#[derive(Debug)]
pub struct SensorRecord {
    pub id: i32,
    pub plant_id: Option<i32>,
    pub dry_adc: i32,
    pub wet_adc: i32,
    pub is_sensor_archived: bool,
    pub is_plant_archived: bool,
}

/// A validated [`SensorRecord`], containing the data needed for message handling.
pub struct Sensor {
    pub id: i32,
    pub plant_id: i32,
    pub dry_adc: i32,
    pub wet_adc: i32,
}

impl TryFrom<&SensorRecord> for Sensor {
    type Error = HandlerError;

    /// Ensure that:
    ///
    /// 1. The sensor has an associated plant.
    /// 2. The sensor's dry adc is higher than the sensor's wet adc.
    /// 3. The sensor is not archived.
    /// 4. The plant associated with the sensor is not archived.
    fn try_from(sensor_row: &SensorRecord) -> Result<Self, HandlerError> {
        let plant_id = sensor_row
            .plant_id
            .ok_or(HandlerError::PlantNotRegistered)?;

        if sensor_row.dry_adc <= sensor_row.wet_adc {
            return Err(HandlerError::InvalidSensorCalibration);
        }

        if sensor_row.is_sensor_archived {
            return Err(HandlerError::SensorArchived);
        }

        if sensor_row.is_plant_archived {
            return Err(HandlerError::PlantArchived);
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
    /// Indicate if the recorded adc from a message is between its sensor's dry and wet adc.
    pub fn check_adc(&self, adc: i32) -> Result<(), HandlerError> {
        if !(self.wet_adc..=self.dry_adc).contains(&adc) {
            return Err(HandlerError::AdcNotInRange);
        }
        Ok(())
    }

    /// Convert an ADC reading to moisture percentage: dry is 0%, wet is 100%.
    ///
    /// Does not validate the reading: Call [`Sensor::check_adc`] first.
    pub fn calculate_moisture_perc(&self, adc: i32) -> f64 {
        let adc = adc as f64;
        let dry_adc = self.dry_adc as f64;
        let wet_adc = self.wet_adc as f64;
        100.0 * ((dry_adc - adc) / (dry_adc - wet_adc))
    }
}
