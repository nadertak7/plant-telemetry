use crate::error::HandlerError;
use crate::schema::{Sensor, SensorPayload, SensorRecord};
use rumqttc::Publish;
use sqlx::{PgPool, postgres::PgQueryResult};

fn parse_payload(payload: &[u8]) -> Result<SensorPayload, serde_json::Error> {
    serde_json::from_slice(payload)
}

async fn get_sensor_record(
    topic: &str,
    pool: &PgPool,
) -> Result<Option<SensorRecord>, sqlx::Error> {
    sqlx::query_as!(
        SensorRecord,
        r#"
        SELECT
            id, plant_id, dry_adc, wet_adc
        FROM
            sensor
        WHERE
            topic = $1
        AND
            sensor.archived_at IS NULL
        "#,
        topic
    )
    .fetch_optional(pool)
    .await
}

async fn insert_reading(
    sensor: &Sensor,
    payload: &SensorPayload,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO
            plant_telemetry
            (plant_id, sensor_id, adc, moisture_perc, recorded_at)
        VALUES
            ($1, $2, $3, $4, $5)
        ON CONFLICT (sensor_id, recorded_at) DO NOTHING
        "#,
        sensor.plant_id,
        sensor.id,
        payload.adc,
        sensor.calculate_moisture_perc(payload.adc),
        payload.timestamp
    )
    .execute(pool)
    .await
}

async fn try_handle_message(
    topic: &str,
    payload: &[u8],
    pool: &PgPool,
) -> Result<(), HandlerError> {
    let payload = parse_payload(payload)?;
    let sensor_record = get_sensor_record(topic, pool)
        .await?
        .ok_or(HandlerError::SensorNotRegistered)?;
    let sensor = Sensor::try_from(&sensor_record)?;
    sensor.check_adc(payload.adc)?;
    insert_reading(&sensor, &payload, pool).await?;
    Ok(())
}

pub async fn handle_message(message: &Publish, pool: &PgPool) {
    let payload = &message.payload;
    let topic = &message.topic;
    match try_handle_message(topic, payload, pool).await {
        Ok(()) => tracing::info!(topic=topic, payload=?payload, "Successfully handled message."),
        Err(e) if e.is_transient() => {
            tracing::warn!(topic=topic, payload=?payload, error=%e, "Ignoring unprocessable message.");
        }
        Err(e) => {
            tracing::error!(topic=topic, payload=?payload, error=%e, "There was an error while handling the message.");
        }
    }
}
