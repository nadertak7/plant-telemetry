use crate::db::MIGRATOR;
use rstest::rstest;
use sqlx::{PgPool, postgres::PgQueryResult};

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
#[case::adc_percs_must_be_unique(40, 40, 50, ExpectedResult::ConstraintFailure { constraint: "ck_plant_lower_warning_upper_threshold_perc"})]
#[sqlx::test(migrator = "MIGRATOR")]
async fn test_lower_warning_upper_adc_thresholds_must_be_sequential(
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
            let error = result.expect_err("Expected error, got success.");
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

#[rstest]
#[case::happy_path(500, 300, ExpectedResult::Success)]
#[case::wet_adc_greater_than_dry_adc(300, 500, ExpectedResult::ConstraintFailure { constraint: "ck_sensor_dry_adc_wet_adc" })]
#[case::wet_adc_equal_to_dry_adc(300, 300, ExpectedResult::ConstraintFailure { constraint: "ck_sensor_dry_adc_wet_adc" })]
#[sqlx::test(migrator = "MIGRATOR")]
async fn test_dry_adc_must_be_greater_than_wet_adc(
    #[case] dry_adc: i32,
    #[case] wet_adc: i32,
    #[case] expected_result: ExpectedResult,
    #[ignore] pool: PgPool,
) {
    let result = sqlx::query!(
        r#"
        INSERT INTO
            sensor
            (id, topic, plant_id, dry_adc, wet_adc)
        VALUES
            (1, 'sensor/1', NULL, $1, $2)
        RETURNING
            dry_adc, wet_adc
    "#,
        dry_adc,
        wet_adc
    )
    .fetch_one(&pool)
    .await;

    match expected_result {
        ExpectedResult::Success => {
            let row = result.expect("Expected success, got error.");
            assert_eq!(dry_adc, row.dry_adc);
            assert_eq!(wet_adc, row.wet_adc);
        }
        ExpectedResult::ConstraintFailure { constraint } => {
            let error = result.expect_err("Expected error, got success.");
            let database_error = error
                .as_database_error()
                .expect("Expected a database error, got a different error.");
            assert!(
                database_error.is_check_violation(),
                "Expected a check violation, got a different error."
            );
            assert_eq!(database_error.constraint(), Some(constraint));
        }
    }
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn test_sensor_and_topic_unique_constraint(pool: PgPool) {
    let result = sqlx::query!(
        r#"
        INSERT INTO
            sensor
            (id, topic, plant_id, dry_adc, wet_adc, archived_at)
        VALUES
            (1, 'sensor_1', NULL, 500, 300, NULL),
            (2, 'sensor_1', NULL, 500, 300, '1970-01-01')
        "#
    )
    .execute(&pool)
    .await;

    let query_result: PgQueryResult =
        result.expect("An active and archived sensor with the same topic should coexist.");
    assert_eq!(query_result.rows_affected(), 2);

    let result = sqlx::query!(
        r#"
        INSERT INTO
            sensor
            (id, topic, plant_id, dry_adc, wet_adc, archived_at)
        VALUES
            (3, 'sensor_1', NULL, 500, 300, NULL)
        "#
    )
    .execute(&pool)
    .await;

    let error = result.expect_err("Two active sensors with the same topic cannot coexist.");
    let database_error = error
        .as_database_error()
        .expect("Expected database error, got a different error.");
    assert!(
        database_error.is_unique_violation(),
        "Expected a unique violation, got a different error."
    );
    assert_eq!(database_error.constraint(), Some("ux_sensor_topic"));
}
