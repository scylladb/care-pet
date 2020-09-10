package com.carepet.server;

import com.carepet.Config;
import io.micronaut.runtime.Micronaut;
import io.swagger.v3.oas.annotations.ExternalDocumentation;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Contact;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.info.License;
import io.swagger.v3.oas.annotations.servers.Server;

@OpenAPIDefinition(
        info = @Info(
                title = "CarePet",
                version = "0.1",
                description = "CarePet: An Example IoT Use Case for Hands-On App Developers",
                license = @License(name = "Apache 2.0", url = "https://github.com/scylladb/care-pet/blob/master/LICENSE"),
                contact = @Contact(url = "https://github.com/scylladb/care-pet")
        ),
        externalDocs = @ExternalDocumentation(url = "https://scylladb.github.io/care-pet/master/index.html"),
        servers = {@Server(description = "CarePet: An Example IoT Use Case for Hands-On App Developers")}
)
public class App {
    public static void main(String[] args) {
        Config config = Config.parse(new Config(), args);
        Micronaut.build(args).
                mainClass(App.class).
                singletons(config, config.builder(Config.keyspace).build()).
                start();
    }
}
