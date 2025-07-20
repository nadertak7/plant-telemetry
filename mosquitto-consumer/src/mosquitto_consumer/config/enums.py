from enum import Enum, StrEnum, auto


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
