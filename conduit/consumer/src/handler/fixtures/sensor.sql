INSERT INTO
    sensor
    (id, topic, plant_id, dry_adc, wet_adc, archived_at)
VALUES
    (1, 'sensor/1', 1, 1000, 0, NULL),
    (2, 'sensor/archived', 1, 1000, 0, '1970-01-01'),
    (3, 'sensor/plant_unregistered', NULL, 1000, 0, NULL),
    (4, 'sensor/plant_archived', 2, 1000, 0, NULL);
