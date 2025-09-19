#include <chrono>
#include <stdexcept>
#include <string>

#include "database.hpp"
#include <cassandra.h>

static std::string future_error_message(CassFuture* future) {
    const char* message;
    size_t message_length;
    cass_future_error_message(future, &message, &message_length);
    return std::string(message, message_length);
}

static const char* cass_value_type_name(CassValueType type) {

#define CASS_VALUE_TYPE_CASE(name, type, cql, klass)                           \
    case name:                                                                 \
        return cql;

    switch (type) {
        CASS_VALUE_TYPE_MAPPING(CASS_VALUE_TYPE_CASE);
    case CASS_VALUE_TYPE_UNKNOWN:
        return "unknown";
    default:
        return "invalid CASS_VALUE_TYPE";
    }
#undef CASS_VALUE_TYPE_CASE
}

static inline constexpr void
assert_deser_success(CassError error, const std::string_view type_name,
                     const CassValue* value) {
    if (error != CASS_OK) {
        throw std::runtime_error(
            std::format("Can't deserialize value to {}: {}. Value type: {}",
                        type_name, cass_error_desc(error),
                        cass_value_type_name(cass_value_type(value))));
    }
}

static inline constexpr void
assert_deser_column_non_null(const CassValue* value) {
    if (value == nullptr) {
        throw std::runtime_error(
            "Column for deserialization out of bounds. This "
            "should be prevented by check in `Rows` "
            "constructor, so a bug is likely.");
    }
}

template <>
CassError bind_to_statement(CassStatement* statement, size_t index,
                            const int32_t& value) {
    return cass_statement_bind_int32(statement, index, value);
}

template <>
CassError bind_to_statement(CassStatement* statement, size_t index,
                            const int64_t& value) {
    return cass_statement_bind_int64(statement, index, value);
}

template <>
CassError bind_to_statement(CassStatement* statement, size_t index,
                            const float& value) {
    return cass_statement_bind_float(statement, index, value);
}

template <>
CassError bind_to_statement(CassStatement* statement, size_t index,
                            const std::string& value) {
    return cass_statement_bind_string_n(statement, index, value.c_str(),
                                        value.length());
}

template <>
CassError bind_to_statement(CassStatement* statement, size_t index,
                            const CassUuid& value) {
    return cass_statement_bind_uuid(statement, index, value);
}

template <>
CassError bind_to_statement(CassStatement* statement, size_t index,
                            const std::chrono::year_month_day& value) {
    auto const tp_midnight = std::chrono::sys_days{value};
    // Just to be sure that leap seconds and other calculation differences
    // between chrono and driver and scylla don't change the day.
    auto const tp_noon = tp_midnight + std::chrono::hours{12};
    int64_t timestamp = std::chrono::duration_cast<std::chrono::seconds>(
                            tp_noon.time_since_epoch())
                            .count();
    uint32_t cass_date_value = cass_date_from_epoch(timestamp);

    return cass_statement_bind_uint32(statement, index, cass_date_value);
}

template <> int32_t deserialize_cass_value(const CassValue* value) {
    assert_deser_column_non_null(value);
    int32_t i;
    CassError err = cass_value_get_int32(value, &i);
    assert_deser_success(err, "int32", value);
    return i;
}

template <> int64_t deserialize_cass_value(const CassValue* value) {
    assert_deser_column_non_null(value);
    int64_t i;
    CassError err = cass_value_get_int64(value, &i);
    assert_deser_success(err, "int64", value);
    return i;
}

template <> float deserialize_cass_value(const CassValue* value) {
    assert_deser_column_non_null(value);
    float i;
    CassError err = cass_value_get_float(value, &i);
    assert_deser_success(err, "float", value);
    return i;
}

template <> double deserialize_cass_value(const CassValue* value) {
    assert_deser_column_non_null(value);
    double i;
    CassError err = cass_value_get_double(value, &i);
    assert_deser_success(err, "double", value);
    return i;
}

template <> CassUuid deserialize_cass_value(const CassValue* value) {
    assert_deser_column_non_null(value);
    CassUuid i;
    CassError err = cass_value_get_uuid(value, &i);
    assert_deser_success(err, "uuid", value);
    return i;
}

template <> std::string deserialize_cass_value(const CassValue* value) {
    assert_deser_column_non_null(value);
    const char* str;
    size_t len;
    CassError err = cass_value_get_string(value, &str, &len);
    assert_deser_success(err, "std::string", value);
    std::string i(str, len);
    return i;
}

Database::Database(const boost::program_options::variables_map& vm) {
    _cluster = cass_cluster_new();
    _session = cass_session_new();
    std::string host = vm["scylla-host"].as<std::string>();
    cass_cluster_set_contact_points(_cluster, host.c_str());

    CassFuture* connect_future = cass_session_connect(_session, _cluster);
    if (cass_future_error_code(connect_future) != CASS_OK) {
        auto message = future_error_message(connect_future);
        cass_future_free(connect_future);
        throw std::runtime_error(message);
    }
    cass_future_free(connect_future);
}

Database::~Database() {
    if (_cluster) {
        cass_cluster_free(_cluster);
    }
    if (_session) {
        cass_session_free(_session);
    }
}

QueryResult Database::execute_raw(const CassStatement* statement) {
    CassFuture* result_future = cass_session_execute(_session, statement);
    bool success = (cass_future_error_code(result_future) == CASS_OK);

    if (!success) {
        auto message = future_error_message(result_future);
        cass_future_free(result_future);
        throw std::runtime_error(message);
    }

    const CassResult* cass_result = cass_future_get_result(result_future);
    cass_future_free(result_future);

    return QueryResult(cass_result);
}

PreparedStatement Database::prepare(const char* query) {
    CassFuture* fut = cass_session_prepare(this->_session, query);
    bool success = (cass_future_error_code(fut) == CASS_OK);
    if (!success) {
        auto message = future_error_message(fut);
        cass_future_free(fut);
        throw std::runtime_error(message);
    }

    const CassPrepared* prepared = cass_future_get_prepared(fut);
    cass_future_free(fut);
    return PreparedStatement(prepared);
}
