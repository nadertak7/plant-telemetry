use chrono::{DateTime, Utc};
use rstest::rstest;

use super::*;

#[derive(Debug)]
enum ExpectedResult {
    Success {
        adc: i32,
        moisture_perc: f64,
        recorded_at: i64,
    },
    PayloadParseError,
    SensorNotRegistered,
    SensorArchived,
    PlantNotRegistered,
    PlantArchived,
    AdcNotInRange,
}

struct ActualTelemetryRecord {
    adc: i32,
    moisture_perc: f64,
    recorded_at: DateTime<Utc>,
    last_active_at: Option<DateTime<Utc>>,
}

#[rstest]
#[case::happy_path_1("sensor/1", r#"{"adc": 300, "timestamp": 1}"#, ExpectedResult::Success{ adc: 300, moisture_perc: 30.0, recorded_at: 1 })]
#[case::happy_path_2("sensor/1", r#"{"adc": 700, "timestamp": 1000}"#, ExpectedResult::Success{ adc: 700, moisture_perc: 70.0, recorded_at: 1000 })]
#[case::unparseable_message_key_adc(
    "sensor/1",
    r#"{"adc1": 300, "timestamp": 1}"#, // adc1 is not an expected key.
    ExpectedResult::PayloadParseError
)]
#[case::unparseable_message_key_timestamp(
    "sensor/1",
    r#"{"adc": 300, "timestamp1": 1}"#, // timestamp1 is not an expected key.
    ExpectedResult::PayloadParseError
)]
#[case::unparseable_data_adc(
    "sensor/1",
    r#"{"adc": "300", "timestamp": 1}"#, // adc cannot be a string.
    ExpectedResult::PayloadParseError
)]
#[case::unparseable_data_timestamp(
    "sensor/1",
    r#"{"adc": 300, "timestamp": "1"}"#, // timestamp cannot be a string.
    ExpectedResult::PayloadParseError
)]
#[case::unparseable_message_format(
    "sensor/1",
    r#"{adc 700, "timestamp": 1}"#, // Invalid json, no speech marks around adc.
    ExpectedResult::PayloadParseError
)]
#[case::sensor_unregistered(
    "sensor/unregistered", // sensor/unregistered does not exist as a topic the sensor table.
    r#"{"adc": 300, "timestamp": 1}"#,
    ExpectedResult::SensorNotRegistered
)]
#[case::sensor_archived(
    "sensor/archived", // sensor/archived has an archived_at date.
    r#"{"adc": 300, "timestamp": 1}"#,
    ExpectedResult::SensorArchived
)]
#[case::plant_unregistered(
    "sensor/plant_unregistered", // sensor/plant_unregistered has a plant id of NULL.
    r#"{"adc": 300, "timestamp": 1}"#,
    ExpectedResult::PlantNotRegistered
)]
#[case::plant_archived(
    "sensor/plant_archived", // the plant linked to sensor/plant_archived has an archived_at date.
    r#"{"adc": 300, "timestamp": 1}"#,
    ExpectedResult::PlantArchived
)]
#[case::adc_out_of_bounds_lower(
    "sensor/1",
    r#"{"adc": -1, "timestamp": 1}"#, // adc cannot be below the sensor's wet adc (0).
    ExpectedResult::AdcNotInRange
)]
#[case::adc_out_of_bounds_upper(
    "sensor/1",
    r#"{"adc": 1001, "timestamp": 1}"#, // adc cannot be higher the sensor's dry adc (1000).
    ExpectedResult::AdcNotInRange
)]
#[sqlx::test(migrations = "../migrations", fixtures("plant", "sensor"))]
async fn test_try_handle_message(
    #[case] topic: &str,
    #[case] payload: &str,
    #[case] expected_result: ExpectedResult,
    #[ignore] pool: PgPool,
) {
    let result = try_handle_message(topic, payload.as_bytes(), &pool).await;

    match expected_result {
        ExpectedResult::Success {
            adc: expected_adc,
            moisture_perc: expected_moisture_perc,
            recorded_at: expected_recorded_at,
        } => {
            if let Err(e) = result {
                panic!("Expected success, got error: {e}")
            }
            let record = sqlx::query_as!(
                ActualTelemetryRecord,
                r#"
                SELECT
                    plant_telemetry.adc,
                    plant_telemetry.moisture_perc,
                    plant_telemetry.recorded_at,
                    sensor.last_active_at
                FROM
                    plant_telemetry
                INNER JOIN
                    sensor
                    ON plant_telemetry.sensor_id = sensor.id
                WHERE
                    sensor.topic = $1
                "#,
                topic
            )
            .fetch_optional(&pool)
            .await
            .expect("Error querying database in test.")
            .expect("Telemetry record not found.");
            assert_eq!(expected_adc, record.adc);
            assert_eq!(expected_moisture_perc, record.moisture_perc);
            assert_eq!(expected_recorded_at, record.recorded_at.timestamp());
            assert!(record.last_active_at.is_some());
        }
        ExpectedResult::PayloadParseError => {
            assert!(
                matches!(result, Err(HandlerError::PayloadParseError(_))),
                "Expected PayloadParseError, got {result:?}."
            );
        }
        ExpectedResult::SensorNotRegistered => {
            assert!(
                matches!(result, Err(HandlerError::SensorNotRegistered)),
                "Expected SensorNotRegistered, got {result:?}."
            )
        }
        ExpectedResult::SensorArchived => {
            assert!(
                matches!(result, Err(HandlerError::SensorArchived)),
                "Expected SensorArchived, got {result:?}."
            )
        }
        ExpectedResult::PlantNotRegistered => {
            assert!(
                matches!(result, Err(HandlerError::PlantNotRegistered)),
                "Expected PlantNotRegistered got {result:?}."
            )
        }
        ExpectedResult::PlantArchived => {
            assert!(
                matches!(result, Err(HandlerError::PlantArchived)),
                "Expected PlantArchived got {result:?}."
            )
        }
        ExpectedResult::AdcNotInRange => {
            assert!(
                matches!(result, Err(HandlerError::AdcNotInRange)),
                "Expected AdcNotInRange, got {result:?}."
            )
        }
    }
}
