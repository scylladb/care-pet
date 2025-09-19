#pragma once

#include "model.hpp"
#include <boost/json.hpp>
#include <cassandra.h>

namespace boost {
namespace json {

void tag_invoke(value_from_tag, value& jv, const Owner& o);
void tag_invoke(value_from_tag, value& jv, const Pet& p);
void tag_invoke(value_from_tag, value& jv, const Sensor& s);
void tag_invoke(value_from_tag, value& jv, const Measure& m);
void tag_invoke(value_from_tag, value& jv, const SensorAvg& sa);

} // namespace json
} // namespace boost
