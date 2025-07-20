from __future__ import annotations

from contextlib import contextmanager
from typing import Dict, Generator, Optional, Sequence

from sqlalchemy import Engine, Row, create_engine, exc, text
from sqlalchemy.engine import Result
from sqlalchemy.exc import ResourceClosedError
from sqlalchemy.orm import Session, sessionmaker

from mosquitto_consumer.config.exceptions import (
    DatabaseConnectionError,
    DialectDriverError,
    SchemaCreationError,
    SqlQueryError,
)
from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.config.settings import settings
from mosquitto_consumer.database.models import Base


class SqlClient:
    """Handle SQLAlchemy database logic, including connections, and dynamic queries."""

    def __init__(
        self,
        dialect: str = 'postgresql',
        driver: str = 'psycopg2'
        ) -> None:
        """Instantiate SqlQueryClient class. Attempt a database connection in the constructor.

        Args:
            dialect (str, optional): The chosen SQL dialect for queries. Defaults to 'postgresql'.
            driver (str, optional): The python driver in the environment. Defaults to 'psycopg2'.

        Raises:
            DialectDriverError: Raise if chosen dialect and/or driver are incompatible.
            DatabaseConnectionError: Raise if there is an operational issue while connecting to a database
                or if there is an unexpected issue.

        """
        self.connection_url = (
            f"{dialect}+{driver}://" \
            f"{settings.POSTGRES_SUPER_USER}" \
            f":{settings.POSTGRES_SUPER_PASSWORD.get_secret_value()}" \
            f"@{settings.POSTGRES_DB_HOST}:5432" \
            f"/{settings.POSTGRES_DB}"
        )

        try:
            self.engine: Engine = create_engine(self.connection_url)
            # Factory to configure future sessions
            self._session = sessionmaker(bind=self.engine)
        except exc.NoSuchModuleError as exception:
            logger.exception(
                "Unsupported dialect and driver combination %s+%s. " \
                "Ensure that dialect is correct and driver is installed in environment.",
                dialect,
                driver
                )
            raise DialectDriverError() from exception
        except exc.OperationalError as exception:
            logger.exception(
                "Error while connecting to psql database %s, ensure credentials are correct.",
                settings.POSTGRES_DB_HOST
            )
            raise DatabaseConnectionError() from exception
        except exc.SQLAlchemyError as exception:
            logger.exception(
                "Unexpected error while connecting to psql database %s.",
                settings.POSTGRES_DB_HOST
            )
            raise DatabaseConnectionError() from exception

    @contextmanager
    def get_session(self) -> Generator[Session]:
        """Provide transactional database session via context manager.

        At the end of the `with` block, automatically handles any commits
            and rollbacks (if a failure is encountered).

        Yields:
            Generator[Session]: SQLAlchemy session object.

        Usage:
            with sql_client.get_session() as session, session.begin():
                    # Perform database operations

        """
        session: Session = self._session()
        try:
            yield session
        finally:
            session.close()

    def create_schema(self) -> None:
        """Create schema from models.py if it does not exist.

        Raises:
            SchemaCreationError: Raise if an error occurs when creating the schema from models.py

        """
        logger.info("Creating schema if it does not exist...")
        try:
            Base.metadata.create_all(bind=self.engine)
            logger.info("Schema created or retained successfully...")
        except exc.SQLAlchemyError as exception:
            logger.exception("Error while creating schema.")
            raise SchemaCreationError() from exception

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
        with self.get_session() as session:
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

sql_client: SqlClient = SqlClient()
