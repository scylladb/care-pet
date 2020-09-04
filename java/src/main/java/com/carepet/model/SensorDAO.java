package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Insert;
import com.datastax.oss.driver.api.mapper.annotations.Select;
import com.datastax.oss.driver.api.mapper.annotations.Update;

import java.util.UUID;

@Dao
public interface SensorDAO {
    @Insert
    void create(Sensor sensor);

    @Update
    void update(Sensor sensor);

    @Select
    Sensor get(UUID pet, UUID id);
}
