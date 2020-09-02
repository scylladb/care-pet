package com.carepet;

import com.carepet.model.Mapper;
import com.carepet.model.Owner;
import com.carepet.model.Pet;
import com.carepet.model.SensorType;
import com.datastax.oss.driver.api.core.CqlSession;
import org.apache.commons.lang.math.RandomUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Sensor
{
    public static void main( String[] args )
    {
        final Config config = Config.parse(args);

        final Sensor client = new Sensor(config);
        client.save();
    }

    private static final Logger LOG = LoggerFactory.getLogger(Migrate.class);

    private final Config config;

    private final Owner owner;
    private final Pet pet;
    private final com.carepet.model.Sensor[] sensor;

    public Sensor(Config config) {
        this.config = config;

        this.owner = Owner.random();
        this.pet = Pet.random();
        this.sensor = new com.carepet.model.Sensor[1 + RandomUtils.nextInt(SensorType.values().length)];
        for (int i = 0; i < this.sensor.length; i ++) {
            this.sensor[i] = com.carepet.model.Sensor.random();
        }
    }

    /** Initiates a connection to the session specified by the application.conf. */
    public CqlSession keyspace() {
        return config.builder(Config.keyspace).build();
    }

    /** Save owner, pet and sensors to the database. */
    private void save() {
        try (CqlSession session = keyspace()) {
            Mapper m = Mapper.builder(session).build();
        }
    }
}
