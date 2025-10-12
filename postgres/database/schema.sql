-- This schema is deprecated by models.py but gives an overview of
-- the database design for those unfamiliar with SQLAlchemy

-- Create tables and indexes if they do not exist

CREATE TABLE IF NOT EXISTS plants (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    plant_name TEXT NOT NULL UNIQUE,
    topic TEXT NOT NULL UNIQUE,
    is_deprecated BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    last_deprecated_at TIMESTAMP WITH TIME ZONE,

    CONSTRAINT check_topic_format CHECK (topic LIKE 'plant-monitoring/%/%/telemetry')
);

CREATE TABLE IF NOT EXISTS plants_moisture_log (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    plant_id INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    adc_value INT NOT NULL,
    dry_value INT NOT NULL,
    wet_value INT NOT NULL,
    moisture_perc INT NOT NULL

    CONSTRAINT check_moisture_perc_range CHECK (moisture_perc BETWEEN 0 AND 100),
    CONSTRAINT check_adc_value_range CHECK (adc_value BETWEEN wet_value AND dry_value)
);

CREATE INDEX IF NOT EXISTS idx_plants_moisture_logs_plant_id
    ON plants_moisture_log(plant_id);

CREATE TABLE IF NOT EXISTS recommended_plant_moisture (
    plant_id INT PRIMARY KEY,
    min_moisture_perc INT NOT NULL,
    max_moisture_perc INT NOT NULL,
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    CONSTRAINT check_recommended_moisture_perc_range CHECK (min_moisture_perc BETWEEN 0 AND 100 AND max_moisture_perc BETWEEN 0 AND 100),
    CONSTRAINT check_max_greater_than_min CHECK (max_moisture_perc > min_moisture_perc)
);

-- Create foreign keys if they do not exist

CREATE OR REPLACE FUNCTION add_foreign_key_if_not_exists (
    p_table_name TEXT,
    p_constrant_name TEXT,
    p_column_name TEXT,
    p_referenced_table TEXT,
    p_referenced_column TEXT
) RETURNS VOID AS $$
BEGIN
    IF NOT EXISTS (
        SELECT      0
        FROM        pg_constraint
        WHERE       pg_constraint.conname = pg_constraint.pg_constraint_name
    )
    THEN EXECUTE
        format(
            "ALTER TABLE %I ADD CONSTRAINT %I FOREIGN KEY (%I) REFERENCES %I(%I)",
            p_table_name,
            p_constraint_name,
            p_column_name,
            p_referenced_table,
            p_referenced_column
        );
    END IF;
END;
$$ LANGUAGE plpgsql;

SELECT
    add_foreign_key_if_not_exists(
        p_table_name=>'plants_moisture_log',
        p_constraint_name=>'fk_plants_moisture_log_plant_id',
        p_column_name=>'plant_id',
        p_referenced_table=>'plants',
        p_referenced_column=>'id'
    );

SELECT
    add_foreign_key_if_not_exists(
        p_table_name=>'recommended_plant_moisture',
        p_constraint_name=>'fk_recommended_plant_moisture_plant_id',
        p_column_name=>'plant_id',
        p_referenced_table=>'plants',
        p_referenced_column=>'id'
    );
