from typing import Sequence

from sqlalchemy import Result, RowMapping, text
from sqlalchemy.exc import IntegrityError, SQLAlchemyError

from mosquitto_consumer.config.exceptions import SqlClientError
from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.database.sql_client import sql_client


def add_plant(plant_name: str, topic: str) -> bool:
    """Add a record to the plants table and subscribe to topic.

    Args:
        plant_name (str): The name of the plant to add to the database
        topic (str): The topic that the consumer will listen to from the broker

    """
    values_dict: dict = {
        "plant_name": plant_name,
        "topic": topic
    }
    # Add to database
    try:
        with sql_client.get_session() as session, session.begin():
            session.execute(
                text("""
                    INSERT INTO plants
                        (plant_name, topic)
                    VALUES
                        (:plant_name, :topic)
                """),
                values_dict
            )
    except SqlClientError:
        logger.exception(f"Error while adding {plant_name} to table.")
        return False
    except IntegrityError:
        # Using error as we do not want to print out whole exception when unneeded.
        logger.error("One of the values provided matches an existing value in the table. Record not created.")
        return False
    except SQLAlchemyError:
        logger.exception(f"Unexpected error while adding {plant_name} to table.")
        return False

    return True

def retrieve_plant_topics() -> Sequence[RowMapping] | None:
    """Retrieve all the existing topics to subscribe to."""
    try:
        with sql_client.get_session() as session, session.begin():
            query_result: Result = session.execute(text("SELECT id, topic FROM plants"))
            topic_list: Sequence[RowMapping] = query_result.mappings().all()
        logger.info("Retrieved %s topics from plants table.", len(topic_list))
    except SqlClientError:
        logger.exception("Error while retrieving topics from plants table.")
        return
    except SQLAlchemyError:
        logger.exception("Unexpected error while retrieving topics from plants table.")
        return

    return topic_list
