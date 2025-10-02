from typing import Optional

import click
from sqlalchemy.exc import SQLAlchemyError

from mosquitto_consumer.config.exceptions import SqlClientError
from mosquitto_consumer.config.logs import logger
from mosquitto_consumer.database.models import Plant
from mosquitto_consumer.database.sql_client import sql_client
from mosquitto_consumer.utils.plants_utils import add_plant


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
def addplant(plant_name: str, topic_plant_name: str, topic_plant_location: str) -> None:
    """Add a plant to the plants table via cli."""
    topic: str = f"plant-monitoring/{topic_plant_location}/{topic_plant_name}/telemetry"

    click.echo("\nAdding plant with the following details:")
    click.echo(f"  Name:  {plant_name}")
    click.echo(f"  Topic: {topic}")

    click.confirm("Do you want to continue?", abort=True)

    click.echo("Adding plant to the database...")
    # Not possible to SQL inject due to the way SQLAlchemy uses parameterised queries.
    if add_plant(plant_name, topic):
        click.echo("Successfully added plant.")
        click.secho("Warning: Container must be restarted to receive messages from this topic.", fg="yellow")

@cli.command
@click.option(
    "--plant_id",
    prompt="Enter the id of the plant in the plants table which you would like to set the deprecation status of.",
    help="The id of the plant to be deprecated or de-deprecated."
)
@click.option(
    "--is_deprecated",
    is_flag=True,
    default=True,
    prompt="Enter the desired deprecation status of the plant.",
    help="The desired deprecation status of the plant."
)
def deprecate(plant_id: int, is_deprecated: bool) -> None:
    """Deprecate or de-deprecate a plant_id via cli."""
    try:
        with sql_client.get_session() as session, session.begin():
            selected_plant: Optional[Plant] = session.get(Plant, plant_id)
            if not selected_plant:
                click.secho(f"Error: Plant ID {plant_id} does not exist. Try again.", fg="red")
                return

            click.echo(f"Setting deprecation status to {is_deprecated} for plant {selected_plant.plant_name}")
            click.confirm("Do you want to continue?", abort=True)
            click.echo("Adding plant to database.")
            selected_plant.is_deprecated = is_deprecated
    except SqlClientError:
        logger.exception("Error while retrieving topics from plants table.")
        raise
    except SQLAlchemyError:
        logger.exception("Unexpected error while retrieving topics from plants table.")
        raise

if __name__ == "__main__":
    cli()
