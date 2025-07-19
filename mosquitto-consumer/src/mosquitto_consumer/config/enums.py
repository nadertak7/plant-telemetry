from dataclasses import dataclass
from enum import Enum, StrEnum, auto
from typing import Dict, List


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

    @classmethod
    def topic_to_id_dict(cls) -> Dict[str, int]:
        """Return a dictionary mapping of topics to ids to allow reverse lookups."""
        return {member.value.topic: member.value.id for member in cls} # pyrefly: ignore[bad-argument-type]

    @classmethod
    def id_to_topic_insert_values(cls) -> List[Dict[str, str]]:
        """Return all the mosquitto topic ids and values in a form that can be inserted into postgres."""
        return [
            {
                "id": member.value.id,
                "plant_name": member.name.lower()
            }
            for member in cls # pyrefly: ignore[bad-argument-type]
        ]

class TableNames(StrEnum):
    """String enums for Postgres table names."""

    _value_: auto

    PLANTS = auto()
    PLANTS_MOISTURE_LOG = auto()

class MosquittoSubscribeMethod(Enum):
    """String enums to determine MQTT subscription method."""

    AT_MOST_ONCE = 0
    AT_LEAST_ONCE = 1
    EXACTLY_ONCE = 2
