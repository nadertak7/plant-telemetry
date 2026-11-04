use shared::db;
use shared::logger;
use shared::settings::DatabaseSettings;
use shared::settings::Settings;
use sqlx::PgPool;
use std::sync::Arc;

pub mod dao;
pub mod endpoints;
pub mod error;
pub mod schema;

pub struct AppState {
    db: PgPool,
}

async fn try_setup() -> anyhow::Result<PgPool> {
    let database_settings = DatabaseSettings::new()?;
    let pool = db::create_connection_pool(&database_settings).await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::initialise();
    let pool = match try_setup().await {
        Ok(v) => {
            tracing::info!("application setup successful");
            v
        }
        Err(e) => {
            tracing::error!(error=%e, "application setup failed");
            return Err(e);
        }
    };
    let state = Arc::new(AppState { db: pool });
    let app = endpoints::router().with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6767").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
