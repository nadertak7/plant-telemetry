CREATE TABLE sensor (
    id SERIAL PRIMARY KEY,
    topic TEXT NOT NULL,
    plant_id INT,
    dry_adc INT NOT NULL,
    wet_adc INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at TIMESTAMPTZ,

    CONSTRAINT ck_sensor_dry_adc_wet_adc
        CHECK (dry_adc > wet_adc)
);

CREATE UNIQUE INDEX ux_sensor_topic
    ON sensor(topic)
    WHERE archived_at IS NULL;

CREATE TABLE plant (
    id SERIAL PRIMARY KEY,
    display_name TEXT NOT NULL,
    lower_threshold_perc SMALLINT NOT NULL,
    warning_threshold_perc SMALLINT NOT NULL,
    upper_threshold_perc SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_at TIMESTAMPTZ,

    CONSTRAINT ck_plant_lower_warning_upper_threshold_perc
        CHECK (
            lower_threshold_perc < warning_threshold_perc AND warning_threshold_perc < upper_threshold_perc
            AND lower_threshold_perc >= 0 AND upper_threshold_perc <= 100
        )
);

CREATE UNIQUE INDEX ux_plant_display_name
    ON plant(display_name)
    WHERE archived_at IS NULL;

ALTER TABLE sensor
    ADD CONSTRAINT fk_sensor_plant
    FOREIGN KEY (plant_id) REFERENCES plant(id)
    ON DELETE SET NULL;

CREATE TABLE plant_telemetry (
    id SERIAL PRIMARY KEY,
    plant_id INT NOT NULL REFERENCES plant(id),
    sensor_id INT NOT NULL REFERENCES sensor(id),
    adc INT NOT NULL,
    moisture_perc SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_plant_telemetry_moisture_perc
        CHECK (moisture_perc BETWEEN 0 AND 100)
);

CREATE INDEX ix_plant_telemetry_plant_id_created_at
    ON plant_telemetry(plant_id, created_at DESC);
