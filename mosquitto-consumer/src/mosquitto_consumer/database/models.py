from datetime import datetime
from typing import Any, Literal, Tuple

from sqlalchemy import CheckConstraint, DateTime, ForeignKey, Integer, String, UniqueConstraint
from sqlalchemy.orm import Mapped, declarative_base, mapped_column

from mosquitto_consumer.config.enums import TableNames

Base: Any = declarative_base()

class Plant(Base):
    """Model for plants table."""

    __tablename__: Literal[TableNames.PLANTS] = TableNames.PLANTS

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    plant_name: Mapped[str] =  mapped_column(String, unique=True, nullable=False)

    # Add constraints for conflict statement
    __table_args__: Tuple[UniqueConstraint] = (
        UniqueConstraint(
            "id",
            "plant_name",
            name="unique_id_plant_name"
        ),
    )


class PlantMoistureLog(Base):
    """Model for plants_moisture_log table."""

    __tablename__: Literal[TableNames.PLANTS_MOISTURE_LOG] = TableNames.PLANTS_MOISTURE_LOG

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    plant_id: Mapped[int] = mapped_column(Integer, ForeignKey('plants.id'), nullable=False, index=True)
    created_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), nullable=False)
    adc_value: Mapped[int] = mapped_column(Integer, nullable=False)
    dry_value: Mapped[int] = mapped_column(Integer, nullable=False)
    wet_value: Mapped[int] = mapped_column(Integer, nullable=False)
    moisture_perc: Mapped[int] = mapped_column(Integer, nullable=False)

    # Add constraints which should be enforced by ESP8266 code
    __table_args__: Tuple[CheckConstraint, CheckConstraint] = (
        CheckConstraint(
            "moisture_perc BETWEEN 0 AND 100",
            name="check_moisture_perc_range"
        ),
        CheckConstraint(
            "adc_value BETWEEN wet_value AND dry_value",
            name="check_adc_value_range"
        )
    )
