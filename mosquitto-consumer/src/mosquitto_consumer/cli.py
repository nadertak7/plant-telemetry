import click

from mosquitto_consumer.utils.add_plants import add_plants

# ^ Though sql_client is also imported into mqtt_consumer_client.py,
# the SQLAlchemy engine is not created twice due to the way python
# caches modules.

@click.group
def cli() -> None:
    """Command line interface for Mosquitto consumer."""
    pass

@cli.command
def updateplants() -> None:
    """Update the plants table if new items have been added while the script is running."""
    add_plants

if __name__ == "__main__":
    cli()
