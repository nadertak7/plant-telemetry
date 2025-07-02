from enum import StrEnum


class MosquittoTopics(StrEnum):
    """String enums for MQTT topics to subscribe to."""

    SCARLET_STAR = "plant-monitoring/living-room/scarlet-star-1/telemetry"
    # More TBD
