package com.carepet;

import com.carepet.model.*;
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.BatchStatementBuilder;
import com.datastax.oss.driver.api.core.cql.BatchType;
import com.datastax.oss.driver.api.core.cql.PreparedStatement;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import picocli.CommandLine;

import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;

public class Sensor
{
    public static void main( String[] args )
    {
        final Sensor client = new Sensor(Config.parse(new SensorConfig(), args));
        client.save();
        client.run();
    }

    private static final Logger LOG = LoggerFactory.getLogger(Migrate.class);

    private final SensorConfig config;

    private final Owner owner;
    private final Pet pet;
    private final com.carepet.model.Sensor[] sensors;

    public Sensor(SensorConfig config) {
        this.config = config;

        this.owner = Owner.random();
        this.pet = Pet.random(this.owner.getOwnerId());
        this.sensors = new com.carepet.model.Sensor[SensorType.values().length];
        for (int i = 0; i < this.sensors.length; i ++) {
            this.sensors[i] = com.carepet.model.Sensor.random(this.pet.getPetId());
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
            PreparedStatement statement =  session.prepare("INSERT INTO measurement (sensor_id, ts, value) VALUES (?, ?, ?)");
            BatchStatementBuilder builder = new BatchStatementBuilder(BatchType.UNLOGGED);

            List<Measure> ms = new ArrayList<>();
            Instant prev = Instant.now();

            while (true) {
                while (Duration.between(prev, Instant.now()).compareTo(config.bufferInterval) < 0) {
                    if (!sleep(config.measurement)) {
                        return;
                    }

                    for (com.carepet.model.Sensor s : sensors) {
                        Measure m = readSensorData(s);
                        ms.add(m);
                        LOG.info(m.toString());
                    }
                }

                prev = prev.plusMillis((Duration.between(prev, Instant.now()).toMillis()/config.bufferInterval.toMillis())*config.bufferInterval.toMillis());

                LOG.info("pushing data");
                // this is simplified example of batch execution. standard
                // best practice is to batch values that end up in the same partition:
                // https://www.scylladb.com/2019/03/27/best-practices-for-scylla-applications/
                for (Measure m: ms) {
                    builder = builder.addStatement(statement.bind(m.getSensorId(), m.getTs(), m.getValue()));
                }

                session.execute(builder.build());

                builder.clearStatements();
                ms.clear();
            }
        }
    }

    private boolean sleep(Duration d) {
        try {
            Thread.sleep(d.toMillis());
            return true;
        } catch (InterruptedException e) {
            return false;
        }
    }

    private Measure readSensorData(com.carepet.model.Sensor s) {
        return new Measure(s.getSensorId(), Instant.now(), s.randomData());
    }

    static class SensorConfig extends Config {
        @CommandLine.Option(names = {"--buffer-interval"}, description = "buffer to accumulate measures", defaultValue = "PT1H")
        Duration bufferInterval;

        @CommandLine.Option(names = {"--measure"}, description = "sensors measurement interval", defaultValue = "PT1M")
        Duration measurement;
    }
}
