use axum::Router;

pub mod home;

pub fn router() -> Router {
    Router::new().merge(home::router())
}
