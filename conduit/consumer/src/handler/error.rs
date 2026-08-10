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
    pub fn is_transient(&self) -> bool {
        match self {
            Self::QueryError(_) => false,
            _ => true,
        }
    }
}
