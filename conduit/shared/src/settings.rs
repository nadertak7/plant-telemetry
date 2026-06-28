use sqlx::postgres::PgConnectOptions;
use std::env;
use std::error::Error;

pub struct DatabaseSettings {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .database(&self.database)
    }
}

pub struct Settings {
    pub database_settings: DatabaseSettings,
}

impl Settings {
    pub fn load() -> Result<Settings, Box<dyn Error>> {
        if let Err(err) = dotenvy::dotenv() {
            log::warn!("Unable to find .env file: {err}");
        }
        Ok(Settings {
            database_settings: DatabaseSettings {
                host: "127.0.0.1".to_string(),
                port: 5432,
                username: env::var("POSTGRES_SUPER_USERNAME")?,
                password: env::var("POSTGRES_SUPER_PASSWORD")?,
                database: env::var("POSTGRES_DB")?,
            },
        })
    }
}
