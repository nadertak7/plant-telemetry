use std::sync::Arc;

use crate::AppState;
use crate::schema::home::Home;
use axum::Json;
use axum::{Router, routing::get};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(read))
}

/// Reaching the home path shows that the API service is up and running.
async fn read() -> Json<Home> {
    Json(Home {
        detail: "service is healthy".to_string(),
    })
}
