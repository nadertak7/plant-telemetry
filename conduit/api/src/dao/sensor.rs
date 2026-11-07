use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::schema::sensor::{SensorCreate, SensorListParameters, SensorRecord};

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

pub async fn create_sensor(
    payload: &SensorCreate,
    db: &PgPool,
) -> Result<SensorRecord, sqlx::Error> {
    let id = sqlx::query_scalar!(
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
        payload.dry_adc,
        payload.wet_adc,
        payload.plant.as_ref().map(|plant| plant.id)
    )
    .fetch_one(db)
    .await?;
    get_sensor_by_id(id, db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}
