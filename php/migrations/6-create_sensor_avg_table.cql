CREATE TABLE IF NOT EXISTS carepet.sensor_avg
(
    sensor_id UUID,
    date DATE,
    hour INT,
    value FLOAT,
    PRIMARY KEY (sensor_id, date, hour)
) WITH compaction = { 'class' : 'TimeWindowCompactionStrategy' };
