use rumqttc::{Event, Packet};
use shared::db;
use shared::logger;
use shared::settings::Settings;
use std::time::Duration;
use tracing::Level;

use crate::schema::sensor::SensorPayload;

mod mqtt;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    logger::initialise();
    let settings = Settings::new()?;
    let pool = db::create_connection_pool(&settings.database_settings).await?;
    db::run_migrations(&pool).await?;
    let (mqtt_client, mut event_loop) = mqtt::get_client(&settings.mqtt_settings);
    mqtt::subscribe(&mqtt_client, &settings.mqtt_settings).await?;

    tracing::event!(Level::INFO, "Starting poll.");
    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Packet::Publish(message))) => {
                let sensor_payload: SensorPayload = match serde_json::from_slice(&message.payload) {
                    Ok(payload) => payload,
                    Err(e) => {
                        tracing::warn!(
                            topic = message.topic,
                            payload = ?message.payload,
                            error = %e,
                            "Error parsing payload."
                        );
                        continue;
                    }
                };
                tracing::info!(topic=message.topic, payload=?sensor_payload, "Parsed sensor payload.")
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error=%e, "MQTT connection error.");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
