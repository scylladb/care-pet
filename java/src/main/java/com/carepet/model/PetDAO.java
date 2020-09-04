package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Insert;
import com.datastax.oss.driver.api.mapper.annotations.Select;
import com.datastax.oss.driver.api.mapper.annotations.Update;

import java.util.UUID;

@Dao
public interface PetDAO {
    @Insert
    void create(Pet pet);

    @Update
    void update(Pet pet);

    @Select
    Pet get(UUID owner, UUID id);
}
