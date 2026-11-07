use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::error::sensor::SensorCreateError;

pub mod sensor;

pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "something went wrong".to_string(),
                )
            }
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("resource not found".into()),
            _ => AppError::Internal(err.to_string()),
        }
    }
}

impl From<SensorCreateError> for AppError {
    fn from(err: SensorCreateError) -> Self {
        match err {
            SensorCreateError::DuplicateTopic { .. } => AppError::Conflict(err.to_string()),
            SensorCreateError::InvalidAdc { .. } | SensorCreateError::PlantNotFound { .. } => {
                AppError::BadRequest(err.to_string())
            }
            SensorCreateError::Database(_) => AppError::Internal(err.to_string()),
        }
    }
}
