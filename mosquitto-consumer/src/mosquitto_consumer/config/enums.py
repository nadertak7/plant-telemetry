from enum import Enum


class MosquittoSubscribeMethod(Enum):
    """String enums to determine MQTT subscription method."""

    AT_MOST_ONCE = 0
    AT_LEAST_ONCE = 1
    EXACTLY_ONCE = 2
