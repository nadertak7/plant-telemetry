use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Error parsing payload: {0}.")]
    PayloadParseError(#[from] serde_json::Error),
    #[error("Error querying database: {0}.")]
    QueryError(#[from] sqlx::Error),
    #[error("Topic from message was not found. Register the sensor that is bound to the topic.")]
    SensorNotRegistered,
    #[error("Sensor is archived. Restore the sensor.")]
    SensorArchived,
    #[error("No plant bound to sensor. Register the plant.")]
    PlantNotRegistered,
    #[error("The plant bound to the sensor is archived. Restore the plant.")]
    PlantArchived,
    #[error(
        "The received adc value is not between the dry and wet adc values of the sensor. Recalibrate the sensor."
    )]
    AdcNotInRange,
    #[error(
        "The dry adc value can not be less than or equal to the wet adc value. Recalibrate the sensor."
    )]
    InvalidSensorCalibration,
}

impl HandlerError {
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::QueryError(_))
    }
}
