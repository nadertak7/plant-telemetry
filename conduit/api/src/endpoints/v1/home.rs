use crate::schema::home::Home;
use axum::Json;
use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(read))
}

async fn read() -> Json<Home> {
    Json(Home {
        detail: "Service is Healthy".to_string(),
    })
}
