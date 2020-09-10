package com.carepet.model;

import com.datastax.oss.driver.api.core.cql.ResultSet;
import com.datastax.oss.driver.api.mapper.annotations.*;

import java.time.LocalDate;
import java.util.UUID;

@Dao
public interface SensorAvgDAO {
    @Insert
    void create(SensorAvg avg);

    @Update
    void update(SensorAvg avg);

    @Select
    SensorAvg get(UUID sensor, LocalDate date, int hour);

    @Query("SELECT value FROM sensor_avg WHERE sensor_id = :sensor AND date = :date")
    ResultSet find(UUID sensor, LocalDate date);
}
