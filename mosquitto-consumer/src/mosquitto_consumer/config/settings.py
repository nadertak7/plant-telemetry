from pydantic import SecretStr
from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """Take credentials from environment."""

    HOST_IP: str

    # Postgres settings
    POSTGRES_SUPER_USER: str
    POSTGRES_SUPER_PASSWORD: SecretStr
    POSTGRES_DB: str

    # MQTT settings
    MQTT_USERNAME: str
    MQTT_PASSWORD: SecretStr

# pyrefly: ignore[missing-argument]
settings: Settings = Settings()
