INSERT INTO
    sensor
    (id, topic, plant_id, dry_adc, wet_adc, archived_at)
VALUES
    (1, 'sensor/active_with_plant', 1, 1000, 0, NULL),
    (2, 'sensor/active_with_archived_plant', 2, 1000, 0, NULL),
    (3, 'sensor/active_without_plant', NULL, 1000, 0, NULL),
    (4, 'sensor/archived', NULL, 1000, 0, '1970-01-01')
