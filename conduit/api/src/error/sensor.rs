use thiserror::Error;

#[derive(Error, Debug)]
pub enum SensorCreateError {
    #[error("the provided plant id does not exist: {}", id)]
    PlantNotFound { id: i32 },
    #[error(
        "the wet adc value {} must not exceed the dry adc value {}",
        wet_adc,
        dry_adc
    )]
    InvalidAdc { dry_adc: u16, wet_adc: u16 },
    #[error("topic already exists: {}", topic)]
    DuplicateTopic { topic: String },
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
