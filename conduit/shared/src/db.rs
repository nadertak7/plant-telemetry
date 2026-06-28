use crate::settings::DatabaseSettings;
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn create_connection_pool(
    database_settings: &DatabaseSettings,
) -> anyhow::Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(database_settings.max_connections)
        .connect_with(database_settings.connect_options())
        .await
        .context("Failed to connect to database.")
}
