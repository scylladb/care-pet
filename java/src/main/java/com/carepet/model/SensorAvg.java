package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.ClusteringColumn;
import com.datastax.oss.driver.api.mapper.annotations.CqlName;
import com.datastax.oss.driver.api.mapper.annotations.Entity;
import com.datastax.oss.driver.api.mapper.annotations.PartitionKey;

import java.sql.Date;
import java.util.UUID;

@Entity
@CqlName("sensor_avg")
public class SensorAvg {
    @PartitionKey
    private UUID sensorId;

    @ClusteringColumn(0)
    private Date date;

    @ClusteringColumn(1)
    private int hour;

    private float value;

    public SensorAvg() {}

    public SensorAvg(UUID sensorId, Date date, int hour, float value) {
        this.sensorId = sensorId;
        this.date = date;
        this.hour = hour;
        this.value = value;
    }

    public UUID getSensorId() {
        return sensorId;
    }

    public void setSensorId(UUID sensorId) {
        this.sensorId = sensorId;
    }

    public Date getDate() {
        return date;
    }

    public void setDate(Date date) {
        this.date = date;
    }

    public int getHour() {
        return hour;
    }

    public void setHour(int hour) {
        this.hour = hour;
    }

    public float getValue() {
        return value;
    }

    public void setValue(float value) {
        this.value = value;
    }

    @Override
    public String toString() {
        return "SensorAvg{" +
                "sensorId=" + sensorId +
                ", date=" + date +
                ", hour=" + hour +
                ", value=" + value +
                '}';
    }
}
