from enum import auto, Enum, StrEnum
from dataclasses import dataclass


class MosquittoTopics(Enum):
    """String enums for MQTT topics to subscribe to."""

    @dataclass(frozen=True)
    class TopicIdMapping:
        id: int
        topic: str

    SCARLET_STAR = TopicIdMapping(1, "plant-monitoring/living-room/scarlet-star-1/telemetry")
    # Add more topics here when ESP modules are connected to new/different plants

    @property
    def id(self):
        return self.value.id

    @property
    def topic(self):
        return self.value.topic

class TableNames(StrEnum):
    """String enums for Postgres table names."""

    PLANTS = auto()
    PLANTS_MOISTURE_LOG = auto()
