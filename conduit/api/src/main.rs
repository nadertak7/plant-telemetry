mod dao;
mod endpoints;
mod error;
mod schema;
#[cfg(test)]
mod tests;

use axum::Router;
use shared::db;
use shared::logger;
use shared::settings::DatabaseSettings;
use shared::settings::Settings;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnRequest;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::TraceLayer;
use tracing::Level;

pub struct AppState {
    db: PgPool,
}

async fn try_setup() -> anyhow::Result<PgPool> {
    let database_settings = DatabaseSettings::new()?;
    let pool = db::create_connection_pool(&database_settings).await?;
    Ok(pool)
}

/// Create the application's router.
/// Functional so that it can be mocked in testing.
fn app(pool: Pool<Postgres>) -> Router {
    let state = Arc::new(AppState { db: pool });
    endpoints::router().with_state(state).layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Micros),
                ),
        ),
    )
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
    let app = app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6767").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
