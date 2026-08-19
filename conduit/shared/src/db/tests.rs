use rstest::rstest;
use sqlx::PgPool;
enum ExpectedResult {
    Success,
    ConstraintFailure { constraint: &'static str },
}

#[rstest]
#[case::happy_path_1(20, 30, 40, ExpectedResult::Success)]
#[case::happy_path_2(0, 50, 100, ExpectedResult::Success)]
// Upper threshold out of bounds (greater than 100).
#[case::adc_perc_above_upper_bound(50, 70, 101, ExpectedResult::ConstraintFailure { constraint: "ck_plant_lower_warning_upper_threshold_perc"})]
// Lower threshold out of bounds (less than 0).
#[case::adc_perc_below_lower_bound(-1, 70, 90, ExpectedResult::ConstraintFailure { constraint: "ck_plant_lower_warning_upper_threshold_perc"})]
// Thresholds in incorrect order - highest to lowest.
#[case::adc_percs_in_incorrect_order_1(90, 70, 50, ExpectedResult::ConstraintFailure { constraint: "ck_plant_lower_warning_upper_threshold_perc"})]
// Thresholds in partially incorrect order - warning adc is lower than lower adc.
#[case::adc_percs_in_incorrect_order_2(40, 30, 50, ExpectedResult::ConstraintFailure { constraint: "ck_plant_lower_warning_upper_threshold_perc"})]
// All thresholds must be unique.
#[case::adc_percs_in_incorrect_order_2(40, 40, 50, ExpectedResult::ConstraintFailure { constraint: "ck_plant_lower_warning_upper_threshold_perc"})]
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
            let row = result.expect("Expected success, got error.");
            assert_eq!(lower_threshold_perc, row.lower_threshold_perc);
            assert_eq!(warning_threshold_perc, row.warning_threshold_perc);
            assert_eq!(upper_threshold_perc, row.upper_threshold_perc);
        }
        ExpectedResult::ConstraintFailure { constraint } => {
            let error = result.expect_err("Expected failure, got success");
            let database_error = error
                .as_database_error()
                .expect("Expected a database error, got a different error.");
            assert!(
                database_error.is_check_violation(),
                "Expected a check violation, got {database_error}."
            );
            assert_eq!(database_error.constraint(), Some(constraint));
        }
    }
}
