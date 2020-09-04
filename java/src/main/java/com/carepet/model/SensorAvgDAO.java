package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Insert;
import com.datastax.oss.driver.api.mapper.annotations.Select;
import com.datastax.oss.driver.api.mapper.annotations.Update;

import java.sql.Date;
import java.time.Instant;
import java.util.UUID;

@Dao
public interface SensorAvgDAO {
    @Insert
    void create(SensorAvg avg);

    @Update
    void update(SensorAvg avg);

    @Select
    SensorAvg get(UUID sensor, Date date, int hour);
}
