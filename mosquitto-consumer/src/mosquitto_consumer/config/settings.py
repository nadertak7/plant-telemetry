from pydantic import SecretStr
from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """Take credentials from environment."""

    host_ip: str

    # Postgres settings
    postgres_super_user: str
    postgres_super_password: SecretStr
    postgres_db: str

    # MQTT settings
    mqtt_username: str
    mqtt_password: SecretStr

settings: Settings = Settings()
