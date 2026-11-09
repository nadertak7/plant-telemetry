use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    error::sensor::SensorMutateError,
    schema::sensor::{SensorCreate, SensorListParameters, SensorRecord, SensorUpdate},
};

fn query_base() -> QueryBuilder<Postgres> {
    QueryBuilder::new(
        r#"
        SELECT
            sensor.id,
            sensor.topic,
            plant.id "plant_id",
            plant.display_name "plant_display_name",
            plant.archived_at "plant_archived_at",
            sensor.dry_adc,
            sensor.wet_adc,
            sensor.created_at,
            sensor.updated_at,
            sensor.archived_at "sensor_archived_at",
            sensor.last_active_at
        FROM
            sensor
        LEFT JOIN
            plant
            ON plant.id = sensor.plant_id
        "#,
    )
}

pub async fn get_sensor_by_id(id: i32, db: &PgPool) -> Result<Option<SensorRecord>, sqlx::Error> {
    let mut query_base = query_base();
    let sensor = query_base
        .push(" WHERE sensor.id = ")
        .push_bind(id)
        .build_query_as::<SensorRecord>()
        .fetch_optional(db)
        .await?;
    Ok(sensor)
}

pub async fn get_sensor_list(
    parameters: SensorListParameters,
    db: &PgPool,
) -> Result<Vec<SensorRecord>, sqlx::Error> {
    let mut query_base = query_base();
    query_base.push(" WHERE TRUE");
    if let Some(topic) = parameters.topic {
        query_base
            .push(" AND topic LIKE ")
            .push_bind(format!("%{}%", topic));
    };
    if let Some(archived) = parameters.archived {
        if !archived {
            query_base.push(" AND sensor.archived_at IS NULL");
        } else {
            query_base.push(" AND sensor.archived_at IS NOT NULL");
        }
    }
    let sensors = query_base
        .build_query_as::<SensorRecord>()
        .fetch_all(db)
        .await?;
    Ok(sensors)
}

fn handle_mutate_result<T>(
    result: Result<T, sqlx::Error>,
    topic_in: &str,
    dry_adc_in: u16,
    wet_adc_in: u16,
    plant_id_in: Option<i32>,
) -> Result<T, SensorMutateError> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            if let Some(db_error) = e.as_database_error() {
                match db_error.constraint() {
                    Some("ck_sensor_dry_adc_wet_adc") => {
                        return Err(SensorMutateError::InvalidAdc {
                            dry_adc: dry_adc_in,
                            wet_adc: wet_adc_in,
                        });
                    }
                    Some("ux_sensor_topic") => {
                        return Err(SensorMutateError::DuplicateTopic {
                            topic: topic_in.into(),
                        });
                    }
                    Some("fk_sensor_plant") => {
                        if let Some(plant_id) = plant_id_in {
                            return Err(SensorMutateError::PlantNotFound { id: plant_id });
                        }
                    }
                    _ => {}
                }
            };
            Err(SensorMutateError::Database(e))
        }
    }
}

pub async fn create_sensor(
    payload: &SensorCreate,
    db: &PgPool,
) -> Result<SensorRecord, SensorMutateError> {
    let plant_id_in = payload.plant.as_ref().map(|plant| plant.id);
    let result = sqlx::query_scalar!(
        r#"
        INSERT INTO
            sensor
            (topic, dry_adc, wet_adc, plant_id)
        VALUES
            ($1, $2, $3, $4)
        RETURNING
            id
        "#,
        payload.topic,
        i32::from(payload.dry_adc),
        i32::from(payload.wet_adc),
        plant_id_in
    )
    .fetch_one(db)
    .await;
    let id = handle_mutate_result(
        result,
        &payload.topic,
        payload.dry_adc,
        payload.wet_adc,
        plant_id_in,
    )?;
    get_sensor_by_id(id, db)
        .await?
        .ok_or(sqlx::Error::RowNotFound.into())
}

pub async fn update_sensor(
    payload: &SensorUpdate,
    id: i32,
    db: &PgPool,
) -> Result<SensorRecord, SensorMutateError> {
    let plant_id_in = payload.plant.as_ref().map(|plant| plant.id);
    let result = sqlx::query!(
        r#"
        UPDATE
            sensor
        SET
            topic = $1,
            dry_adc = $2,
            wet_adc = $3,
            plant_id = $4,
            updated_at = NOW(),
            archived_at =
                CASE
                    WHEN $6
                    THEN COALESCE(archived_at, NOW())
                    ELSE NULL
                END
        WHERE
            id = $5
            AND (archived_at IS NULL OR NOT $6)
        "#,
        payload.topic,
        i32::from(payload.dry_adc),
        i32::from(payload.wet_adc),
        plant_id_in,
        id,
        payload.archived
    )
    .execute(db)
    .await;
    let result = handle_mutate_result(
        result,
        &payload.topic,
        payload.dry_adc,
        payload.wet_adc,
        plant_id_in,
    )?;
    if result.rows_affected() == 0 {
        return Err(SensorMutateError::SensorNotFound { id });
    }
    get_sensor_by_id(id, db)
        .await?
        .ok_or(sqlx::Error::RowNotFound.into())
}
