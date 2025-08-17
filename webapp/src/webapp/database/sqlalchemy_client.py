import streamlit as st
from sqlalchemy import Engine, create_engine, exc
from sqlalchemy.orm import Session, sessionmaker

from webapp.config.exceptions import DatabaseConnectionError, DialectDriverError, SessionFactoryCreationError
from webapp.config.logs import logger
from webapp.config.settings import settings


@st.cache_resource # Ensure that engine and session cannot be returned more than once
def create_session_factory(
    dialect: str = 'postgresql',
    driver: str = 'psycopg2',
    user: str = settings.POSTGRES_SUPER_USER,
    password: str = settings.POSTGRES_SUPER_PASSWORD.get_secret_value(),
    host: str = settings.POSTGRES_DB_HOST,
    port: int = 5432,
    database: str = settings.POSTGRES_DB
) -> sessionmaker[Session]:
    """Create a session factory from a connection. With a context manager,
      we can treat one or more database operations with atomicity (meaning that
      a commit occurs if all operations in the context succeed, or a rollback occurs
      if one operation in the context fails.

    Usage:
        Session: sessionmaker[Session] = create_session_factory()
        with Session.begin() as session:
            # Operations go here
        # Once context ends, a commit/rollback occurs depending on the success
        # of the operations.

    Args:
        Refer to https://docs.sqlalchemy.org/en/20/core/engines.html
        dialect (str):  Defaults to 'postgresql'.
        driver (str): Defaults to 'psycopg2'.
        user (str): Defaults to settings.POSTGRES_SUPER_USER.
        password (str): Defaults to settings.POSTGRES_SUPER_PASSWORD.get_secret_value().
        host (str): Defaults to settings.POSTGRES_DB_HOST.
        port (int): Defaults to 5432.
        database (str): Defaults to settings.POSTGRES_DB.

    Raises:
        DialectDriverError: Raise if dialect or driver are incorrect.
        DatabaseConnectionError: Raise if there is an issue while connecting via
          the provided connection string.
        SessionFactoryCreationError: Raise if there was an issue creating the session
          factory.

    Returns:
        sessionmaker[Session]: The session factory we can begin sessions from.

    """  # noqa: D205
    connection_url: str = (
        f"{dialect}+{driver}://" \
        f"{user}" \
        f":{password}" \
        f"@{host}:{port}" \
        f"/{database}"
    )
    try:
        engine: Engine = create_engine(
            connection_url,
            pool_pre_ping=True # Check for stale connections
        )

    except exc.NoSuchModuleError as exception:
        error_message: str = (
            f"Unsupported dialect and driver combination {dialect}+{driver}. " \
            "Ensure that dialect is correct and driver is installed in environment."
            )
        logger.exception(error_message)
        raise DialectDriverError() from exception
    except exc.OperationalError as exception:
        logger.exception(
            "Error while connecting to psql database %s, ensure credentials are correct.",
            host
        )
        raise DatabaseConnectionError() from exception
    except exc.SQLAlchemyError as exception:
        logger.exception(
            "Unexpected error while connecting to psql database %s.",
            host
        )
        raise DatabaseConnectionError() from exception

    try:
        session_factory: sessionmaker[Session] = sessionmaker(bind=engine)

    except exc.SQLAlchemyError as exception:
        logger.exception(f"Error while creating session factory for {connection_url}.")
        raise SessionFactoryCreationError() from exception

    return session_factory

Session: sessionmaker[Session] = create_session_factory()
