use anyhow::Context;
use rumqttc::{MqttOptions, QoS};
use sqlx::postgres::PgConnectOptions;
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
    keep_alive_seconds: u64,
    pub subscribe_topic: String,
    pub request_queue_capacity: usize,
    pub quality_of_service: QoS,
}

impl MqttSettings {
    pub fn connect_options(&self) -> MqttOptions {
        let mut mqtt_options = MqttOptions::new(&self.id, &self.host, self.port);
        mqtt_options
            .set_credentials(&self.username, &self.password)
            .set_keep_alive(Duration::from_secs(self.keep_alive_seconds));
        mqtt_options
    }
}

pub struct Settings {
    pub database_settings: DatabaseSettings,
    pub mqtt_settings: MqttSettings,
}

impl Settings {
    pub fn new() -> anyhow::Result<Settings> {
        Ok(Settings {
            database_settings: DatabaseSettings {
                database_url: env::var("DATABASE_URL")
                    .context("Could not find DATABASE_URL in environment.")?,
                max_connections: 5,
            },
            mqtt_settings: MqttSettings {
                id: "conduit-consumer".to_string(),
                host: "localhost".to_string(),
                port: 1883,
                username: env::var("MQTT_USERNAME")
                    .context("Could not find MQTT_USERNAME in environment.")?,
                password: env::var("MQTT_PASSWORD")
                    .context("Could not find MQTT_PASSWORD in environment.")?,
                subscribe_topic: "sensor/+".to_string(),
                keep_alive_seconds: 60,
                request_queue_capacity: 10,
                quality_of_service: QoS::AtMostOnce,
            },
        })
    }
}
