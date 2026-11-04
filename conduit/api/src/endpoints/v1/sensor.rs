use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::{Router, routing::get};

use crate::AppState;
use crate::dao;
use crate::error::AppError;
use crate::schema::sensor::{SensorListParameters, SensorRead, SensorRecord};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sensor/{id}", get(read))
        .route("/sensor", get(list))
}

/// Get sensor details from an id.
async fn read(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SensorRead>, AppError> {
    let sensor: SensorRecord = dao::sensor::get_sensor_by_id(id, &state.db)
        .await?
        .ok_or(AppError::NotFound("resource not found".to_string()))?;
    Ok(Json(sensor.into()))
}

/// List sensors that match the given parameters.
async fn list(
    Query(params): Query<SensorListParameters>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SensorRead>>, AppError> {
    let sensors: Vec<SensorRead> = dao::sensor::get_sensor_list(params, &state.db)
        .await?
        .into_iter()
        .map(SensorRead::from)
        .collect();
    Ok(Json(sensors))
}
