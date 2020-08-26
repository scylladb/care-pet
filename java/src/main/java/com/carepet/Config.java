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

    private final static int port = 9042;

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
        return builder(null);
    }

    /**
     * Builds configured CqlSession builder to acquire a new session.
     */
    public CqlSessionBuilder builder(String keyspace) {
        CqlSessionBuilder builder = CqlSession.builder()
                .withApplicationName(applicationName)
                .withClientId(clientId)
                .addContactPoints(Arrays.stream(hosts).map(unwrap(Config::resolve)).collect(Collectors.toList()));

        if (!isNullOrEmpty(username)) {
            builder = builder.withAuthCredentials(username, password);
        }

        if (!isNullOrEmpty(keyspace)) {
            builder = builder.withKeyspace(keyspace);
        }

        return builder;
    }

    /**
     * Transforms an address of the form host:port into an InetSocketAddress.
     */
    public static InetSocketAddress resolve(String addr) throws URISyntaxException {
        URI uri = new URI("scylladb://" + withPort(addr, port));
        String host = uri.getHost();
        int port = uri.getPort();

        if (isNullOrEmpty(uri.getHost()) || uri.getPort() == -1) {
            throw new URISyntaxException(uri.toString(), "URI must have host and port");
        }

        return new InetSocketAddress (host, port);
    }

    /**
     * Ensures an address has port provided.
     */
    private static String withPort(String addr, int port) {
        if (!addr.contains(":")) {
            return addr + ":" + port;
        }

        return addr;
    }

    /**
     * Determine if a string is {@code null} or {@link String#isEmpty()} returns {@code true}.
     */
    public static boolean isNullOrEmpty(String s) {
        return s == null || s.isEmpty();
    }

    /**
     * Loads a resource content.
     */
    public static String getResource(String name) {
        final URL res = ClassLoader.getSystemClassLoader().getResource(name);
        return new String(unwrap(Files::readAllBytes).apply(Paths.get(unwrap0(res::toURI).get())));
    }
}
