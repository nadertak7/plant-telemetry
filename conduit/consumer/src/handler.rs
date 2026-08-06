use crate::schema::message::SensorPayload;
use crate::schema::sensor::Sensor;
use rumqttc::Publish;
use sqlx::PgPool;

fn parse_payload(message: &Publish) -> Option<SensorPayload> {
    match serde_json::from_slice(&message.payload) {
        Ok(payload) => {
            tracing::info!(topic=message.topic, payload=?payload, "Parsed sensor payload.");
            Some(payload)
        }
        Err(e) => {
            tracing::warn!(
              topic = message.topic,
                payload = ?message.payload,
                error = %e,
                "Error parsing payload."
            );
            None
        }
    }
}

async fn get_sensor_record(topic: &String, pool: &PgPool) -> Option<Sensor> {
    let result = sqlx::query_as!(
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
    .await;

    match result {
        Ok(Some(sensor)) => {
            tracing::info!(topic=topic, sensor=?sensor, "Successfully retrieved sensor record.");
            Some(sensor)
        }
        Ok(None) => {
            tracing::info!(
                topic = topic,
                "Received message for a non-existent topic. Register the sensor."
            );
            None
        }
        Err(e) => {
            tracing::warn!(topic=topic, error=%e, "Error querying topic.");
            None
        }
    }
}

pub async fn handle_message(message: &Publish, pool: &PgPool) {
    let Some(_) = parse_payload(message) else {
        return;
    };
    let Some(_) = get_sensor_record(&message.topic, pool).await else {
        return;
    };
}
