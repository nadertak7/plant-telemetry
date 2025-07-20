import click

from mosquitto_consumer.utils.plants_utils import add_plant

# ^ Though sql_client is also imported into mqtt_consumer_client.py,
# the SQLAlchemy engine is not created twice due to the way python
# caches modules.

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


if __name__ == "__main__":
    cli()
