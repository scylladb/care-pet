#include <cassandra.h>
#include <chrono>
#include <iostream>
#include <string>
#include <thread>

#include "database.hpp"
#include "model.hpp"
#include "sensor.hpp"

static void insert_owner(Database& db, const Owner& owner) {
    const char* query =
        "INSERT INTO carepet.owner (owner_id, name, address) VALUES (?, ?, ?)";
    Statement statement(query);
    db.execute(statement, owner.id, owner.name, owner.address);
}

static void insert_pet(Database& db, const Pet& pet) {
    const char* query =
        "INSERT INTO carepet.pet (owner_id, pet_id, chip_id, species, breed, "
        "color, gender, age, weight, address, name) VALUES (?, ?, ?, ?, ?, ?, "
        "?, "
        "?, ?, ?, ?)";
    Statement statement(query);
    db.execute(statement, pet.owner_id, pet.id, pet.chip_id, pet.species,
               pet.breed, pet.color, pet.gender, pet.age, pet.weight,
               pet.address, pet.name);
}

static void insert_sensor(Database& db, const Sensor& sensor) {
    const char* query =
        "INSERT INTO carepet.sensor (pet_id, sensor_id, type) VALUES (?, ?, ?)";
    Statement statement(query);
    db.execute(statement, sensor.pet_id, sensor.id, sensor.type);
}

static void insert_measure(Database& db, const PreparedStatement& statement,
                           const Measure& measure) {
    db.execute(statement, measure.sensor_id, measure.ts, measure.value);
}

void run_sensor(const boost::program_options::variables_map& vm) {
    Database db(vm);
    char uuid_str[CASS_UUID_STRING_LENGTH];

    CassUuidGen* uuid_gen = cass_uuid_gen_new();
    CassUuid owner_id, pet_id, temp_sensor_id, pulse_sensor_id;
    cass_uuid_gen_time(uuid_gen, &owner_id);
    cass_uuid_gen_time(uuid_gen, &pet_id);
    cass_uuid_gen_time(uuid_gen, &temp_sensor_id);
    cass_uuid_gen_time(uuid_gen, &pulse_sensor_id);

    Owner owner{.id = owner_id, .name = "John Doe", .address = "123 Main St"};
    insert_owner(db, owner);

    cass_uuid_string(owner_id, uuid_str);
    std::cout << "Owner id: " << uuid_str << "\n";

    Pet pet{
        .id = pet_id,
        .owner_id = owner.id,
        .chip_id = "1234-5678-9012",
        .species = "Dog",
        .breed = "Golden Retriever",
        .color = "Golden",
        .gender = "Male",
        .age = 5,
        .weight = 30,
        .address = "123 Main St",
        .name = "Fido",
    };
    insert_pet(db, pet);
    cass_uuid_string(pet_id, uuid_str);
    std::cout << "Pet id: " << uuid_str << "\n";

    Sensor temp_sensor{
        .id = temp_sensor_id, .pet_id = pet.id, .type = "Temperature"};
    insert_sensor(db, temp_sensor);
    cass_uuid_string(temp_sensor_id, uuid_str);
    std::cout << "Temperature sensor id: " << uuid_str << "\n";

    Sensor pulse_sensor{
        .id = pulse_sensor_id, .pet_id = pet.id, .type = "Pulse"};
    insert_sensor(db, pulse_sensor);
    cass_uuid_string(pulse_sensor_id, uuid_str);
    std::cout << "Pulse sensor id: " << uuid_str << "\n";

    PreparedStatement insert_measure_stmt =
        db.prepare("INSERT INTO carepet.measurement (sensor_id, ts, "
                   "value) VALUES (?, ?, ?)");

    auto start_time = std::chrono::high_resolution_clock::now();

    int seconds = vm["seconds"].as<int>();
    while (std::chrono::high_resolution_clock::now() - start_time <
           std::chrono::seconds(seconds)) {
        Measure temp_measure{
            .sensor_id = temp_sensor.id,
            .ts = std::chrono::duration_cast<std::chrono::milliseconds>(
                      std::chrono::system_clock::now().time_since_epoch())
                      .count(),
            .value = static_cast<float>(35.0 + (rand() / (RAND_MAX / 5.0)))};
        insert_measure(db, insert_measure_stmt, temp_measure);

        Measure pulse_measure{
            .sensor_id = pulse_sensor.id,
            .ts = std::chrono::duration_cast<std::chrono::milliseconds>(
                      std::chrono::system_clock::now().time_since_epoch())
                      .count(),
            .value = static_cast<float>(60.0 + (rand() / (RAND_MAX / 40.0)))};
        insert_measure(db, insert_measure_stmt, pulse_measure);

        std::this_thread::sleep_for(std::chrono::seconds(1));
    }
}
