class SqlAlchemyOperationError(Exception):
    """Inhert by all exceptions raised by sqlalchemy_operations.py.

    Usage:
        try:
            with Session.begin() as session:
                client.execute_sql("select 0 from table")
        except SqlAlchemyOperationError as exception:
            logging.exception("Database operation failure.")
            raise SqlAlchemyOperationError() from exception
    """

    pass

class DialectDriverError(SqlAlchemyOperationError):
    """Raise when SQLAlchemy dialect or driver is not found in environment or invalid."""

    pass

class DatabaseConnectionError(SqlAlchemyOperationError):
    """Raise when SQLAlchemy encounters an error when connecting to database."""

    pass

class SessionFactoryCreationError(SqlAlchemyOperationError):
    """Raise when SQLAlchemy encounters an error when creating a session factory."""

    pass
