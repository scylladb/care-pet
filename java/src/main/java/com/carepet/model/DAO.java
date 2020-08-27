package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Select;

import java.util.UUID;

@Dao
public interface DAO {
        /** Simple selection by full primary key. */
        @Select
        Owner getOwner(UUID id);
}
