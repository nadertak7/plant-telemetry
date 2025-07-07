from pydantic import SecretStr
from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """Take credentials from environment."""

    # Postgres settings
    POSTGRES_DB_HOST: str
    POSTGRES_SUPER_USER: str
    POSTGRES_SUPER_PASSWORD: SecretStr
    POSTGRES_DB: str

    # MQTT settings
    MQTT_BROKER_HOST: str
    MQTT_USERNAME: str
    MQTT_PASSWORD: SecretStr
    MQTT_PORT: int = 1883

# pyrefly: ignore[missing-argument]
settings: Settings = Settings()
