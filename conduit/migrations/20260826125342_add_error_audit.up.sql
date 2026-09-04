CREATE TYPE handler_error AS ENUM (
   'PayloadParseError',
   'SensorNotRegistered',
   'SensorArchived',
   'PlantNotRegistered',
   'PlantArchived',
   'AdcNotInRange',
   'InvalidSensorCalibration'
);

CREATE TABLE error_audit (
    id SERIAL PRIMARY KEY,
    topic TEXT NOT NULL,
    sensor_id INT REFERENCES sensor(id),
    recorded_at TIMESTAMPTZ,
    kind handler_error NOT NULL,
    message TEXT NOT NULL,
    payload BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ix_error_audit_sensor_id_recorded_at
    ON error_audit(sensor_id, recorded_at DESC)
    WHERE recorded_at IS NOT NULL;
