use axum_test::TestServer;
use sqlx::PgPool;

pub(super) fn server(pool: PgPool) -> TestServer {
    TestServer::new(crate::app(pool))
}
