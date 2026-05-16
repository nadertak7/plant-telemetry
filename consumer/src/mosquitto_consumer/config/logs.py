import logging
import os
from logging import Formatter, Logger, StreamHandler
from logging.handlers import RotatingFileHandler
from typing import TextIO


def configure_logging() -> Logger:
    """Configure logs for console (DEBUG and above) and log file (INFO and above).

    Use `from mosquitto_consumer.config.logs import logger`

    Returns:
        Logger: Configuration that is instantiated and imported into files
        that require logging.

    """
    os.makedirs('logs', exist_ok=True)
    logger: Logger = logging.getLogger(__name__)
    logger.setLevel(logging.DEBUG)
    log_format: Formatter= logging.Formatter(
        "%(asctime)s - %(filename)s - %(levelname)s - %(message)s"
    )

    # Handle console logs
    console_handler: StreamHandler[TextIO] = logging.StreamHandler()
    console_handler.setFormatter(log_format)
    console_handler.setLevel(logging.DEBUG)
    logger.addHandler(console_handler)

    # Handle file logs
    file_handler: RotatingFileHandler = RotatingFileHandler(
        "logs/mosquitto_consumer.log",
        maxBytes=5*1024*1024, # 5mb
        backupCount=3
    )
    file_handler.setFormatter(log_format)
    file_handler.setLevel(logging.INFO)
    logger.addHandler(file_handler)

    return logger

logger: Logger = configure_logging()
