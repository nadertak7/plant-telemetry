use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

use crate::schema::common::ObjectWithId;

#[derive(sqlx::FromRow)]
pub struct SensorRecord {
    pub id: i32,
    pub topic: String,
    pub dry_adc: i32,
    pub wet_adc: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
    pub last_active_at: Option<DateTime<Utc>>,
    pub plant_id: Option<i32>,
    pub plant_display_name: Option<String>,
}

#[derive(Serialize)]
pub struct SensorPlant {
    id: i32,
    display_name: String,
}

#[derive(Serialize)]
pub struct SensorRead {
    id: i32,
    topic: String,
    dry_adc: i32,
    wet_adc: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    archived_at: Option<DateTime<Utc>>,
    last_active_at: Option<DateTime<Utc>>,
    plant: Option<SensorPlant>,
}

impl From<SensorRecord> for SensorRead {
    fn from(row: SensorRecord) -> Self {
        let plant = if let (Some(id), Some(display_name)) = (row.plant_id, row.plant_display_name) {
            Some(SensorPlant { id, display_name })
        } else {
            None
        };
        SensorRead {
            id: row.id,
            topic: row.topic,
            dry_adc: row.dry_adc,
            wet_adc: row.wet_adc,
            created_at: row.created_at,
            updated_at: row.updated_at,
            archived_at: row.archived_at,
            last_active_at: row.last_active_at,
            plant,
        }
    }
}

#[derive(Deserialize)]
pub struct SensorListParameters {
    pub topic: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Deserialize)]
pub struct SensorCreate {
    pub topic: String,
    pub dry_adc: u16,
    pub wet_adc: u16,
    pub plant: Option<ObjectWithId>,
}
