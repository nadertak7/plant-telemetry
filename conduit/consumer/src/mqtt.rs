use anyhow::Context;
use rumqttc::{AsyncClient, EventLoop};
use shared::settings::{MqttSettings, Settings};

/// Create a client for sending MQTT requests and an event loop that drives communication with the
/// broker service.
pub fn get_client(mqtt_settings: &MqttSettings) -> (AsyncClient, EventLoop) {
    let connect_options = mqtt_settings.connect_options();
    AsyncClient::new(connect_options, mqtt_settings.request_queue_capacity)
}

/// Ask the broker to send sensor messages matching the configured topic filter.
pub async fn subscribe(client: &AsyncClient, mqtt_settings: &MqttSettings) -> anyhow::Result<()> {
    client
        .subscribe(
            mqtt_settings.subscribe_topic.to_string(),
            mqtt_settings.quality_of_service,
        )
        .await
        .context("Unable to subscribe to mqtt topic.")
}
