package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Insert;
import com.datastax.oss.driver.api.mapper.annotations.Select;
import com.datastax.oss.driver.api.mapper.annotations.Update;

import java.sql.Date;
import java.time.Instant;
import java.util.UUID;

@Dao
public interface OwnerDAO {
    @Insert
    void create(Owner owner);

    @Update
    void update(Owner owner);

    @Select
    Owner get(UUID id);

    // @Select
    // List<Pet> getOwnerPets(UUID ownerId);
}
