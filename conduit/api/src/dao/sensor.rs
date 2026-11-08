use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    error::sensor::SensorMutateError,
    schema::sensor::{SensorListParameters, SensorMutate, SensorRecord},
};

fn query_base() -> QueryBuilder<Postgres> {
    QueryBuilder::new(
        r#"
        SELECT
            sensor.id,
            sensor.topic,
            plant.id "plant_id",
            plant.display_name "plant_display_name",
            sensor.dry_adc,
            sensor.wet_adc,
            sensor.created_at,
            sensor.updated_at,
            sensor.archived_at,
            sensor.last_active_at
        FROM
            sensor
        LEFT JOIN
            plant
            ON plant.id = sensor.plant_id
            AND plant.archived_at is NULL
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

pub async fn check_active_sensor_exists_by_id(id: i32, db: &PgPool) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT
            EXISTS(
                SELECT
                    1
                FROM
                    sensor
                WHERE
                    sensor.id = $1
                AND
                    sensor.archived_at IS NULL
            ) "exists!"
        "#,
        id
    )
    .fetch_one(db)
    .await?;
    Ok(result)
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
    payload: &SensorMutate,
) -> Result<T, SensorMutateError> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            if let Some(db_error) = e.as_database_error() {
                match db_error.constraint() {
                    Some("ck_sensor_dry_adc_wet_adc") => {
                        return Err(SensorMutateError::InvalidAdc {
                            dry_adc: payload.dry_adc,
                            wet_adc: payload.wet_adc,
                        });
                    }
                    Some("ux_sensor_topic") => {
                        return Err(SensorMutateError::DuplicateTopic {
                            topic: payload.topic.clone(),
                        });
                    }
                    _ => {}
                }
            };
            return Err(SensorMutateError::Database(e));
        }
    }
}

pub async fn create_sensor(
    payload: &SensorMutate,
    db: &PgPool,
) -> Result<SensorRecord, SensorMutateError> {
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
        payload.plant.as_ref().map(|plant| plant.id)
    )
    .fetch_one(db)
    .await;
    let id = handle_mutate_result(result, payload)?;
    get_sensor_by_id(id, db)
        .await?
        .ok_or(sqlx::Error::RowNotFound.into())
}

pub async fn update_sensor(
    payload: &SensorMutate,
    id: i32,
    db: &PgPool,
) -> Result<SensorRecord, SensorMutateError> {
    let result = sqlx::query!(
        r#"
        UPDATE
            sensor
        SET
            topic = $1,
            dry_adc = $2,
            wet_adc = $3,
            plant_id = $4,
            updated_at = NOW()
        WHERE
            id = $5
        "#,
        payload.topic,
        i32::from(payload.dry_adc),
        i32::from(payload.wet_adc),
        payload.plant.as_ref().map(|plant| plant.id),
        id
    )
    .execute(db)
    .await;
    handle_mutate_result(result, payload)?;
    get_sensor_by_id(id, db)
        .await?
        .ok_or(sqlx::Error::RowNotFound.into())
}
