#pragma once

#include <cassandra.h>
#include <string>

struct Owner {
    CassUuid id;
    std::string name;
    std::string address;
};

struct Pet {
    CassUuid id;
    CassUuid owner_id;
    std::string chip_id;
    std::string species;
    std::string breed;
    std::string color;
    std::string gender;
    int32_t age;
    float weight;
    std::string address;
    std::string name;
};

struct Sensor {
    CassUuid id;
    CassUuid pet_id;
    std::string type;
};

struct Measure {
    CassUuid sensor_id;
    cass_int64_t ts;
    float value;
};

struct SensorAvg {
    CassUuid sensor_id;
    std::string date;
    float value;
};
