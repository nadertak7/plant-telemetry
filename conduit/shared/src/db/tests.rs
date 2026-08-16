use rstest::rstest;
use sqlx::PgPool;
enum ExpectedResult {
    Success,
    Failure,
}

#[rstest]
#[case::happy_path_1(20, 30, 40, ExpectedResult::Success)]
#[case::happy_path_2(50, 70, 90, ExpectedResult::Success)]
#[case::happy_path_2(50, 70, 200, ExpectedResult::Failure)] // Upper threshold out of bounds.
#[sqlx::test(migrations = "../migrations")]
async fn test_ck_plant_lower_warning_upper_threshold_perc(
    #[case] lower_threshold_perc: i16,
    #[case] warning_threshold_perc: i16,
    #[case] upper_threshold_perc: i16,
    #[case] expected_result: ExpectedResult,
    #[ignore] pool: PgPool,
) {
    let result = sqlx::query!(
        r#"
        INSERT INTO
            plant
            (id, display_name, lower_threshold_perc, warning_threshold_perc, upper_threshold_perc)
        VALUES
            (1, 'Plant', $1, $2, $3)
        RETURNING
            lower_threshold_perc, warning_threshold_perc, upper_threshold_perc
    "#,
        lower_threshold_perc,
        warning_threshold_perc,
        upper_threshold_perc
    )
    .fetch_one(&pool)
    .await;

    match expected_result {
        ExpectedResult::Success => {
            result.expect("Expected success, got error.");
        }
        ExpectedResult::Failure => {
            let error = result.expect_err("Expected failure, got success");
            let database_error = error
                .as_database_error()
                .expect("Expected a database error, got a different error.");
            assert!(
                database_error.is_check_violation(),
                "Expected a check violation, got {database_error}."
            );
            assert_eq!(
                database_error.constraint(),
                Some("ck_plant_lower_warning_upper_threshold_perc")
            );
        }
    }
}
