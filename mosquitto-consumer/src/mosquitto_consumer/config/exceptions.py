# SqlClient errors
class SqlClientError(Exception):
    """Inhert by all exceptions raised by SqlClient.

    Use to catch any database error when using the SqlClient, especially as a
        context manager.

    Usage:
        try:
            with sql_client.get_session() as session, session.begin():
                client.create_schema()
                client.execute_sql("select 0 from table")
        except SqlClientError as exception:
            logging.exception("Database operation failure.")
            raise SqlClientError() from exception
    """

    pass

class DialectDriverError(SqlClientError):
    """Raise when SQLAlchemy dialect or driver is not found in environment or invalid."""

    pass

class DatabaseConnectionError(SqlClientError):
    """Raise when SQLAlchemy encounters an error when connecting to database."""

    pass

class SchemaCreationError(SqlClientError):
    """Raise when SQLAlchemy encounters an error while creating from a specified model."""

    pass

class SqlQueryError(SqlClientError):
    """Raise when SQLAlchemy encounters an error while executing a query."""

    def __init__(self, query: str) -> None:
        """Generate the message and call base class constructor.

        Args:
            query (str): The query that raised the error.

        """
        self.query = query
        super().__init__(f"Error while executing query: {self.query}")

# Mqtt consumer errors
class MqttConsumerError(Exception):
    """Inherit by all exceptions raised by mqtt consumer."""

    pass

class JsonPayloadDecodeError(MqttConsumerError):
    """Raise when an error is encountered while processing the JSON payload from the MQTT broker."""

    pass

class ModelObjectProcessingError(MqttConsumerError):
    """Raise when an error is encountered while creating a SQLAlchemy model object with data."""

    pass

class MqttBrokerConnectionError(MqttConsumerError):
    """Raise when mosquitto encounters an error while connecting to the specified client."""

    pass
