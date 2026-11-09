use shared::db::MIGRATOR;
use sqlx::PgPool;

use crate::tests::endpoints::common;

/// Assert that the home endpoint returns successfully.
#[sqlx::test(migrator = "MIGRATOR")]
fn test_view_home(pool: PgPool) -> () {
    let response = common::server(pool).get("/v1").await;

    response.assert_status_ok();
}
