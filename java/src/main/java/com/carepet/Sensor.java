package com.carepet;

import com.carepet.model.*;
import com.datastax.oss.driver.api.core.CqlSession;
import org.apache.commons.lang.math.RandomUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.time.Instant;

public class Sensor
{
    public static void main( String[] args )
    {
        final Config config = Config.parse(args);

        final Sensor client = new Sensor(config);
        client.save();
        client.run();
    }

    private static final Logger LOG = LoggerFactory.getLogger(Migrate.class);

    private final Config config;

    private final Owner owner;
    private final Pet pet;
    private final com.carepet.model.Sensor[] sensors;

    public Sensor(Config config) {
        this.config = config;

        this.owner = Owner.random();
        this.pet = Pet.random();
        this.sensors = new com.carepet.model.Sensor[SensorType.values().length];
        for (int i = 0; i < this.sensors.length; i ++) {
            this.sensors[i] = com.carepet.model.Sensor.random();
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

            LOG.info("owner = " + owner);
            LOG.info("pet = " + pet);

            m.owner().create(owner);
            m.pet().create(pet);

            for (com.carepet.model.Sensor s: sensors) {
                LOG.info("sensor = " + s);

                m.sensor().create(s);
            }
        }
    }

    /** Generate random sensors data and push it to the app. */
    private void run() {
        try (CqlSession session = keyspace()) {
            Mapper map = Mapper.builder(session).build();
            Measure m = new Measure();

            while (true) {
                for (com.carepet.model.Sensor s: sensors) {
                    readSensorData(s, m);

                    map.measurement().create(m);

                    LOG.info(m.toString());
                }

                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    break;
                }
            }
        }
    }

    private void readSensorData(com.carepet.model.Sensor s, Measure m) {
        m.setSensorId(s.getSensorId());
        m.setTs(Instant.now());
        m.setValue(s.randomData());
    }
}
