package com.carepet;

import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;
import picocli.CommandLine;
import picocli.CommandLine.Option;

import java.net.InetSocketAddress;
import java.net.URI;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.UUID;
import java.util.stream.Collectors;

import static com.carepet.util.Wrapper.unwrap;
import static com.carepet.util.Wrapper.unwrap0;

public class Config {
    final static String applicationName = "care-pet";

    final static UUID clientId = UUID.randomUUID();

    final static String keyspace = "carepet";

    @Option(names = {"-h", "--hosts"}, description = "database contact points", defaultValue = "127.0.0.1")
    String[] hosts;

    @Option(names = {"-u", "--username"}, description = "password based authentication username")
    String username;

    @Option(names = {"-p", "--password"}, description = "password based authentication password")
    String password;

    /**
     * Parses arguments into a new instance of the {@link Config} object.
     */
    public static Config parse(String[] args) {
        final Config config = new Config();

        new CommandLine(config).parseArgs(args);

        return config;
    }

    /**
     * Builds configured CqlSession builder to acquire a new session.
     */
    public CqlSessionBuilder builder() {
        return builder("");
    }

    /**
     * Builds configured CqlSession builder to acquire a new session.
     */
    public CqlSessionBuilder builder(String keyspace) {
        CqlSessionBuilder builder = CqlSession.builder()
                .withApplicationName(applicationName)
                .withClientId(clientId)
                .addContactPoints(Arrays.stream(hosts).map(unwrap(Config::resolve)).collect(Collectors.toList()));

        if (!username.isEmpty()) {
            builder = builder.withAuthCredentials(username, password);
        }

        if (!keyspace.isEmpty()) {
            builder = builder.withKeyspace(keyspace);
        }

        return builder;
    }

    /**
     * Transforms an address of the form host:port into an InetSocketAddress.
     */
    public static InetSocketAddress resolve(String addr) throws URISyntaxException {
        URI uri = new URI("my://" + addr);
        String host = uri.getHost();
        int port = uri.getPort();

        if (uri.getHost() == null || uri.getPort() == -1) {
            throw new URISyntaxException(uri.toString(), "URI must have host and port");
        }

        return new InetSocketAddress (host, port);
    }

    /**
     * Loads a resource content.
     */
    public static String getResource(String name) {
        final URL res = ClassLoader.getSystemClassLoader().getResource(name);
        return new String(unwrap(Files::readAllBytes).apply(Paths.get(unwrap0(res::toURI).get())));
    }
}
