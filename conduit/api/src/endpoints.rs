use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod v1;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/v1", v1::router())
}
