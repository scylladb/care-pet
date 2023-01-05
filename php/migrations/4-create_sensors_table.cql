CREATE TABLE IF NOT EXISTS carepet.sensor
(
    pet_id UUID,
    sensor_id UUID,
    type TEXT,
    PRIMARY KEY (pet_id, sensor_id)
    );