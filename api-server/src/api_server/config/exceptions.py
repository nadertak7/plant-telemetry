from sqlalchemy import URL


class SqlAlchemyOperationError(Exception):
    """Inherited by all exceptions raised by SQLAlchemy client."""

    pass

class ConnectionUrlValidationError(SqlAlchemyOperationError):
    """Raise when there is a failure to validate the specified connection URL arguments."""

    def __init__(self) -> None:
        """Create instance of ConnectionUrlValidationError class."""
        super().__init__("Error while validating the specified connection URL arguments.")

class DialectDriverError(SqlAlchemyOperationError):
    """Raise when SQLAlchemy does not find the specified dialect or driver."""

    def __init__(self, driver_name: str) -> None:
        """Create instance of DialectDriverError class."""
        super().__init__(
            f"Unsupported dialect and driver combination {driver_name}" \
            "Ensure that dialect is correct and driver is installed in environment."
        )

class DatabaseConnectionError(SqlAlchemyOperationError):
    """Raise when SQLAlchemy encounters an error when connecting to specified database."""

    def __init__(self, connection_url: URL) -> None:
        """Create instance of DatabaseConnectionError class."""
        super().__init__(
            f"Error while connection with connection string {connection_url}, ensure credentials are correct"
        )

class SchemaReflectionError(SqlAlchemyOperationError):
    """Raise when SQLAlchemy encounters an error when reflecting an existing model."""

    def __init__(self, connection_url: URL) -> None:
        """Create instance of SchemaReflectionError class."""
        super().__init__(
            f"Error while reflecting schema in connection string {connection_url}."
        )

class SessionFactoryCreationError(SqlAlchemyOperationError):
    """Raise when there is an error while creating a session factory."""

    def __init__(self, connection_url: URL) -> None:
        """Create instance of SchemaReflectionError class."""
        super().__init__(
            f"Error while creating session factory for connection {connection_url}."
        )
