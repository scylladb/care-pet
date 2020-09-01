package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.ClusteringColumn;
import com.datastax.oss.driver.api.mapper.annotations.CqlName;
import com.datastax.oss.driver.api.mapper.annotations.Entity;
import com.datastax.oss.driver.api.mapper.annotations.PartitionKey;

import java.util.UUID;

@Entity
@CqlName("sensor")
public class Sensor {
    @PartitionKey
    private UUID petId;

    @ClusteringColumn
    private UUID sensorId;

    private String type;

    public Sensor() {}

    public Sensor(UUID petId, UUID sensorId, String type) {
        this.petId = petId;
        this.sensorId = sensorId;
        this.type = type;
    }

    public UUID getPetId() {
        return petId;
    }

    public void setPetId(UUID petId) {
        this.petId = petId;
    }

    public UUID getSensorId() {
        return sensorId;
    }

    public void setSensorId(UUID sensorId) {
        this.sensorId = sensorId;
    }

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
    }

    @Override
    public String toString() {
        return "Sensor{" +
                "petId=" + petId +
                ", sensorId=" + sensorId +
                ", type='" + type + '\'' +
                '}';
    }
}
