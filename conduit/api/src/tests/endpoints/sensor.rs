use crate::tests::endpoints::common;
use axum::http::StatusCode;
use axum_test::expect_json::{self, iso_date_time};
use rstest::rstest;
use serde::Deserialize;
use serde_json::{Value, json};
use shared::db::MIGRATOR;
use sqlx::PgPool;

/// Full sensor response to ensure response returns the correct field values.
fn sensor_id_1_expected_json() -> Value {
    json!(
        {
            "topic": "sensor/active_with_plant",
            "dry_adc": 1000,
            "wet_adc": 0,
            "created_at": expect_json::iso_date_time(),
            "updated_at": expect_json::iso_date_time(),
            "archived_at": null,
            "last_active_at": null,
            "plant": {
                "id": 1,
                "display_name": "Active Plant"
            },
        }
    )
}

/// An active sensor with an archived plant should have its plant omitted from its reponse.
fn no_plant_expected_json() -> Value {
    json!(
        {
            "plant": null
        }
    )
}

/// An archived sensor will appear in the response with an archived date populated.
fn archived_expected_json() -> Value {
    json!(
        {
            "archived_at": iso_date_time(),
        }
    )
}

/// Assert sensor view endpoint responses against expected outputs.
#[rstest]
#[case::active_with_plant(1, sensor_id_1_expected_json())]
#[case::active_with_archived_plant(2, no_plant_expected_json())]
#[case::active_without_plant(3, no_plant_expected_json())]
#[case::archived(4, archived_expected_json())]
#[sqlx::test(migrator = "MIGRATOR", fixtures("plant", "sensor"))]
async fn test_view_sensor(#[case] id: u16, #[case] expected_json: Value, #[ignore] pool: PgPool) {
    let response = common::server(pool)
        .get(&format!("/v1/sensor/{}", id))
        .await;

    response
        .assert_status_ok()
        .assert_json_contains(&json!({"id": id}))
        .assert_json_contains(&expected_json);
}

/// Deserialisable data needed per sensor in [`test_list_sensor`]
#[derive(Deserialize)]
struct TestSensorListItem {
    id: i32,
}

/// Assert sensor list endpoint responses against expected outputs.
#[rstest]
#[case::all(json!({}), vec![1, 2, 3, 4])]
#[case::active(json!({"archived": false}), vec![1, 2, 3])]
#[case::archived(json!({"archived": true}), vec![4])]
#[case::topic_full_match(json!({"topic": "sensor/active_without_plant"}), vec![3])]
#[case::topic_partial_match(json!({"topic": "sor/act"}), vec![1, 2, 3])]
#[case::topic_gibberish(json!({"topic": "abcdefg"}), vec![])]
#[case::combined_filters(json!({"topic": "archived", "archived": false}), vec![2])]
#[sqlx::test(migrator = "MIGRATOR", fixtures("plant", "sensor"))]
async fn test_list_sensor(
    #[case] params: Value,
    #[case] mut expected_ids: Vec<i32>,
    #[ignore] pool: PgPool,
) {
    let response = common::server(pool)
        .get("/v1/sensor")
        .add_query_params(&params)
        .await;

    response.assert_status_ok();
    let json: Vec<TestSensorListItem> = response.json();
    let mut ids: Vec<i32> = json.iter().map(|item| item.id).collect();
    ids.sort();
    expected_ids.sort();
    assert_eq!(expected_ids, ids);
}

/// Assert sensor create endpoint payloads against expected outputs.
#[rstest]
#[case::happy_path_with_plant(json!({ "topic": "test", "dry_adc": 500, "wet_adc": 100, "plant": { "id": 1 } }), StatusCode::CREATED)]
#[case::happy_path_without_plant(json!({ "topic": "test", "dry_adc": 500, "wet_adc": 100 }), StatusCode::CREATED)]
#[case::invalid_adcs(json!({ "topic": "test", "dry_adc": 100, "wet_adc": 500 }), StatusCode::UNPROCESSABLE_ENTITY)]
#[case::missing_key(json!({ "topic": "test", "dry_adc": 500 }), StatusCode::UNPROCESSABLE_ENTITY)]
#[case::wrong_datatype(json!({ "topic": 1, "dry_adc": 500, "wet_adc": 100 }), StatusCode::UNPROCESSABLE_ENTITY)]
#[case::non_existent_plant(json!({ "topic": "test", "dry_adc": 500, "wet_adc": 100, "plant": { "id": 9999 } }), StatusCode::UNPROCESSABLE_ENTITY)]
#[sqlx::test(migrator = "MIGRATOR", fixtures("plant"))]
async fn test_create_sensor(
    #[case] body: Value,
    #[case] expected_status_code: StatusCode,
    #[ignore] pool: PgPool,
) {
    let response = common::server(pool).post("/v1/sensor").json(&body).await;

    response.assert_status(expected_status_code);
    if expected_status_code == StatusCode::CREATED {
        response.assert_json_contains(&body);
    }
}

/// Assert that a sensor with an already existing topic returns a conflict error.
#[sqlx::test(migrator = "MIGRATOR")]
async fn test_create_sensor_duplicate_topic(pool: PgPool) {
    let payload = json!({ "topic": "test", "dry_adc": 500, "wet_adc": 100 });
    let response = common::server(pool.clone())
        .post("/v1/sensor")
        .json(&payload)
        .await;
    response.assert_status(StatusCode::CREATED);

    let response = common::server(pool.clone())
        .post("/v1/sensor")
        .json(&payload)
        .await;
    response.assert_status(StatusCode::CONFLICT);
}
