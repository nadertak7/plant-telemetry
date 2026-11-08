use std::sync::Arc;

use axum::Json;
use axum::extract;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Router, routing::get};

use crate::AppState;
use crate::dao;
use crate::error::AppError;
use crate::error::sensor::SensorMutateError;
use crate::schema::sensor::{SensorListParameters, SensorMutate, SensorRead, SensorRecord};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sensor/{id}", get(read).put(update))
        .route("/sensor", get(list).post(create))
}

/// Get sensor details from a provided id.
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

/// Create a sensor from a given payload.
async fn create(
    State(state): State<Arc<AppState>>,
    extract::Json(payload): Json<SensorMutate>,
) -> Result<(StatusCode, Json<SensorRead>), AppError> {
    if let Some(plant) = &payload.plant {
        let plant_exists = dao::plant::check_active_plant_exists_by_id(plant.id, &state.db).await?;
        if !plant_exists {
            return Err(SensorMutateError::PlantNotFound { id: plant.id }.into());
        }
    }
    let sensor = dao::sensor::create_sensor(&payload, &state.db).await?;
    Ok((StatusCode::CREATED, Json(sensor.into())))
}

/// Update an existing sensor with a given payload.
async fn update(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    extract::Json(payload): Json<SensorMutate>,
) -> Result<(StatusCode, Json<SensorRead>), AppError> {
    let sensor_exists = dao::sensor::check_active_sensor_exists_by_id(id, &state.db).await?;
    if !(sensor_exists) {
        return Err(SensorMutateError::SensorNotFound { id }.into());
    }
    if let Some(plant) = &payload.plant {
        let plant_exists = dao::plant::check_active_plant_exists_by_id(plant.id, &state.db).await?;
        if !plant_exists {
            return Err(SensorMutateError::PlantNotFound { id: plant.id }.into());
        }
    }
    let sensor = dao::sensor::update_sensor(&payload, id, &state.db).await?;
    Ok((StatusCode::OK, Json(sensor.into())))
}
