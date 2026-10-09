mod error;
mod handler;
mod mqtt;
mod schema;

use rumqttc::{Event, Packet};
use shared::db;
use shared::logger;
use shared::settings::DatabaseSettings;
use shared::settings::MqttSettings;
use shared::settings::Settings;
use sqlx::{Pool, Postgres};
use std::time::Duration;

/// Attempt setup steps that can fail, so errors can be propagated to logs in [`main`].
async fn try_setup() -> anyhow::Result<(MqttSettings, Pool<Postgres>)> {
    let database_settings = DatabaseSettings::new()?;
    let mqtt_settings = MqttSettings::new()?;
    let pool = db::create_connection_pool(&database_settings).await?;
    db::run_migrations(&pool).await?;
    Ok((mqtt_settings, pool))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::initialise();
    let (mqtt_settings, pool) = match try_setup().await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(error=%e, "Application setup failed.");
            return Err(e);
        }
    };
    let (mqtt_client, mut event_loop) = mqtt::get_client(&mqtt_settings);

    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Packet::Publish(message))) => {
                // Spawn task per message to not block mqtt event loop.
                let pool_cloned = pool.clone();
                tokio::spawn(async move { handler::handle_message(&message, &pool_cloned).await });
            }
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                mqtt::subscribe(&mqtt_client, &mqtt_settings).await?;
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error=%e, "MQTT connection error.");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
