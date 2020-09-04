package com.carepet.model;

import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.mapper.annotations.DaoFactory;

@com.datastax.oss.driver.api.mapper.annotations.Mapper
public interface Mapper {
    @DaoFactory
    OwnerDAO owner();

    @DaoFactory
    PetDAO pet();

    @DaoFactory
    SensorDAO sensor();

    @DaoFactory
    MeasureDAO measurement();

    @DaoFactory
    SensorAvgDAO sensorAvg();

    static com.datastax.oss.driver.api.mapper.MapperBuilder<Mapper> builder(CqlSession session) {
        return new MapperBuilder(session);
    }
}
