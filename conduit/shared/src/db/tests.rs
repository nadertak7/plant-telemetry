use crate::db::MIGRATOR;
use rstest::rstest;
use sqlx::{Error, PgPool, error::ErrorKind, postgres::PgQueryResult};

enum ExpectedResult {
    Success,
    ConstraintFailure,
}

fn validate_database_error<T: std::fmt::Debug>(
    result: Result<T, Error>,
    expected_error_kind: ErrorKind,
    expected_constraint: &str,
) {
    let error = result.expect_err("Expected error but insert query succeeded.");
    let database_error = error
        .as_database_error()
        .expect("Expected a database error, got a different error.");
    let actual_error_kind = database_error.kind();
    assert_eq!(
        actual_error_kind, expected_error_kind,
        "Expected an {expected_error_kind:?} error, but got {actual_error_kind:?}."
    );
    assert_eq!(database_error.constraint(), Some(expected_constraint));
}

#[rstest]
#[case::happy_path_1(20, 30, 40, ExpectedResult::Success)]
#[case::happy_path_2(0, 50, 100, ExpectedResult::Success)]
// Upper threshold out of bounds (greater than 100).
#[case::adc_perc_above_upper_bound(50, 70, 101, ExpectedResult::ConstraintFailure)]
// Lower threshold out of bounds (less than 0).
#[case::adc_perc_below_lower_bound(-1, 70, 90, ExpectedResult::ConstraintFailure)]
// Thresholds in incorrect order - highest to lowest.
#[case::adc_percs_in_incorrect_order_1(90, 70, 50, ExpectedResult::ConstraintFailure)]
// Thresholds in partially incorrect order - warning adc is lower than lower adc.
#[case::adc_percs_in_incorrect_order_2(40, 30, 50, ExpectedResult::ConstraintFailure)]
// All thresholds must be unique.
#[case::adc_percs_must_be_unique(40, 40, 50, ExpectedResult::ConstraintFailure)]
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
            let row = result.expect("Expected success but got an error.");
            assert_eq!(lower_threshold_perc, row.lower_threshold_perc);
            assert_eq!(warning_threshold_perc, row.warning_threshold_perc);
            assert_eq!(upper_threshold_perc, row.upper_threshold_perc);
        }
        ExpectedResult::ConstraintFailure => {
            validate_database_error(
                result,
                ErrorKind::CheckViolation,
                "ck_plant_lower_warning_upper_threshold_perc",
            );
        }
    }
}

#[rstest]
#[case::happy_path(500, 300, ExpectedResult::Success)]
#[case::wet_adc_greater_than_dry_adc(300, 500, ExpectedResult::ConstraintFailure)]
#[case::wet_adc_equal_to_dry_adc(300, 300, ExpectedResult::ConstraintFailure)]
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
            let row = result.expect("Expected success but got an error.");
            assert_eq!(dry_adc, row.dry_adc);
            assert_eq!(wet_adc, row.wet_adc);
        }
        ExpectedResult::ConstraintFailure => {
            validate_database_error(
                result,
                ErrorKind::CheckViolation,
                "ck_sensor_dry_adc_wet_adc",
            );
        }
    }
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn test_active_sensor_topic_must_be_unique(pool: PgPool) {
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

    validate_database_error(result, ErrorKind::UniqueViolation, "ux_sensor_topic");
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn test_active_plant_display_name_must_be_unique(pool: PgPool) {
    let result = sqlx::query!(r#"
        INSERT INTO
            plant
            (id, display_name, lower_threshold_perc, warning_threshold_perc, upper_threshold_perc, archived_at)
        VALUES
            (1, 'plant', 30, 40, 50, NULL),
            (2, 'plant', 30, 40, 50, '1970-01-01')
        "#).execute(&pool).await;

    let query_result =
        result.expect("An active and archived plant with the same display name should coexist.");
    assert_eq!(query_result.rows_affected(), 2);

    let result = sqlx::query(r#"
        INSERT INTO
            plant
            (id, display_name, lower_threshold_perc, warning_threshold_perc, upper_threshold_perc, archived_at)
        VALUES
            (3, 'plant', 30, 40, 50, NULL)
        "#).execute(&pool).await;

    validate_database_error(result, ErrorKind::UniqueViolation, "ux_plant_display_name");
}

#[sqlx::test(migrator = "MIGRATOR", fixtures("plant", "sensor"))]
async fn test_plant_telemetry_sensor_recorded_at_must_be_unique(pool: PgPool) {
    let result = sqlx::query!(
        r#"
        INSERT INTO
            plant_telemetry
            (id, plant_id, sensor_id, adc, moisture_perc, recorded_at)
        VALUES
            (1, 1, 1, 400, 50, '1970-01-01'),
            (2, 1, 1, 400, 50, '1970-01-01')
        "#
    )
    .execute(&pool)
    .await;

    validate_database_error(
        result,
        ErrorKind::UniqueViolation,
        "ux_plant_telemetry_sensor_id_recorded_at",
    );
}

#[rstest]
#[case::happy_path(50.0, ExpectedResult::Success)]
#[case::moisture_perc_less_than_lower_bound(-1.0, ExpectedResult::ConstraintFailure)]
#[case::moisture_perc_higher_than_upper_bound(101.0, ExpectedResult::ConstraintFailure)]
#[sqlx::test(migrator = "MIGRATOR", fixtures("plant", "sensor"))]
async fn test_telemetry_moisture_perc_must_be_in_bounds(
    #[case] moisture_perc: f64,
    #[case] expected_result: ExpectedResult,
    #[ignore] pool: PgPool,
) {
    let result = sqlx::query!(
        r#"
        INSERT INTO
            plant_telemetry
            (id, plant_id, sensor_id, adc, moisture_perc, recorded_at)
        VALUES
            (1, 1, 1, 400, $1, '1970-01-01')
        RETURNING
            moisture_perc
        "#,
        moisture_perc
    )
    .fetch_one(&pool)
    .await;

    match expected_result {
        ExpectedResult::Success => {
            let row = result.expect("Expected success but got an error.");
            assert_eq!(moisture_perc, row.moisture_perc);
        }
        ExpectedResult::ConstraintFailure => {
            validate_database_error(
                result,
                ErrorKind::CheckViolation,
                "ck_plant_telemetry_moisture_perc",
            );
        }
    }
}
