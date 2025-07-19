import json
from datetime import datetime
from typing import Any, Dict, List, Tuple

import paho.mqtt.client as mqtt
from paho.mqtt.client import Client, ConnectFlags, MQTTMessage, Properties, ReasonCode
from sqlalchemy.exc import SQLAlchemyError

from mosquitto_consumer.config.enums import MosquittoSubscribeMethod, MosquittoTopics
from mosquitto_consumer.config.exceptions import MqttBrokerConnectionError, SqlClientError
from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.config.settings import settings
from mosquitto_consumer.database.models import PlantMoistureLog
from mosquitto_consumer.database.sql_client import sql_client
from mosquitto_consumer.utils.add_plants import add_plants

MQTT_CLIENT_NAME = 'plant-telemetry-moisture'

# pyrefly: ignore[bad-argument-type]
TOPIC_TO_ID: Dict[str, int] = MosquittoTopics.topic_to_id_dict()
SUBSCRIBED_TOPICS: List[Tuple[str, int]] = [
    (topic, MosquittoSubscribeMethod.EXACTLY_ONCE.value)
    for topic in TOPIC_TO_ID.keys()
]

def on_connect(  # noqa: D417
    client: mqtt.Client,
    userdata: Any,  # noqa: ANN401
    flags: ConnectFlags,
    reason_code: ReasonCode,
    properties: Properties
) -> None:
    """Determine what occurs once connected to MQTT broker.

    Args:
        All arguments are paho-mqtt specific.

    """
    if reason_code == 0:
        logger.info(f"Successfully connected to MQTT Broker with reason code: {reason_code}")
        if SUBSCRIBED_TOPICS:
            client.subscribe(SUBSCRIBED_TOPICS)
        else:
            logger.warning("No topics to subscribe to, check MosquittoTopics enum.")
    else:
        logger.error("Failed to connect to MQTT broker with error code %s", reason_code)


def on_message(  # noqa: D417
    client: Client,
    userdata: Any,  # noqa: ANN401
    msg: MQTTMessage
) -> None:
    """Determine what occurs when message has been received by MQTT broker.

    Args:
        All arguments are paho-mqtt specific.

    """
    topic: str = msg.topic
    payload: str = msg.payload.decode("utf-8")
    logger.info(f"Received message from topic '{topic}': {payload}")

    plant_id: int | None = TOPIC_TO_ID.get(topic)
    if not plant_id:
        logger.warning(f"Received message on an un-mapped topic: {topic}. Ignoring.")
        return

    try:
        # Check if keys from decoded message are valid
        data: Any = json.loads(payload)
        required_keys: list[str] = ["timestamp", "adc_value", "dry_value", "wet_value", "moisture_perc"]
        if not all(key in data for key in required_keys):
            logger.error(f"Data does not contain all required keys: {required_keys}")
            return
    except json.JSONDecodeError as _:
        logger.exception(f"Error decoding JSON from topic {topic}. Payload: {payload}")
        return

    try:
        # Attempt to create PlantMoistureLog model object
        timestamp: datetime = datetime.fromisoformat(data["timestamp"])
        moisture_log_entry: PlantMoistureLog = PlantMoistureLog(
            plant_id=plant_id,
            created_at=timestamp,
            adc_value=int(data["adc_value"]),
            dry_value=int(data["dry_value"]),
            wet_value=int(data["wet_value"]),
            moisture_perc=int(data["moisture_perc"])
        )
    except (ValueError, TypeError) as _:
        logger.exception("Error processing data to model. Message may be in incorrect format.")
        return

    try:
        with sql_client.get_session() as session, session.begin():
            session.add(moisture_log_entry)
            logger.info(f"Successfully inserted moisture log for plant_id {plant_id} at {timestamp}")
    except SqlClientError:
        logger.exception("Error while adding record to database.")
        return
    except SQLAlchemyError:
        logger.exception("Unexpected error while adding record to database.")
        return

def main() -> None:
    """Core logic of mosquitto consumer."""
    sql_client.create_schema()
    add_plants()

    mqtt_client: Client = mqtt.Client(
        callback_api_version=mqtt.CallbackAPIVersion.VERSION2,
        client_id = MQTT_CLIENT_NAME,
        clean_session=False # Allow messages to be retained if the broker is up but the client is down
    )
    mqtt_client.on_connect = on_connect # pyrefly: ignore[bad-argument-type]
    mqtt_client.on_message = on_message

    try:
        mqtt_client.username_pw_set(
            username=settings.MQTT_USERNAME,
            password=settings.MQTT_PASSWORD.get_secret_value()
        )
        mqtt_client.connect(settings.MQTT_BROKER_HOST, port=settings.MQTT_PORT, keepalive=60)
    except (ConnectionRefusedError, OSError, TypeError) as exception:
        logger.exception(f"Error connecting to {settings.MQTT_BROKER_HOST} on port {settings.MQTT_PORT}.")
        raise MqttBrokerConnectionError() from exception

    mqtt_client.loop_forever()

if __name__ == "__main__":
    main()
