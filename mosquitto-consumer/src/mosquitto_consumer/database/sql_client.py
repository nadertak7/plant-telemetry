from __future__ import annotations

import traceback
from types import TracebackType
from typing import Dict, Optional, Sequence, Type

from sqlalchemy import Engine, Row, create_engine, exc, text
from sqlalchemy.dialects import registry
from sqlalchemy.engine import Result
from sqlalchemy.exc import ResourceClosedError
from sqlalchemy.orm import Session, sessionmaker

from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.config.settings import settings
from mosquitto_consumer.database.models import Base
from mosquitto_consumer.utils.exceptions import DatabaseConnectionError, DialectDriverError, SqlQueryError


class SqlClient:
    """Handle SQLAlchemy database logic, including connections, and dynamic queries."""

    def __init__(
        self,
        dialect: str = 'postgresql',
        driver: str = 'psycopg2'
        ) -> None:
        """Instantiate SqlClient class. Attempt a database connection in the constructor.

        Args:
            dialect (str, optional): The chosen SQL dialect for queries. Defaults to 'postgresql'.
            driver (str, optional): The python driver in the environment. Defaults to 'psycopg2'.

        Raises:
            DialectDriverError: Raise if chosen dialect and/or driver are incompatible.
            DatabaseConnectionError: Raise if there is an operational issue while connecting to a database.
            RuntimeError: Raise if there is an unexpected issue while connecting to a database.

        """
        # Validate dialect and driver args
        dialect_driver: str = f"{dialect}+{driver}"
        try:
            registry.load(dialect_driver)
        except exc.NoSuchModuleError as exception:
            logger.exception(
                f"Unsupported dialect and driver combination {dialect_driver} " \
                "Ensure that dialect is correct and driver is installed in environment.")
            raise DialectDriverError() from exception

        self.connection_url = (
            f"{dialect_driver}://" \
            f"{settings.POSTGRES_SUPER_USER}" \
            f":{settings.POSTGRES_SUPER_PASSWORD.get_secret_value()}" \
            f"@{settings.HOST_IP}:5432" \
            f"/{settings.POSTGRES_DB}"
        )

        try:
            self.engine: Engine = create_engine(self.connection_url)
            self.session = sessionmaker(bind=self.engine)
        except exc.OperationalError as exception:
            logger.exception(
                "Error while connecting to psql database %s, ensure credentials are correct.",
                settings.HOST_IP
            )
            raise DatabaseConnectionError() from exception
        except exc.SQLAlchemyError as exception:
            logger.exception(
                "Unexpected error while connecting to psql database %s.",
                settings.HOST_IP
            )
            raise RuntimeError() from exception

    def __enter__(self) -> SqlClient:
        """Enter context manager. Use `with SqlClient():`."""
        return self

    def __exit__(
            self,
            exc_type: Optional[Type[BaseException]],
            exc_value: Optional[BaseException],
            exc_traceback: Optional[TracebackType]
        ) -> Optional[bool]:
        """Exit the context manager and dispose of the engine."""
        logger.info("Disposing of database engine connection pool...")
        self.engine.dispose()
        if exc_type:
            print("An error occurred:")
            traceback.print_exception(exc_type, exc_value, exc_traceback)
        return False

    def create_schema(self) -> None:
        """Create schema from models.py if it does not exist.

        Raises:
            RuntimeError: Raise if an error occurs when creating the schema in models.py

        """
        logger.info("Creating schema if it does not exist...")
        try:
            Base.metadata.create_all(bind=self.engine)
            logger.info("Schema created...")
        except exc.SQLAlchemyError as exception:
            logger.exception("Error while creating schema.")
            raise RuntimeError() from exception

    def execute_sql(
        self, query: str,
        params: Optional[Dict[str, str]] = None
    ) -> Optional[Sequence[Row]]:
        """Add connection to pool and run SQL query before returning connection.

        Args:
            query (str): Query string. Should be in same syntax as dialect specified when
                instantiating.
            params (Optional[Dict[str, str]], optional): Parameters to insert into query. Defaults to None.

        Raises:
            SqlQueryError: Raise if SQLAlchemy encounters an error during query execution.

        Returns:
            Optional[Sequence[Row]]: Data returned by query (if any).

        """
        session: Session = self.session()
        try:
            # Add connection to pool. Commits if code within executes successfully,
            # and rolls back if not.
            with session.begin():
                result: Result = session.execute(text(query), params or {})
                try:
                    return result.all()
                except ResourceClosedError:
                    # Raised when .all() is called on an empty result (for inserts, updates, deletes etx.)
                    return None
        except exc.SQLAlchemyError as exception:
            logger.exception("Error executing query. Transaction rolled back.")
            raise SqlQueryError(query) from exception
        finally:
            session.close() # Return connection to pool
