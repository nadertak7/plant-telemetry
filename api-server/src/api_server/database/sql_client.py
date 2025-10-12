from dataclasses import asdict, dataclass
from typing import AsyncGenerator

from sqlalchemy.engine import URL
from sqlalchemy.exc import NoSuchModuleError, OperationalError, SQLAlchemyError
from sqlalchemy.ext.asyncio import AsyncEngine, AsyncSession, async_sessionmaker, create_async_engine

from api_server.config.exceptions import (
    ConnectionUrlValidationError,
    DatabaseConnectionError,
    DialectDriverError,
    SchemaCreationError,
    SessionFactoryCreationError,
    SessionRetrievalError,
    SqlAlchemyOperationError,
)
from api_server.config.logs import logger
from api_server.database.models import Base


@dataclass
class _ConnectionUrlArgs:
    """Validate arguments passed into the SQLAlchemy URL object."""

    drivername: str
    username: str
    password: str
    host: str
    port: int
    database: str


class SqlClient:
    """Handle SQLAlchemy database logic, including connections, and dynamic queries."""

    def __init__(
        self,
        username: str,
        password: str,
        host: str,
        port: int,
        database: str,
        dialect: str = 'postgresql',
        driver: str = 'asyncpg',
        handle_stale_connections: bool = True
        ) -> None:
        """Instantiate SqlClient class. Attempt a database connection in the constructor.

        Args:
            username (str): The database connection username.
            password (str): The database connection password.
            host (str): The host of the database.
            port (int): The port of the database.
            database (str): The name of the schema to connect to.
            dialect (str, optional): The chosen SQL dialect for queries. Defaults to 'postgresql'.
            driver (str, optional): The python driver in the environment. Defaults to 'asyncpg'.
            handle_stale_connections (bool, optional): Choose whether to ping the connection pool for stale connections.
              defaults to true.

        Raises:
            DialectDriverError: Raise if chosen dialect and/or driver are incompatible.
            DatabaseConnectionError: Raise if there is an operational issue while connecting to a database
                or if there is an unexpected issue.

        """
        try:
            self.connection_url_kwargs = _ConnectionUrlArgs(
                drivername=f"{dialect}+{driver}",
                username=username,
                password=password,
                host=host,
                port=port,
                database=database
            )
        except TypeError as exception:
            logger.exception("Failed to verify connection url arguments.")
            raise ConnectionUrlValidationError() from exception

        self.handle_stale_connections = handle_stale_connections
        self.connection_url = self._create_connection_url()
        self.engine = self._create_engine(handle_stale_connections=self.handle_stale_connections)
        self.sessionmaker = self._create_sessionmaker(expire_on_commit=False)

    def _create_connection_url(self) -> URL:
        """Combine arguments passed into the constructor into a SQLAlchemy connection URL string.

        Returns:
        URL: Correctly encoded connection URL used to create SQLAlchemy engine.

        """
        connection_url_object: URL = URL.create(**asdict(self.connection_url_kwargs))

        return connection_url_object

    def _create_engine(self, handle_stale_connections: bool = True) -> AsyncEngine:
        try:
            engine = create_async_engine(
                self.connection_url,
                pool_pre_ping=handle_stale_connections
            )
        except NoSuchModuleError as exception:
            raise DialectDriverError(self.connection_url_kwargs.drivername) from exception
        except OperationalError as exception:
            raise DatabaseConnectionError(self.connection_url) from exception
        except SQLAlchemyError as exception:
            raise SqlAlchemyOperationError() from exception

        return engine

    def _create_sessionmaker(self, expire_on_commit: bool = False) -> async_sessionmaker[AsyncSession]:
        """Create an asynchronous sessionmaker used to expend sessions.

        Args:
            expire_on_commit (bool, optional): Do not cache object attributes once session is closed. Defaults to False.

        Raises:
            SessionFactoryCreationError: Raise when there is an issue while creating the session factory.

        Returns:
            async_sessionmaker[AsyncSession]: The session factory.

        """
        try:
            session_factory = async_sessionmaker(
                self.engine,
                expire_on_commit=expire_on_commit
            )
        except SQLAlchemyError as exception:
            raise SessionFactoryCreationError(self.connection_url) from exception
        return session_factory

    async def create_schema(self) -> None:
        """Create the schema in ./postgres/database/models.py.

        Raises:
            SchemaCreationError: Raise if there is an error while creating the scehema.

        """
        logger.info("Creating schema if it does not exist...")
        try:
            async with self.engine.begin() as conn:
                await conn.run_sync(Base.metadata.create_all)
            logger.info("Schema created or retained successfully...")
        except SQLAlchemyError as exception:
            logger.exception("Error while creating schema.")
            raise SchemaCreationError(self.connection_url) from exception

    async def get_session(self) -> AsyncGenerator[AsyncSession, None]:
        """Constrain session operation within a context manager.

        Raises:
            SessionRetrievalError: Raise if there is an error while retrieving the session.

        Yields:
            AsyncSession: Active database session that commits/rolls back depending on success.
              The session closes at the end of the context.

        """
        async with self.sessionmaker() as session:
            try:
                yield session
                await session.commit()
            except (SQLAlchemyError, Exception) as exception:
                await session.rollback()
                raise SessionRetrievalError() from exception
            finally:
                await session.close()
