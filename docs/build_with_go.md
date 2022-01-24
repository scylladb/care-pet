Build an IoT App with Go
------------------------

### Architecture

In this section, we will walk through and explain the code for the different commands.
As explained in the Getting Started page, the project is structured as follow:
- Migrate (/cmd/migrate) - creates the CarePet keyspace and tables
- Collar (/cmd/sensor) - generates a pet health data and pushes it into the storage
- Server (/cmd/server) - REST API service for tracking the pets’ health state



### Migrate

The `/migrate` command creates the keyspace and tables that will be used by the collar and server services.

Line 25 to 27 in the `/cmd/migrate/migrate.go` file call the `createKeyspace` , `migrateKeyspace` then the `printKeyspaceMetadata` functions.

```
func main() {
	
	/// ...

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

The `migrateKeyspace` function opens a new session with the `carepet` keyspace and creates the following tables in the carepet keyspace using the CQL file located in `/db/cql/care-pet-ddl.cql`:
- `owner`
- `pet`
- `sensor`
- `measurement`
- `sensor_avg`

```
func migrateKeyspace() {
        // Create a new session with the carepet keyspace 
	ses, err := config.Keyspace()
	if err != nil {
		log.Fatalln("session: ", err)
	}T
	defer ses.Close()

        // Execute the queries in the migration file om db/cql
	if err := migrate.Migrate(context.Background(), ses, "db/cql"); err != nil {
		log.Fatalln("migrate: ", err)
	}
}
```

As the name suggests, the `printKeyspaceMetadata` function will then print the metadata related to the `carepet` keyspace and confirm that the tables were properly created.

### Sensor

The sensor service simulates the collar's activity. The service uses the `pet struct` and its functions defined in `sensor/pet.go` to create a new `pet` along with an `owner` and `sensorType` then saves it to the database.

```
func main() {

	/// ...

	// Create a new session with carepet keyspace
	ses, err := config.Keyspace()
	if err != nil {
		log.Fatalln("session: ", err)
	}
	defer ses.Close()

	// Generate new pet
	pet := NewPet()

	// Save new pet to the database
	if err := pet.save(context.Background(), ses); err != nil {
		log.Fatalln("pet save: ", err)
	}

	log.Println("New owner #", pet.p.OwnerID)
	log.Println("New pet #", pet.p.PetID)

	pet.run(context.Background(), ses)
}
```

