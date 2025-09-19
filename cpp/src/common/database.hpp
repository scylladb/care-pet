#pragma once

#include <boost/core/demangle.hpp>
#include <boost/program_options.hpp>
#include <cassandra.h>
#include <format>
#include <optional>
#include <stdexcept>
#include <tuple>
#include <utility>

template <typename T> T deserialize_cass_value(const CassValue* value);

class Statement {
  public:
    friend class Database;

    Statement(const char* query_str) {
        this->inner = cass_statement_new(query_str, 0);
    }

    Statement(CassStatement* statement) { this->inner = statement; }

    ~Statement() { cass_statement_free(this->inner); }

  private:
    CassStatement* inner;
};

class PreparedStatement {
  public:
    friend class Database;

    PreparedStatement(const CassPrepared* prepared) { this->inner = prepared; }

    ~PreparedStatement() { cass_prepared_free(this->inner); }

  private:
    const CassPrepared* inner;
};

template <typename... Types, std::size_t... Is>
static std::tuple<Types...> next_row_impl(const CassRow* row,
                                          std::index_sequence<Is...>) {
    return std::make_tuple<Types...>(
        deserialize_cass_value<Types>(cass_row_get_column(row, Is))...);
}

template <typename... Types> class Rows {
  public:
    friend class QueryResult;

    Rows(const CassResult* result) {
        if (cass_result_column_count(result) != sizeof...(Types)) {
            throw std::runtime_error(std::format(
                "Invalid column count in response expected {} found {}",
                sizeof...(Types), cass_result_column_count(result)));
        }
        this->iterator = cass_iterator_from_result(result);
    }

    ~Rows() { cass_iterator_free(this->iterator); }

    std::optional<std::tuple<Types...>> next_row() {
        if (!cass_iterator_next(this->iterator)) {
            return std::nullopt;
        }
        const CassRow* row = cass_iterator_get_row(this->iterator);

        return std::make_optional<std::tuple<Types...>>(
            next_row_impl<Types...>(row, std::index_sequence_for<Types...>{}));
    }

  private:
    CassIterator* iterator;
};

class QueryResult {
  public:
    QueryResult(const CassResult* result) { this->inner = result; }

    template <typename... Types> Rows<Types...> rows() {
        return Rows<Types...>(this->inner);
    }

    ~QueryResult() { cass_result_free(this->inner); }

  private:
    const CassResult* inner;
};

static inline constexpr void assert_ser_success(CassError error,
                                                const char* name) {
    if (error != CASS_OK) {
        throw std::runtime_error(
            std::format("Failed to serialize value of type {}. Error: {}",
                        boost::core::demangle(name), cass_error_desc(error)));
    }
}

template <typename T>
CassError bind_to_statement(CassStatement* statement, size_t index,
                            const T& value);

class Database {
  public:
    Database(const boost::program_options::variables_map& vm);

    Database(const Database& other) = delete;

    Database(Database&& other) {
        this->_cluster = other._cluster;
        other._cluster = nullptr;
        this->_session = other._session;
        other._session = nullptr;
    };

    ~Database();

    template <typename... Args>
    QueryResult execute(Statement& statement, Args... args) {
        CassStatement* c_statement = statement.inner;
        cass_statement_reset_parameters(c_statement, sizeof...(Args));
        size_t bind_idx = 0;
        (assert_ser_success(bind_to_statement(c_statement, bind_idx++, args),
                            typeid(args).name()),
         ...);
        return this->execute_raw(c_statement);
    }

    PreparedStatement prepare(const char* query_str);

    template <typename... Args>
    QueryResult execute(const PreparedStatement& statement, Args... args) {
        CassStatement* c_statement = cass_prepared_bind(statement.inner);
        size_t bind_idx = 0;
        (assert_ser_success(bind_to_statement(c_statement, bind_idx++, args),
                            typeid(args).name()),
         ...);
        return this->execute_raw(c_statement);
    }

  private:
    QueryResult execute_raw(const CassStatement* statement);
    CassCluster* _cluster = nullptr;
    CassSession* _session = nullptr;
};
