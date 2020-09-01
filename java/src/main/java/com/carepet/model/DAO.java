package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Select;

import java.time.Instant;
import java.sql.Date;
import java.util.UUID;

@Dao
public interface DAO {
        @Select
        Owner getOwner(UUID id);

        @Select
        Pet getPet(UUID ownerId, UUID petId);

        // @Select
        // List<Pet> getOwnerPets(UUID ownerId);

        @Select
        Sensor getSensor(UUID petId, UUID sensorId);

        @Select
        Measure getMeasurement(UUID sensorId, Instant ts);

        @Select
        SensorAvg getSensorAvg(UUID sensorId, Date date, int hour);
}
