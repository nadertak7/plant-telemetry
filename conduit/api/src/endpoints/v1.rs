use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod home;
pub mod sensor;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().merge(home::router()).merge(sensor::router())
}
