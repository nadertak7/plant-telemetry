#[cfg(test)]
mod tests;

use crate::settings::DatabaseSettings;
use anyhow::Context;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn create_connection_pool(
    database_settings: &DatabaseSettings,
) -> anyhow::Result<Pool<Postgres>> {
    let connect_options = database_settings.connect_options()?;

    PgPoolOptions::new()
        .max_connections(database_settings.max_connections)
        .connect_with(connect_options)
        .await
        .context("Failed to connect to database.")
}

pub static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

pub async fn run_migrations(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    MIGRATOR
        .run(pool)
        .await
        .context("Failed to run migrations.")
}
