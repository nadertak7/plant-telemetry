class DialectDriverError(Exception):
    """Raise when SQLAlchemy dialect or driver is not found in environment or invalid."""

    pass

class DatabaseConnectionError(Exception):
    """Raise when SQLAlchemy raises an OperationalError when connecting to database."""

    pass

class SqlQueryError(Exception):
    """Raise when there is an error while executing a query via SQLAlchemy."""

    def __init__(self, query: str) -> None:
        """Generate the message and call base class constructor.

        Args:
            query (str): The query that raised the error.

        """
        self.query = query
        super().__init__(f"Error while executing query: {self.query}")
