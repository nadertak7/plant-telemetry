use anyhow::Context;
use sqlx::postgres::PgConnectOptions;
use std::env;

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

pub struct Settings {
    pub database_settings: DatabaseSettings,
}

impl Settings {
    pub fn new() -> anyhow::Result<Settings> {
        Ok(Settings {
            database_settings: DatabaseSettings {
                database_url: env::var("DATABASE_URL")
                    .context("Could not find DATABASE_URL in environment.")?,
                max_connections: 5,
            },
        })
    }
}
