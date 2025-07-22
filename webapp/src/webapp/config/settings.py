from pydantic import SecretStr
from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """Take credentials from environment."""

    # Postgres settings
    POSTGRES_DB_HOST: str
    POSTGRES_SUPER_USER: str
    POSTGRES_SUPER_PASSWORD: SecretStr
    POSTGRES_DB: str

# pyrefly: ignore[missing-argument]
settings: Settings = Settings()
