use rumqttc::{Event, Packet};
use shared::db;
use shared::logger;
use shared::settings::Settings;
use std::time::Duration;

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

    println!("Starting to poll");
    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Packet::Publish(message))) => {
                let sensor_payload: SensorPayload = match serde_json::from_slice(&message.payload) {
                    Ok(payload) => payload,
                    Err(e) => {
                        println!(
                            "Failed to parse payload. Topic: {}. Payload: {:?}. Error: {e}.",
                            &message.topic, &message.payload
                        );
                        continue;
                    }
                };
                println!(
                    "Successfully parsed SensorPayload: topic: {}, adc: {}, timestamp: {}",
                    message.topic, sensor_payload.adc, sensor_payload.timestamp
                );
            }
            Ok(_) => {}
            Err(e) => {
                println!("MqttError: {e}");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
