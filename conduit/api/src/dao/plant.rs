use sqlx::PgPool;

/// Return true id a plant with a given id exists. Else false.
pub async fn check_plant_exists_by_id(id: i32, pool: &PgPool) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT
            EXISTS(
                SELECT
                    1
                FROM
                    plant
                WHERE
                    plant.id = $1
            ) "exists!"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(result)
}
