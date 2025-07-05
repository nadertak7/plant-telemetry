-- This schema is deprecated by models.py but gives an overview of
-- the database design for those unfamiliar with SQLAlchemy

-- Create tables and indexes if they do not exist

CREATE TABLE IF NOT EXISTS plants (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    plant_name TEXT NOT NULL
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
    CONSTRAINT check_adc_value_range CHECK (adc_value BETWEEN dry_value AND wet_value)
);

CREATE INDEX IF NOT EXISTS idx_plants_moisture_logs_plant_id
    ON plants_moisture_log(plant_id);

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
            p_constrant_name,
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
        p_constrant_name=>'fk_plants_moisture_log_plant_id',
        p_column_name=>'plant_id',
        p_referenced_table=>'plants',
        p_referenced_column=>'id'
    );
