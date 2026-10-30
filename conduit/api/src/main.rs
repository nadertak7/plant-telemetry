use shared::logger;

pub mod endpoints;
pub mod schema;

#[tokio::main]
async fn main() {
    logger::initialise();

    let app = endpoints::router();
    let listener = tokio::net::TcpListener::bind("localhost:6767")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
