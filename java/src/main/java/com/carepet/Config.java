package com.carepet;

import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;
import picocli.CommandLine;
import picocli.CommandLine.Option;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.InetSocketAddress;
import java.net.URI;
import java.net.URISyntaxException;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.UUID;
import java.util.stream.Collectors;

import static com.carepet.util.Wrapper.*;

public class Config {
    final static String applicationName = "care-pet";

    final static UUID clientId = UUID.randomUUID();

    final static String keyspace = "carepet";

    private final static int port = 9042;

    @Option(names = {"--hosts"}, description = "database contact points")
    String[] hosts;

    @Option(names = {"-dc", "--datacenter"}, description = "local datacenter name for default profile")
    String datacenter;

    @Option(names = {"-u", "--username"}, description = "password based authentication username")
    String username;

    @Option(names = {"-p", "--password"}, description = "password based authentication password")
    String password;

    @Option(names = { "-h", "--help" }, usageHelp = true, description = "display a help message")
    private boolean help = false;

    /**
     * Parses arguments into a new instance of the {@link Config} object.
     */
    public static Config parse(String[] args) {
        Config config = new Config();

        CommandLine cmd = new CommandLine(config);
        cmd.setUnmatchedArgumentsAllowed(false);
        cmd.parseArgs(args);

        if (cmd.isUsageHelpRequested()) {
            cmd.usage(System.err);
            System.exit(1);
        }

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
                .withClientId(clientId);

        if (hosts != null && hosts.length > 0) {
            builder = builder
                    .addContactPoints(Arrays.stream(hosts).map(unwrap(Config::resolve)).collect(Collectors.toList()))
                    .withLocalDatacenter(datacenter);
        }

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
        return unwrap(Config::getResourceFileAsString).apply(name);
    }

    /**
     * Reads given resource file as a string.
     */
    private static String getResourceFileAsString(String name) throws IOException {
        ClassLoader classLoader = ClassLoader.getSystemClassLoader();
        try (InputStream input = classLoader.getResourceAsStream(name)) {
            if (input == null) return null;
            try (InputStreamReader isr = new InputStreamReader(input, StandardCharsets.UTF_8)) {
                BufferedReader reader = new BufferedReader(isr);
                return reader.lines().collect(Collectors.joining(System.lineSeparator()));
            }
        }
    }
}
