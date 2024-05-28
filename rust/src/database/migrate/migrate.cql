CREATE TABLE IF NOT EXISTS carepet.owners
(
    owner_id UUID,
    address TEXT,
    name    TEXT,
    PRIMARY KEY (owner_id)
);

CREATE TABLE IF NOT EXISTS carepet.pets
(
    owner_id UUID,
    pet_id   UUID,
    chip_id  TEXT,
    species  TEXT,
    breed    TEXT,
    color    TEXT,
    gender   TEXT,
    age      INT,
    weight   FLOAT,
    address  TEXT,
    name     TEXT,
    PRIMARY KEY (owner_id, pet_id)
);

CREATE TABLE IF NOT EXISTS carepet.sensors
(
    pet_id UUID,
    sensor_id UUID,
    type TEXT,
    PRIMARY KEY (pet_id, sensor_id)
);

CREATE TABLE IF NOT EXISTS carepet.measurements
(
    sensor_id UUID,
    ts    TIMESTAMP,
    value FLOAT,
    PRIMARY KEY (sensor_id, ts)
) WITH compaction = { 'class' : 'TimeWindowCompactionStrategy' };

CREATE TABLE IF NOT EXISTS carepet.sensors_avg
(
    sensor_id UUID,
    date  DATE,
    hour  INT,
    value FLOAT,
    PRIMARY KEY (sensor_id, date, hour)
) WITH compaction = { 'class' : 'TimeWindowCompactionStrategy' };
