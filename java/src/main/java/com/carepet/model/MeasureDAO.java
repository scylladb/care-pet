package com.carepet.model;

import com.datastax.oss.driver.api.core.cql.ResultSet;
import com.datastax.oss.driver.api.mapper.annotations.*;

import java.time.Instant;
import java.util.UUID;

@Dao
public interface MeasureDAO {
    @Insert
    void create(Measure measure);

    @Update
    void update(Measure measure);

    @Select
    Measure get(UUID sensor, Instant ts);

    @Query("SELECT value FROM measurement WHERE sensor_id = :sensor AND ts >= :start AND ts <= :end")
    ResultSet find(UUID sensor, Instant start, Instant end);
}
