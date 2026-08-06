use crate::schema::message::SensorPayload;
use rumqttc::Publish;

fn parse_payload(message: &Publish) -> Option<SensorPayload> {
    match serde_json::from_slice(&message.payload) {
        Ok(payload) => Some(payload),
        Err(e) => {
            tracing::warn!(
                topic = message.topic,
                payload = ?message.payload,
                error = %e,
                "Error parsing payload."
            );
            None
        }
    }
}

pub fn handle_message(message: &Publish) {
    let Some(sensor_payload) = parse_payload(message) else {
        return;
    };
    tracing::info!(topic=message.topic, payload=?sensor_payload, "Parsed sensor payload.")
}
