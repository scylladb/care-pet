package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.ClusteringColumn;
import com.datastax.oss.driver.api.mapper.annotations.CqlName;
import com.datastax.oss.driver.api.mapper.annotations.Entity;
import com.datastax.oss.driver.api.mapper.annotations.PartitionKey;
import com.fasterxml.jackson.annotation.JsonProperty;
import org.apache.commons.lang.math.RandomUtils;

import java.util.UUID;

@Entity
@CqlName("sensor")
public class Sensor {
    @PartitionKey
    @JsonProperty("pet_id")
    private UUID petId;

    @ClusteringColumn
    @JsonProperty("sensor_id")
    private UUID sensorId;

    private String type;

    public Sensor() {
    }

    public Sensor(UUID petId, UUID sensorId, String type) {
        this.petId = petId;
        this.sensorId = sensorId;
        this.type = type;
    }

    public static Sensor random(UUID petId) {
        return new Sensor(
                petId,
                UUID.randomUUID(),
                SensorType.values()[RandomUtils.nextInt(SensorType.values().length)].getType());
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

    public float randomData() {
        switch (SensorType.fromString(type)) {
            case Temperature:
                // average F
                return 101.0f + RandomUtils.nextInt(10) - 4;
            case Pulse:
                // average beat per minute
                return 100.0f + RandomUtils.nextInt(40) - 20;
            case Respiration:
                // average inhales per minute
                return 35.0f + RandomUtils.nextInt(5) - 2;
            case Location:
                // pet can teleport
                return 10 * RandomUtils.nextFloat();
            default:
                return 0.0f;
        }
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
