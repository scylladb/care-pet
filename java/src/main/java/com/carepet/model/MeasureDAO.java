package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Insert;
import com.datastax.oss.driver.api.mapper.annotations.Select;
import com.datastax.oss.driver.api.mapper.annotations.Update;

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
}
