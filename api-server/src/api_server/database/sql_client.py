from dataclasses import asdict, dataclass
from typing import Any, Callable, Union, overload

from sqlalchemy import Engine, MetaData, create_engine
from sqlalchemy.engine import URL
from sqlalchemy.exc import NoSuchModuleError, OperationalError, SQLAlchemyError
from sqlalchemy.ext.asyncio import AsyncEngine, AsyncSession, async_sessionmaker, create_async_engine
from sqlalchemy.orm import Session, sessionmaker

from api_server.config.exceptions import (
    ConnectionUrlValidationError,
    DatabaseConnectionError,
    DialectDriverError,
    SchemaReflectionError,
    SessionFactoryCreationError,
    SqlAlchemyOperationError,
)
from api_server.config.logs import logger


@dataclass
class _ConnectionUrlArgs:
    """Validate arguments passed into the SQLAlchemy URL object."""

    drivername: str
    username: str
    password: str
    host: str
    port: int
    database: str


class SqlClientBase:
    """Handle SQLAlchemy database logic, including connections, and dynamic queries."""

    def __init__(
        self,
        username: str,
        password: str,
        host: str,
        port: int,
        database: str,
        dialect: str = 'postgresql',
        driver: str = 'psycopg2',
        ) -> None:
        """Instantiate SqlClient class. Attempt a database connection in the constructor.

        Args:
            username (str): The database connection username.
            password (str): The database connection password.
            host (str): The host of the database.
            port (int): The port of the database.
            database (str): The name of the schema to connect to.
            dialect (str, optional): The chosen SQL dialect for queries. Defaults to 'postgresql'.
            driver (str, optional): The python driver in the environment. Defaults to 'psycopg2'.

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

        self.connection_url = self._create_connection_url()

    def _create_connection_url(self) -> URL:
        """Combine arguments passed into the constructor into a SQLAlchemy connection URL string.

        Returns:
        URL: Correctly encoded connection URL used to create SQLAlchemy engine.

        """
        connection_url_object: URL = URL.create(**asdict(self.connection_url_kwargs))

        return connection_url_object

    @overload
    def _create_engine_factory(
        self,
        factory: Callable[..., Engine]
    ) -> Engine: ...

    @overload
    def _create_engine_factory(
        self,
        factory: Callable[..., AsyncEngine]
    ) -> AsyncEngine: ...

    def _create_engine_factory(
        self,
        factory: Callable[..., Union[Engine, AsyncEngine]]
    ) -> Union[Engine, AsyncEngine]:
        try:
            engine = factory(
                self.connection_url,
                pool_pre_ping=True # checks for stale connections
            )
        except NoSuchModuleError as exception:
            raise DialectDriverError(self.connection_url_kwargs.drivername) from exception
        except OperationalError as exception:
            raise DatabaseConnectionError(self.connection_url) from exception
        except SQLAlchemyError as exception:
            raise SqlAlchemyOperationError() from exception

        return engine

    @overload
    def create_sessionmaker_factory(
        self,
        factory: type[sessionmaker[Session]],
        engine: Engine,
        **sessionmaker_extra_kwargs: Any  # noqa: ANN401
    ) -> sessionmaker[Session]: ...

    @overload
    def create_sessionmaker_factory(
        self,
        factory: type[async_sessionmaker[AsyncSession]],
        engine: AsyncEngine,
        **sessionmaker_extra_kwargs: Any  # noqa: ANN401
    ) -> async_sessionmaker[AsyncSession]: ...

    def create_sessionmaker_factory(
        self,
        factory: Union[type[sessionmaker[Session]], type[async_sessionmaker[AsyncSession]]],
        engine: Union[Engine, AsyncEngine],
        **sessionmaker_extra_kwargs: Any
    ) -> Union[sessionmaker[Session], async_sessionmaker[AsyncSession]]:
        try:
            session_factory = factory(engine, **sessionmaker_extra_kwargs)  # type: ignore[call-overload]
        except SQLAlchemyError as exception:
            raise SessionFactoryCreationError(self.connection_url) from exception

        return session_factory

    def reflect_schema_factory(
        self,
        factory: Callable[[], MetaData]
    ) -> MetaData:
        try:
            reflected_schema = factory()
        except OperationalError as exception:
            raise SchemaReflectionError(self.connection_url) from exception
        except SQLAlchemyError as exception:
            raise SqlAlchemyOperationError() from exception

        return reflected_schema

class SqlClient(SqlClientBase):
    """Handle synchronous SQLAlchemy database logic, including connections, and dynamic queries."""

    def __init__(
        self,
        username: str,
        password: str,
        host: str,
        port: int,
        database: str,
        dialect: str = 'postgresql',
        driver: str = 'psycopg2',
        ) -> None:
        """Instantiate SqlAsyncClient class. Attempt a database connection in the constructor."""
        super().__init__(
            username,
            password,
            host,
            port,
            database,
            dialect,
            driver
        )
        self.engine = self._create_engine()

    def _create_engine(self) -> Engine:
        return self._create_engine_factory(create_engine)

    def create_sessionmaker(self) -> sessionmaker[Session]:
        return self.create_sessionmaker_factory(sessionmaker, self.engine)

    def reflect_schema(self) -> MetaData:
        def reflect_schema_synchronous() -> MetaData:
            metadata = MetaData()
            metadata.reflect(bind=self.engine)
            return metadata

        return self.reflect_schema_factory(reflect_schema_synchronous)


class AsyncSqlClient(SqlClientBase):
    """Handle synchronous SQLAlchemy database logic, including connections, and dynamic queries."""

    def __init__(
        self,
        username: str,
        password: str,
        host: str,
        port: int,
        database: str,
        dialect: str = 'postgresql',
        driver: str = 'psycopg2',
        ) -> None:
        """Instantiate SqlAsyncClient class. Attempt a database connection in the constructor."""
        super().__init__(
            username,
            password,
            host,
            port,
            database,
            dialect,
            driver
        )
        self.engine = self._create_engine()

    def _create_engine(self) -> AsyncEngine:
        return self._create_engine_factory(create_async_engine)

    def create_sessionmaker(self) -> async_sessionmaker[AsyncSession]:
        return self.create_sessionmaker_factory(
            async_sessionmaker,
            self.engine,
            expire_on_commit=False
        )

    async def reflect_schema(self) -> MetaData:
        try:
            metadata = MetaData()
            async with self.engine.begin() as conn:
                await conn.run_sync(metadata.reflect)
            return metadata
        except OperationalError as exception:
            raise SchemaReflectionError(self.connection_url) from exception
        except SQLAlchemyError as exception:
            raise SqlAlchemyOperationError() from exception
