Build an IoT App with Go
------------------------

### Architecture

In this section, we will go through the code for the different commands.
As explained in the Getting Started page, The project is structured as follow:
- Migrate (/cmd/migrate) - creates the CarePet keyspace and tables
- Collar (/cmd/sensor) - generates a pet health data and pushes it into the storage
- Server (/cmd/server) - REST API service for tracking the pets’ health state



### Migrate

The `/migrate` command creates the keyspace and tables that will be used by the collar and server services.

Line 25 to 27 in the `/cmd/migrate/migrate.go` file call the `createKeyspace` , `migrateKeyspace` then the `printKeyspaceMetadata` functions.

```
func main() {
	///

	createKeyspace()
	migrateKeyspace()
	printKeyspaceMetadata()
}
```

The `createKeyspace` function creates a new session then executes the following CQL query stored in the  `db.go` file:

```
CREATE KEYSPACE IF NOT EXISTS carepet WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '3' } AND durable_writes = TRUE;
```

```
func createKeyspace() {
        // Creates a new session
	ses, err := config.Session()
	if err != nil {
		log.Fatalln("session: ", err)
	}
	defer ses.Close()

        // Executes the CREATE KEYSPACE query and checks for errors
	if err := ses.Query(db.KeySpaceCQL).Exec(); err != nil {
		log.Fatalln("ensure keyspace exists: ", err)
	}
}
```

The `migrateKeyspace` function opens a new session with the `carepet` keyspace and creates the tables using the CQL file located in `/db/cql/care-pet-ddl.cql`.

```
func migrateKeyspace() {
        // Create a new session with the carepet keyspace 
	ses, err := config.Keyspace()
	if err != nil {
		log.Fatalln("session: ", err)
	}
	defer ses.Close()

        // Execute the queries in the migration file om db/cql
	if err := migrate.Migrate(context.Background(), ses, "db/cql"); err != nil {
		log.Fatalln("migrate: ", err)
	}
}
```