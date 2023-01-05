CREATE TABLE IF NOT EXISTS carepet.measurement
(
    sensor_id UUID,
    ts    TIMESTAMP,
    value FLOAT,
    PRIMARY KEY (sensor_id, ts)
    ) WITH compaction = { 'class' : 'TimeWindowCompactionStrategy' };