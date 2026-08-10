use crate::schema::sensor::{HandlerError, Sensor, SensorPayload};
use rumqttc::Publish;
use sqlx::PgPool;

fn parse_payload(payload: &[u8]) -> Result<SensorPayload, serde_json::Error> {
    serde_json::from_slice(payload)
}

async fn get_sensor_record(topic: &str, pool: &PgPool) -> Result<Option<Sensor>, sqlx::Error> {
    sqlx::query_as!(
        Sensor,
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

async fn try_handle_message(
    topic: &str,
    payload: &[u8],
    pool: &PgPool,
) -> Result<(), HandlerError> {
    let _ = parse_payload(payload)?;
    let _ = get_sensor_record(topic, pool)
        .await?
        .ok_or(HandlerError::SensorNotRegistered)?;
    Ok(())
}

pub async fn handle_message(message: &Publish, pool: &PgPool) {
    let payload = &message.payload;
    let topic = &message.topic;
    match try_handle_message(topic, payload, pool).await {
        Ok(()) => tracing::info!(topic=topic, payload=?payload, "Successfully handled message."),
        Err(e) if e.is_operational_error() => {
            tracing::error!(topic=topic, payload=?payload, error=%e, "There was an error while handling the message.");
        }
        Err(e) => {
            tracing::warn!(topic=topic, payload=?payload, error=%e, "Ignoring unprocessable message.")
        }
    }
}
