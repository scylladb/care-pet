#include "json.hpp"
#include "model.hpp"
#include <boost/json.hpp>
#include <cassandra.h>

namespace boost {
namespace json {

void tag_invoke(value_from_tag, value& jv, const Owner& o) {
    char id_str[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(o.id, id_str);
    jv = {{"id", id_str}, {"name", o.name}, {"address", o.address}};
}

void tag_invoke(value_from_tag, value& jv, const Pet& p) {
    char owner_id_str[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(p.owner_id, owner_id_str);
    char pet_id_str[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(p.id, pet_id_str);
    jv = {
        {"owner_id", owner_id_str}, {"id", pet_id_str}, {"chip_id", p.chip_id},
        {"species", p.species},     {"breed", p.breed}, {"color", p.color},
        {"gender", p.gender},       {"age", p.age},     {"weight", p.weight},
        {"address", p.address},     {"name", p.name}};
}

void tag_invoke(value_from_tag, value& jv, const Sensor& s) {
    char pet_id_str[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(s.pet_id, pet_id_str);
    char sensor_id_str[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(s.id, sensor_id_str);
    jv = {{"pet_id", pet_id_str}, {"id", sensor_id_str}, {"type", s.type}};
}

void tag_invoke(value_from_tag, value& jv, const Measure& m) {
    char sensor_id_str[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(m.sensor_id, sensor_id_str);
    jv = {
        {"sensor_id", sensor_id_str},
        {"ts", m.ts},
        {"value", m.value},
    };
}

void tag_invoke(value_from_tag, value& jv, const SensorAvg& sa) {
    char sensor_id_str[CASS_UUID_STRING_LENGTH];
    cass_uuid_string(sa.sensor_id, sensor_id_str);
    jv = {
        {"sensor_id", sensor_id_str},
        {"date", sa.date},
        {"value", sa.value},
    };
}

} // namespace json
} // namespace boost
