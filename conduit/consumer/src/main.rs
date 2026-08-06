use std::time::Duration;

use rumqttc::{Event, Packet};
use shared::db;
use shared::logger;
use shared::settings::Settings;

mod mqtt;

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
                println!(
                    "Received payload. Topic: {}. Payload: {:?}",
                    message.topic, message.payload
                );
            }
            Ok(_) => {}
            Err(e) => {
                println!("MqttError: {e}");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
