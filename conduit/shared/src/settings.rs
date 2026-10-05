use anyhow::Context;
use rumqttc::{MqttOptions, QoS};
use sqlx::postgres::PgConnectOptions;
use std::fs;
use std::path::Path;
use std::{env, time::Duration};

pub struct DatabaseSettings {
    database_url: String,
    pub max_connections: u32,
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> anyhow::Result<PgConnectOptions> {
        self.database_url
            .parse()
            .context("Failed to parse database URL.")
    }
}

pub struct MqttSettings {
    id: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    keep_alive_seconds: Duration,
    pub subscribe_topic: String,
    pub request_queue_capacity: usize,
    pub quality_of_service: QoS,
}

impl MqttSettings {
    pub fn connect_options(&self) -> MqttOptions {
        let mut mqtt_options = MqttOptions::new(&self.id, &self.host, self.port);
        mqtt_options
            .set_credentials(&self.username, &self.password)
            .set_keep_alive(self.keep_alive_seconds);
        mqtt_options
    }
}

pub struct Settings {
    pub database_settings: DatabaseSettings,
    pub mqtt_settings: MqttSettings,
}

/// Read the value of a secret file stored in /run/secrets/
fn read_secret(name: &str) -> anyhow::Result<String> {
    let secret_filepath = Path::new("/run/secrets").join(name);
    fs::read_to_string(&secret_filepath).with_context(|| {
        format!(
            "Failed to read from secrets file: {}",
            secret_filepath.display()
        )
    })
}

impl Settings {
    pub fn new() -> anyhow::Result<Settings> {
        Ok(Settings {
            database_settings: DatabaseSettings {
                database_url: read_secret("postgres_database_url")?,
                max_connections: 5,
            },
            mqtt_settings: MqttSettings {
                id: "conduit-consumer".to_string(),
                host: env::var("MQTT_HOST").context("Could not find MQTT_HOST in environment.")?,
                port: 1883,
                username: read_secret("mqtt_username")?,
                password: read_secret("mqtt_password")?,
                subscribe_topic: "sensor/+".to_string(),
                keep_alive_seconds: Duration::from_secs(60),
                request_queue_capacity: 10,
                quality_of_service: QoS::AtMostOnce,
            },
        })
    }
}
