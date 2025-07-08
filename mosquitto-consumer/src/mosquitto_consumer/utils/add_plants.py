from typing import Dict, List

from sqlalchemy.dialects.postgresql import Insert, insert
from sqlalchemy.exc import SQLAlchemyError

from mosquitto_consumer.config.enums import MosquittoTopics
from mosquitto_consumer.config.exceptions import SqlClientError
from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.database.models import Plant
from mosquitto_consumer.database.sql_client import sql_client


def add_plants() -> None:
    """Update plants table based from data in the MosquittoTopics enum."""
    values: List[Dict[str, str]] = MosquittoTopics.id_to_topic_insert_values()
    # Formulate insert statement using SQLAlchemy dialects
    logger.info("Adding plants to table if they do not already exist...")
    insert_statement: Insert = insert(Plant).values(values)
    # This will do nothing if the id and plant name are the same, but will
    # raise an integrity error if the id is the same and the plant name is different
    on_conflict_statement: Insert = (
        insert_statement
        .on_conflict_do_nothing(
            index_elements=["id", "plant_name"]
        )
    )
    try:
        with sql_client.get_session() as session, session.begin():
            session.execute(on_conflict_statement)
            logger.info("Successfully inserted values to plants table...")
    except SqlClientError:
        logger.exception("Error while updating plants table.")
        return
    except SQLAlchemyError:
        logger.exception("Unexpected error while updating plants table.")
        return
