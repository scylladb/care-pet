package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.ClusteringColumn;
import com.datastax.oss.driver.api.mapper.annotations.CqlName;
import com.datastax.oss.driver.api.mapper.annotations.Entity;
import com.datastax.oss.driver.api.mapper.annotations.PartitionKey;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.time.Instant;
import java.util.UUID;

@Entity
@CqlName("measurement")
public class Measure {
    @PartitionKey
    @JsonProperty("sensor_id")
    private UUID sensorId;

    @ClusteringColumn
    private Instant ts;

    private float value;

    public Measure() {
    }

    public Measure(UUID sensorId, Instant ts, float value) {
        this.sensorId = sensorId;
        this.ts = ts;
        this.value = value;
    }

    public UUID getSensorId() {
        return sensorId;
    }

    public void setSensorId(UUID sensorId) {
        this.sensorId = sensorId;
    }

    public Instant getTs() {
        return ts;
    }

    public void setTs(Instant ts) {
        this.ts = ts;
    }

    public float getValue() {
        return value;
    }

    public void setValue(float value) {
        this.value = value;
    }

    @Override
    public String toString() {
        return "Measure{" +
                "sensorId=" + sensorId +
                ", ts=" + ts +
                ", value=" + value +
                '}';
    }
}
