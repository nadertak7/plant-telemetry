from datetime import datetime, timezone
from typing import Optional

import click
from sqlalchemy.exc import IntegrityError, SQLAlchemyError

from mosquitto_consumer.config.exceptions import SqlClientError
from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.database.models import Plant
from mosquitto_consumer.database.sql_client import sql_client


@click.group
def cli() -> None:
    """Command line interface for Mosquitto consumer."""
    pass

@cli.command
@click.option(
    "--plant_name",
    prompt="Enter the name of the plant",
    help="The name of the plant. This is how it will be aliased in the monitoring app."
)
@click.option(
    "--topic_plant_location",
    prompt="Enter the exact <location> in 'plant-monitoring/<location>/<topic_plant_name>/telemetry'",
    help="The name of the plant in the topic. Must match exactly that in the producer to recieve messages."
)
@click.option(
    "--topic_plant_name",
    prompt="Enter the exact <topic_plant_name> in 'plant-monitoring/<location>/<topic_plant_name>/telemetry'.",
    help="The name of the location in the topic. Must match exactly that in the producer to recieve messages."
)
def add(plant_name: str, topic_plant_name: str, topic_plant_location: str) -> None:
    """Add a plant to the plants table via cli."""
    topic: str = f"plant-monitoring/{topic_plant_location}/{topic_plant_name}/telemetry"

    click.echo("\nAdding plant with the following details:")
    click.echo(f"  Name:  {plant_name}")
    click.echo(f"  Topic: {topic}")

    click.confirm("Do you want to continue?", abort=True)

    click.echo("Adding plant to the database...")

    try:
        new_plant: Plant = Plant(
            plant_name=plant_name,
            topic=topic
        )
        with sql_client.get_session() as session, session.begin():
            session.add(new_plant)
    except SqlClientError:
        logger.exception(f"Error while adding {plant_name} to table.")
        raise
    except IntegrityError:
        logger.error("One of the values provided matches an existing value in the table. Record not created.")
        raise
    except SQLAlchemyError:
        logger.exception(f"Unexpected error while adding {plant_name} to table.")
        raise

    click.echo("Successfully added plant.")
    click.secho("Warning: Container must be restarted to receive messages from this topic.", fg="yellow")

@cli.command
@click.option(
    "--plant_id",
    prompt="Enter the id of the plant in the plants table which you would like to set the deprecation status of",
    help="The id of the plant to be deprecated or de-deprecated."
)
def deprecate(plant_id: int) -> None:
    """Deprecate or de-deprecate a plant_id via cli."""
    try:
        with sql_client.get_session() as session, session.begin():
            selected_plant: Optional[Plant] = session.get(Plant, plant_id)
            if not selected_plant:
                click.secho(f"Error: Plant ID {plant_id} does not exist. Try again.", fg="red")
                return

            click.echo(f"Select deprecation status for plant {selected_plant.plant_name}")
            click.echo("\n1. Deprecate")
            click.echo("2. Activate")

            is_deprecated: bool = True if click.prompt(
                "Choice",
                type=click.IntRange(1, 2),
                default=1
            ) == 1 else False

            if is_deprecated == selected_plant.is_deprecated:
                click.secho(
                    f"Warning: Plant ID {plant_id} already has a deprecation status of {is_deprecated}.",
                    fg="yellow"
                )
                return

            click.echo(f"Setting deprecation status to {is_deprecated} for plant {selected_plant.plant_name}")
            click.confirm("Do you want to continue?", abort=True)
            click.echo("Adding plant to database.")
            selected_plant.is_deprecated = is_deprecated
            if is_deprecated:
                selected_plant.last_deprecated_at = datetime.now(timezone.utc)
    except SqlClientError:
        logger.exception("Error while retrieving topics from plants table.")
        raise
    except SQLAlchemyError:
        logger.exception("Unexpected error while retrieving topics from plants table.")
        raise

    click.echo(f"Successfully set deprecation status of {selected_plant.plant_name} to {is_deprecated}.")

if __name__ == "__main__":
    cli()
