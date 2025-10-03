from typing import Sequence

from sqlalchemy import RowMapping, Select, select
from sqlalchemy.exc import SQLAlchemyError

from mosquitto_consumer.config.exceptions import SqlClientError
from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.database.models import Plant
from mosquitto_consumer.database.sql_client import sql_client


def retrieve_plant_topics() -> Sequence[RowMapping] | None :
    """Retrieve all the existing topics to subscribe to."""
    try:
        with sql_client.get_session() as session, session.begin():
            select_statment: Select[tuple[int, str]] = select(Plant.id, Plant.topic)
            topic_list: Sequence[RowMapping] = session.execute(select_statment).mappings().all()
        logger.info("Retrieved %s topics from plants table.", len(topic_list))
    except SqlClientError:
        logger.exception("Error while retrieving topics from plants table.")
        return
    except SQLAlchemyError:
        logger.exception("Unexpected error while retrieving topics from plants table.")
        return

    return topic_list
