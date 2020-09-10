package com.carepet;

import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.metadata.schema.KeyspaceMetadata;
import com.datastax.oss.driver.api.core.metadata.schema.TableMetadata;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Migrate {
    private static final Logger LOG = LoggerFactory.getLogger(Migrate.class);
    private final Config config;

    public Migrate(Config config) {
        this.config = config;
    }

    public static void main(String[] args) {
        final Config config = Config.parse(new Config(), args);

        final Migrate client = new Migrate(config);
        client.createKeyspace();
        client.createSchema();
        client.printMetadata();
    }

    /**
     * Initiates a connection to the session specified by the application.conf.
     */
    public CqlSession connect() {
        return config.builder().build();
    }

    /**
     * Initiates a connection to the session specified by the application.conf.
     */
    public CqlSession keyspace() {
        return config.builder(Config.keyspace).build();
    }

    /**
     * Creates the keyspace for this example.
     */
    public void createKeyspace() {
        LOG.info("creating keyspace...");
        try (CqlSession session = connect()) {
            session.execute(Config.getResource("care-pet-keyspace.cql"));
        }
    }

    /**
     * Creates the tables for this example.
     */
    public void createSchema() {
        LOG.info("creating table...");
        try (CqlSession session = keyspace()) {
            for (String cql : Config.getResource("care-pet-ddl.cql").split(";")) {
                session.execute(cql);
            }
        }
    }

    /**
     * Prints keyspace metadata.
     */
    public void printMetadata() {
        try (CqlSession session = keyspace()) {
            KeyspaceMetadata keyspace = session.getMetadata().getKeyspace(Config.keyspace).get();
            for (TableMetadata table : keyspace.getTables().values()) {
                System.out.printf("Keyspace: %s; Table: %s%n", keyspace.getName(), table.getName());
            }
        }
    }
}
