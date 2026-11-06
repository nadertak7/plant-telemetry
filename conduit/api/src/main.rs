use shared::db;
use shared::logger;
use shared::settings::DatabaseSettings;
use shared::settings::Settings;
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnRequest;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::TraceLayer;
use tracing::Level;

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
    let app = endpoints::router().with_state(state).layer(
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
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6767").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
