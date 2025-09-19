#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>
#include <boost/json.hpp>
#include <boost/url.hpp>
#include <cassandra.h>
#include <chrono>
#include <iostream>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <vector>

#include "database.hpp"
#include "handlers.hpp"
#include "json.hpp"
#include "model.hpp"

namespace beast = boost::beast;
namespace http = boost::beast::http;

class ResponseFactory {
  public:
    ResponseFactory(const http::request<http::string_body>& req) : req(req) {}

    ~ResponseFactory() = default;

    http::response<http::string_body>
    serverError(beast::string_view why) const {
        http::response<http::string_body> res{
            http::status::internal_server_error, req.version()};
        res.set(http::field::server, BOOST_BEAST_VERSION_STRING);
        res.set(http::field::content_type, "text/html");
        res.keep_alive(req.keep_alive());
        res.body() = std::string(why);
        res.prepare_payload();
        return res;
    }

    http::response<http::string_body> badRequest(beast::string_view why) const {
        http::response<http::string_body> res{http::status::bad_request,
                                              req.version()};
        res.set(http::field::server, BOOST_BEAST_VERSION_STRING);
        res.set(http::field::content_type, "text/html");
        res.keep_alive(req.keep_alive());
        res.body() = std::string(why);
        res.prepare_payload();
        return res;
    }

    http::response<http::string_body>
    notFound(beast::string_view target) const {
        http::response<http::string_body> res{http::status::not_found,
                                              req.version()};
        res.set(http::field::server, BOOST_BEAST_VERSION_STRING);
        res.set(http::field::content_type, "text/html");
        res.keep_alive(req.keep_alive());
        res.body() =
            "The resource '" + std::string(target) + "' was not found.";
        res.prepare_payload();
        return res;
    }

    http::response<http::string_body>
    apiResponse(boost::json::value body) const {
        http::response<http::string_body> res{http::status::ok, req.version()};
        res.set(http::field::server, BOOST_BEAST_VERSION_STRING);
        res.set(http::field::content_type, "application/json");
        res.keep_alive(req.keep_alive());
        res.body() = boost::json::serialize(body);
        res.prepare_payload();
        return res;
    }

  private:
    const http::request<http::string_body>& req;
};

class RequestHandler::Impl {
  public:
    Impl(Database db)
        : db(std::move(db)),
          fetch_owner(this->db.prepare("SELECT owner_id, name, address FROM "
                                       "carepet.owner WHERE owner_id = ?")),
          fetch_pets(this->db.prepare(
              "SELECT pet_id, owner_id, chip_id, species, "
              "breed, color, gender, age, weight, address, name "
              "FROM carepet.pet WHERE owner_id = ?")),
          fetch_sensors(
              this->db.prepare("SELECT sensor_id, pet_id, type "
                               "FROM carepet.sensor WHERE pet_id = ?")),
          fetch_measurements(
              this->db.prepare("SELECT ts, value FROM carepet.measurement "
                               "WHERE sensor_id = ? AND ts >= ? AND ts <= ?")),
          fetch_avg(
              this->db.prepare("SELECT hour, value FROM carepet.sensor_avg "
                               "WHERE sensor_id = ? AND date = ?")),
          insert_sensor_avg(this->db.prepare(
              "INSERT INTO carepet.sensor_avg "
              "(sensor_id, date, hour, value) VALUES (?, ?, ?, ?)")) {}

    ~Impl() = default;

    http::response<http::string_body>
    handle_get_owner(const http::request<http::string_body>& req,
                     const ResponseFactory& responses,
                     std::string owner_id_str);

    http::response<http::string_body>
    handle_get_pets(const http::request<http::string_body>& req,
                    const ResponseFactory& responses, std::string owner_id_str);

    http::response<http::string_body>
    handle_get_sensors(const http::request<http::string_body>& req,
                       const ResponseFactory& responses,
                       std::string pet_id_str);

    http::response<http::string_body>
    handle_get_measurements(const http::request<http::string_body>& req,
                            const ResponseFactory& responses,
                            std::string sensor_id_str, std::string from,
                            std::string to);

    http::response<http::string_body>
    handle_get_sensor_avg(const http::request<http::string_body>& req,
                          const ResponseFactory& responses,
                          std::string sensor_id_str, std::string date);

  private:
    void aggregate_missing_hours(
        CassUuid sensor_id,
        const std::chrono::time_point<std::chrono::system_clock>& now,
        const std::chrono::year_month_day& date, std::vector<float>& data);

    void group_by_hour(std::vector<float>& data,
                       const std::vector<Measure>& measures, int current_hour,
                       bool same_date);

    void save_aggregated_data(CassUuid sensor_id,
                              const std::chrono::year_month_day& date,
                              const std::vector<float>& data, int prev_avg_size,
                              bool same_date, int current_hour);

    Database db;
    PreparedStatement fetch_owner;
    PreparedStatement fetch_pets;
    PreparedStatement fetch_sensors;
    PreparedStatement fetch_measurements;
    PreparedStatement fetch_avg;
    PreparedStatement insert_sensor_avg;
};

RequestHandler::RequestHandler(Database db)
    : pImpl(std::make_unique<Impl>(std::move(db))) {}

RequestHandler::~RequestHandler() = default;

http::response<http::string_body>
RequestHandler::handle_request(const http::request<http::string_body>& req) {
    const ResponseFactory responseFactory(req);

    if (req.method() != http::verb::get) {
        return responseFactory.badRequest("Unknown HTTP-method");
    }

    boost::url_view url(req.target());
    boost::urls::segments_view path_segments_view = url.segments();
    std::vector<std::string> path_segments(path_segments_view.begin(),
                                           path_segments_view.end());
    // Ugly request routing - for such small example it doesn't make sense
    // to write something more sophisticated.

    // /owner/{owner_id}
    if (path_segments.size() == 2 && path_segments[0] == "owner") {
        return this->pImpl->handle_get_owner(req, responseFactory,
                                             path_segments[1]);
    }
    // /owner/{owner_id}/pets
    if (path_segments.size() == 3 && path_segments[0] == "owner" &&
        path_segments[2] == "pets") {
        return this->pImpl->handle_get_pets(req, responseFactory,
                                            path_segments[1]);
    }
    // /pet/{pet_id}/sensors
    if (path_segments.size() == 3 && path_segments[0] == "pet" &&
        path_segments[2] == "sensors") {
        return this->pImpl->handle_get_sensors(req, responseFactory,
                                               path_segments[1]);
    }
    // /sensors/{sensor_id}/values
    if (path_segments.size() == 3 && path_segments[0] == "sensors" &&
        path_segments[2] == "values") {
        auto params = url.params();
        auto from_iter = params.find("from"), to_iter = params.find("to");
        if (from_iter == params.end()) {
            return responseFactory.badRequest("No value for \"to\" parameter");
        }
        if (to_iter == params.end()) {
            return responseFactory.badRequest(
                "No value for \"from\" parameter");
        }
        std::string from((*from_iter).value), to((*to_iter).value);
        return this->pImpl->handle_get_measurements(req, responseFactory,
                                                    path_segments[1], from, to);
    }
    // /sensors/{sensor_id}/values/day/{date}
    if (path_segments.size() == 5 && path_segments[0] == "sensors" &&
        path_segments[2] == "values" && path_segments[3] == "day") {
        return this->pImpl->handle_get_sensor_avg(
            req, responseFactory, path_segments[1], path_segments[4]);
    }

    return responseFactory.notFound(req.target());
}

#define ASSERT_SUCCESS(ERR_EXPR, MESSAGE)                                      \
    do {                                                                       \
        CassError err = ERR_EXPR;                                              \
        if (err != CASS_OK) {                                                  \
            throw std::runtime_error(                                          \
                std::format("Operation failed. Error: {}. {}",                 \
                            cass_error_desc(err), MESSAGE));                   \
        }                                                                      \
    } while (0)

class ParsingError {};

std::optional<std::chrono::year_month_day>
parse_date(const std::string& date_str) {
    // Parse date using C++20 chrono
    std::istringstream in{date_str};
    std::chrono::year_month_day ymd;
    in >> std::chrono::parse("%F", ymd);
    if (in.fail() || !ymd.ok()) {
        return std::nullopt;
    }

    return ymd;
}

std::pair<cass_int64_t, cass_int64_t>
get_day_time_range(const std::chrono::year_month_day& date) {
    auto start_of_day = std::chrono::sys_days{date};
    auto end_of_day = start_of_day + std::chrono::hours(23) +
                      std::chrono::minutes(59) + std::chrono::seconds(59) +
                      std::chrono::milliseconds(999);

    return {start_of_day.time_since_epoch().count(),
            end_of_day.time_since_epoch().count()};
}

int get_hour_from_timestamp(cass_int64_t timestamp_ms) {
    auto tp = std::chrono::sys_time<std::chrono::milliseconds>{
        std::chrono::milliseconds{timestamp_ms}};
    auto dp = std::chrono::floor<std::chrono::days>(tp);
    auto time_of_day = std::chrono::hh_mm_ss{tp - dp};
    return time_of_day.hours().count();
}

int get_hour_from_time_point(
    const std::chrono::time_point<std::chrono::system_clock>& now) {
    auto dp = std::chrono::floor<std::chrono::days>(now);
    auto time_of_day = std::chrono::hh_mm_ss{now - dp};
    return time_of_day.hours().count();
}

std::optional<cass_int64_t> parse_iso_datetime(const std::string& iso_date) {
    std::istringstream in{iso_date};
    std::chrono::sys_time<std::chrono::milliseconds> tp;
    in >> std::chrono::parse("%FT%TZ", tp);
    if (in.fail()) {
        in.clear();
        in.exceptions(std::ios::failbit);
        in.str(iso_date);
        in >> std::chrono::parse("%FT%T%Ez", tp);
    }
    if (in.fail()) {
        return std::nullopt;
    }
    return tp.time_since_epoch().count();
}

std::optional<CassUuid> parse_uuid(const char* uuid_str) {
    CassUuid result;
    if (cass_uuid_from_string(uuid_str, &result) != CASS_OK) {
        return std::nullopt;
    };
    return result;
}

http::response<http::string_body> RequestHandler::Impl::handle_get_owner(
    const http::request<http::string_body>& req,
    const ResponseFactory& responses, std::string owner_id_str) {
    auto maybe_owner_id = parse_uuid(owner_id_str.c_str());
    if (!maybe_owner_id) {
        return responses.badRequest("Invalid owner id");
    }
    CassUuid owner_id = *maybe_owner_id;

    QueryResult result = db.execute(fetch_owner, owner_id);
    Rows rows = result.rows<CassUuid, std::string, std::string>();

    auto row = rows.next_row();
    if (!row) {
        return responses.badRequest("No owner with this id found");
    }
    auto [selected_owner_id, name, address] = *row;
    // We know there will be at most one row.

    Owner owner{.id = selected_owner_id, .name = name, .address = address};

    return responses.apiResponse(boost::json::value_from(owner));
}

http::response<http::string_body> RequestHandler::Impl::handle_get_pets(
    const http::request<http::string_body>& req,
    const ResponseFactory& responses, std::string owner_id_str) {
    auto maybe_owner_id = parse_uuid(owner_id_str.c_str());
    if (!maybe_owner_id) {
        return responses.badRequest("Invalid owner id");
    }
    CassUuid owner_id = *maybe_owner_id;
    QueryResult query_result = db.execute(fetch_pets, owner_id);
    std::vector<Pet> pets;
    Rows rows = query_result.rows<CassUuid, CassUuid, std::string, std::string,
                                  std::string, std::string, std::string,
                                  int32_t, float, std::string, std::string>();
    for (auto row = rows.next_row(); row; row = rows.next_row()) {
        auto [pet_id, owner_id, chip_id, species, breed, color, gender, age,
              weight, address, name] = *row;
        Pet pet{.id = pet_id,
                .owner_id = owner_id,
                .chip_id = chip_id,
                .species = species,
                .breed = breed,
                .color = color,
                .gender = gender,
                .age = age,
                .weight = weight,
                .address = address,
                .name = name};
        pets.push_back(pet);
    }

    return responses.apiResponse(boost::json::value_from(pets));
}

http::response<http::string_body> RequestHandler::Impl::handle_get_sensors(
    const http::request<http::string_body>& req,
    const ResponseFactory& responses, std::string pet_id_str) {
    auto maybe_pet_id = parse_uuid(pet_id_str.c_str());
    if (!maybe_pet_id) {
        return responses.badRequest("Invalid pet id");
    }
    CassUuid pet_id = *maybe_pet_id;

    QueryResult result = db.execute(fetch_sensors, pet_id);
    Rows rows = result.rows<CassUuid, CassUuid, std::string>();

    std::vector<Sensor> sensors;
    for (auto row = rows.next_row(); row; row = rows.next_row()) {
        auto [sensor_id, pet_id, type] = *row;
        Sensor sensor{
            .id = sensor_id,
            .pet_id = pet_id,
            .type = type,
        };
        sensors.push_back(sensor);
    }

    return responses.apiResponse(boost::json::value_from(sensors));
}

http::response<http::string_body> RequestHandler::Impl::handle_get_measurements(
    const http::request<http::string_body>& req,
    const ResponseFactory& responses, std::string sensor_id_str,
    std::string from_str, std::string to_str) {

    auto maybe_sensor_id = parse_uuid(sensor_id_str.c_str());
    if (!maybe_sensor_id) {
        return responses.badRequest("Invalid sensor id");
    }
    CassUuid sensor_id = *maybe_sensor_id;

    auto maybe_from = parse_iso_datetime(from_str);
    if (!maybe_from) {
        return responses.badRequest("Invalid `from` date");
    }
    int64_t from = *maybe_from;

    auto maybe_to = parse_iso_datetime(to_str);
    if (!maybe_to) {
        return responses.badRequest("Invalid `to` date");
    }
    int64_t to = *maybe_to;

    QueryResult query_result =
        db.execute(fetch_measurements, sensor_id, from, to);
    Rows rows = query_result.rows<int64_t, float>();

    std::vector<Measure> measurements;
    for (auto row = rows.next_row(); row; row = rows.next_row()) {
        auto [ts, value] = *row;
        Measure m{.sensor_id = sensor_id, .ts = ts, .value = value};
        measurements.push_back(m);
    }

    return responses.apiResponse(boost::json::value_from(measurements));
}

http::response<http::string_body> RequestHandler::Impl::handle_get_sensor_avg(
    const http::request<http::string_body>& req,
    const ResponseFactory& responses, std::string sensor_id_str,
    std::string date_str) {
    auto maybe_sensor_id = parse_uuid(sensor_id_str.c_str());
    if (!maybe_sensor_id) {
        return responses.badRequest("Invalid sensor id");
    }
    CassUuid sensor_id = *maybe_sensor_id;

    auto now = std::chrono::system_clock::now();

    auto maybe_date = parse_date(date_str);
    if (!maybe_date) {
        return responses.badRequest("Invalid date or request into the future");
    }
    std::chrono::year_month_day requested_date = *maybe_date;

    {
        // Check if date is in the future
        auto today_days = std::chrono::floor<std::chrono::days>(now);
        auto requested_date_days = std::chrono::sys_days{requested_date};

        if (requested_date_days > today_days) {
            return responses.badRequest(
                "Can't get avearges for date in the future");
        }
    }

    QueryResult query_result = db.execute(fetch_avg, sensor_id, requested_date);
    Rows rows = query_result.rows<int32_t, float>();

    std::vector<float> data;
    for (auto row = rows.next_row(); row; row = rows.next_row()) {
        auto [hour, avg] = *row;
        if (hour != data.size()) {
            return responses.serverError(
                "Invalid cached averages data. Please drop avg data for this "
                "date in order to recalculate");
        }
        data.push_back(avg);
    }

    if (data.size() != 24) {
        aggregate_missing_hours(sensor_id, now, requested_date, data);
    }

    // Convert to SensorAvg for response
    std::vector<SensorAvg> sensor_avgs;
    for (size_t hour = 0; hour < data.size(); ++hour) {
        SensorAvg sensor_avg{
            .sensor_id = sensor_id, .date = date_str, .value = data[hour]};
        sensor_avgs.push_back(sensor_avg);
    }

    return responses.apiResponse(boost::json::value_from(sensor_avgs));
}

void RequestHandler::Impl::aggregate_missing_hours(
    CassUuid sensor_id,
    const std::chrono::time_point<std::chrono::system_clock>& now,
    const std::chrono::year_month_day& date, std::vector<float>& data) {

    std::chrono::year_month_day now_date =
        std::chrono::year_month_day{std::chrono::floor<std::chrono::days>(now)};
    auto [start_ts, end_ts] = get_day_time_range(date);

    QueryResult query_result =
        db.execute(fetch_measurements, sensor_id, start_ts, end_ts);
    Rows rows = query_result.rows<int64_t, float>();

    std::vector<Measure> measures;
    for (auto row = rows.next_row(); row; row = rows.next_row()) {
        auto [ts, value] = *row;
        Measure m{.sensor_id = sensor_id, .ts = ts, .value = value};
        measures.push_back(m);
    }

    int prev_avg_size = data.size();
    int current_hour = get_hour_from_time_point(now);
    bool same_day = now_date == date;
    group_by_hour(data, measures, current_hour, same_day);

    save_aggregated_data(sensor_id, date, data, prev_avg_size, same_day,
                         current_hour);
}

void RequestHandler::Impl::group_by_hour(std::vector<float>& data,
                                         const std::vector<Measure>& measures,
                                         int current_hour, bool same_date) {
    int start_hour = data.size();

    struct HourlyAvg {
        double value = 0.0;
        int total = 0;
    };

    // aggregate data by hour
    HourlyAvg hourly_agg[24];

    for (const auto& m : measures) {
        int hour = get_hour_from_timestamp(m.ts);

        HourlyAvg& a = hourly_agg[hour];
        a.total++;
        a.value += m.value;
    }

    // fill the averages
    for (int hour = start_hour;
         hour < 24 && (!same_date || hour <= current_hour); hour++) {
        HourlyAvg& a = hourly_agg[hour];
        if (a.total > 0) {
            data.push_back(a.value / a.total);
        } else {
            data.push_back(0.0);
        }
    }
}

void RequestHandler::Impl::save_aggregated_data(
    CassUuid sensor_id, const std::chrono::year_month_day& date,
    const std::vector<float>& data, int prev_avg_size, bool same_date,
    int current_hour) {
    for (int hour = prev_avg_size; hour < (int)data.size(); hour++) {
        if (same_date && hour >= current_hour) {
            break;
        }

        // Execute insert statement
        char sensor_id_str[CASS_UUID_STRING_LENGTH];
        cass_uuid_string(sensor_id, sensor_id_str);
        std::cout << std::format("Inserting average. Sensor: {}, date: {}, "
                                 "hour: {}, value: {}\n",
                                 sensor_id_str, date, hour, (float)data[hour]);
        db.execute(insert_sensor_avg, sensor_id, date, hour, (float)data[hour]);
    }
}
