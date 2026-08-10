use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Error parsing payload: {0}.")]
    PayloadParseError(#[from] serde_json::Error),
    #[error("Error querying database: {0}.")]
    QueryError(#[from] sqlx::Error),
    #[error("Topic from message was not found. Register the sensor that is bound to the topic.")]
    SensorNotRegistered,
}

impl HandlerError {
    pub fn is_operational_error(&self) -> bool {
        match self {
            Self::QueryError(_) => true,
            _ => false,
        }
    }
}

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
