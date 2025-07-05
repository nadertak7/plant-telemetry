from dataclasses import dataclass
from enum import Enum, StrEnum, auto


@dataclass(frozen=True)
class TopicIdMapping:
    """Maps an incrementing ID to a topic."""

    id: int
    topic: str

class MosquittoTopics(Enum):
    """Enumerate MQTT topics."""

    _value_: TopicIdMapping

    SCARLET_STAR = TopicIdMapping(1, "plant-monitoring/living-room/scarlet-star-1/telemetry")
    # Add more topics here when ESP modules are connected to new/different plants

    @property
    def id(self) -> int:
        """Property for id. Use like `MosquittoTopics.SCARLET_STAR.id`."""
        return self.value.id

    @property
    def topic(self) -> str:
        """Property for topic. Use like `MosquittoTopics.SCARLET_STAR.topic`."""
        return self.value.topic

class TableNames(StrEnum):
    """String enums for Postgres table names."""

    _value_: auto

    PLANTS = auto()
    PLANTS_MOISTURE_LOG = auto()
