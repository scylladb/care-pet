#pragma once

#include <cassandra.h>
#include <cstdint>
#include <string>

// Convert a CassUuid to its canonical "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" string.
inline std::string uuid_to_string(CassUuid uuid) {
    char buf[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(uuid, buf);
    return {buf};
}

struct Owner {
    CassUuid    owner_id{};
    std::string address;
    std::string name;
};

struct Pet {
    CassUuid    owner_id{};
    CassUuid    pet_id{};
    std::string chip_id;
    std::string species;
    std::string breed;
    std::string color;
    std::string gender;
    int32_t     age    = 0;
    float       weight = 0.0f;
    std::string address;
    std::string name;
};

struct Sensor {
    CassUuid    pet_id{};
    CassUuid    sensor_id{};
    std::string type; // "T" temperature, "P" pulse, "R" respiration, "L" location
};

struct Measurement {
    CassUuid sensor_id{};
    int64_t  ts    = 0;   // milliseconds since Unix epoch  (CQL TIMESTAMP)
    float    value = 0.0f;
};

struct SensorAvg {
    CassUuid sensor_id{};
    uint32_t date  = 0;   // days since Unix epoch           (CQL DATE)
    int32_t  hour  = 0;
    float    value = 0.0f;
};
