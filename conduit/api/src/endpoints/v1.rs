use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod home;
mod sensor;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().merge(home::router()).merge(sensor::router())
}
