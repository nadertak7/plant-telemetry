mod error;
mod handler;
mod mqtt;
mod schema;

use rumqttc::{Event, Packet};
use shared::db;
use shared::logger;
use shared::settings::Settings;
use sqlx::{Pool, Postgres};
use std::time::Duration;

/// Attempt setup steps that can fail, so errors can be propagated to logs in [`main`].
async fn try_setup() -> anyhow::Result<(Settings, Pool<Postgres>)> {
    let settings = Settings::new()?;
    let pool = db::create_connection_pool(&settings.database_settings).await?;
    db::run_migrations(&pool).await?;
    Ok((settings, pool))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::initialise();
    let (settings, pool) = match try_setup().await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(error=%e, "Application setup failed.");
            return Err(e);
        }
    };

    db::run_migrations(&pool).await?;
    let (mqtt_client, mut event_loop) = mqtt::get_client(&settings.mqtt_settings);

    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Packet::Publish(message))) => {
                // Spawn task per message to not block mqtt event loop.
                let pool_cloned = pool.clone();
                tokio::spawn(async move { handler::handle_message(&message, &pool_cloned).await });
            }
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                mqtt::subscribe(&mqtt_client, &settings.mqtt_settings).await?;
            }
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error=%e, "MQTT connection error.");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
