from datetime import datetime, timezone
from typing import Any, Literal, Tuple

from sqlalchemy import Boolean, CheckConstraint, DateTime, ForeignKey, Integer, String
from sqlalchemy.orm import Mapped, declarative_base, mapped_column

from mosquitto_consumer.config.enums import TableNames

Base: Any = declarative_base()

class Plant(Base):
    """Model for plants table."""

    __tablename__: Literal[TableNames.PLANTS] = TableNames.PLANTS

    id: Mapped[int] = mapped_column(Integer, primary_key=True)
    plant_name: Mapped[str] =  mapped_column(String, unique=True, nullable=False)
    topic: Mapped[str] = mapped_column(String, unique=True, nullable=False)
    is_deprecated: Mapped[bool] = mapped_column(Boolean, nullable=False, default=False)
    created_at: Mapped[datetime] = mapped_column(
        DateTime(timezone=True),
        nullable=False,
        default=datetime.now(timezone.utc)
    )
    last_deprecated_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), nullable=True)

    # Add constraints for topic format which should be enforced by cli function
    __table_args__: Tuple[CheckConstraint] = (
        CheckConstraint(
            "topic LIKE 'plant-monitoring/%/%/telemetry'",
            name="check_topic_format"
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
        ),
    )

class RecommendedPlantMoisture(Base):
    """Model for plant_moisture_recommended_percentage table."""

    __tablename__: Literal[TableNames.RECOMMENDED_PLANT_MOISTURE] = (
        TableNames.RECOMMENDED_PLANT_MOISTURE
    )

    plant_id: Mapped[int] = mapped_column(Integer, ForeignKey('plants.id'), primary_key=True)
    min_moisture_perc: Mapped[int] = mapped_column(Integer, nullable=False)
    max_moisture_perc: Mapped[int] = mapped_column(Integer, nullable=False)
    last_updated_at: Mapped[datetime] = mapped_column(DateTime(timezone=True), nullable=False)

    __table_args__: Tuple[CheckConstraint, CheckConstraint] = (
        CheckConstraint(
            "min_moisture_perc BETWEEN 0 AND 100 " \
            "AND max_moisture_perc BETWEEN 0 AND 100",
            name="check_recommended_moisture_perc_range"
        ),
        CheckConstraint(
            "max_moisture_perc > min_moisture_perc",
            name="check_max_greater_than_min"
        ),
    )
