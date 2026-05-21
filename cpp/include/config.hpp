#pragma once

#include <cassandra.h>
#include <cstdlib>
#include <stdexcept>
#include <string>

// ---------------------------------------------------------------------------
// Config: connection parameters, populated from env vars then CLI args.
// ---------------------------------------------------------------------------
struct Config {
    std::string hosts    = "127.0.0.1";
    std::string username;
    std::string password;
    std::string dc;
    int         port     = 9042;

    // Populate from environment variables first, then override with argv.
    // Accepted flags: --hosts / -hosts, --username, --password, --dc, --port
    void parse_args(int argc, char** argv) {
        if (const char* e = std::getenv("SCYLLADB_HOSTS"))    hosts    = e;
        if (const char* e = std::getenv("SCYLLADB_USERNAME")) username = e;
        if (const char* e = std::getenv("SCYLLADB_PASSWORD")) password = e;

        for (int i = 1; i + 1 < argc; ++i) {
            std::string a = argv[i];
            while (!a.empty() && a.front() == '-') a.erase(0, 1);
            if      (a == "hosts"    || a == "h") hosts    = argv[++i];
            else if (a == "username")             username = argv[++i];
            else if (a == "password")             password = argv[++i];
            else if (a == "dc")                   dc       = argv[++i];
            else if (a == "port")                 port     = std::stoi(argv[++i]);
        }
    }

    bool is_cloud() const {
        return hosts.find(".clusters.scylla.cloud") != std::string::npos;
    }
};

// ---------------------------------------------------------------------------
// Session: RAII wrapper around CassCluster + CassSession.
// ---------------------------------------------------------------------------
class Session {
public:
    Session() : cluster_(cass_cluster_new()), session_(cass_session_new()) {}

    ~Session() {
        if (session_) {
            CassFuture* f = cass_session_close(session_);
            cass_future_wait(f);
            cass_future_free(f);
            cass_session_free(session_);
            session_ = nullptr;
        }
        if (cluster_) {
            cass_cluster_free(cluster_);
            cluster_ = nullptr;
        }
    }

    Session(const Session&)            = delete;
    Session& operator=(const Session&) = delete;

    // Connect without selecting a keyspace.
    void connect(const Config& cfg) {
        apply_config(cfg);
        check(cass_session_connect(session_, cluster_), "connect");
    }

    // Connect and select a keyspace.
    void connect_keyspace(const Config& cfg, const std::string& ks) {
        apply_config(cfg);
        check(cass_session_connect_keyspace(session_, cluster_, ks.c_str()),
              "connect_keyspace(" + ks + ")");
    }

    // Execute a DDL / DML statement that returns no rows.
    void execute(const std::string& query) {
        auto* stmt = cass_statement_new(query.c_str(), 0);
        auto* f    = cass_session_execute(session_, stmt);
        cass_statement_free(stmt);
        check(f, query.substr(0, 60));
    }

    // Return the raw session pointer for use with the C driver API directly.
    CassSession* get() { return session_; }

private:
    CassCluster* cluster_;
    CassSession* session_;

    void apply_config(const Config& cfg) {
        cass_cluster_set_contact_points(cluster_, cfg.hosts.c_str());
        cass_cluster_set_port(cluster_, cfg.port);
        if (!cfg.username.empty())
            cass_cluster_set_credentials(cluster_,
                                         cfg.username.c_str(),
                                         cfg.password.c_str());
    }

    // Wait for future, always free it, throw on error.
    static void check(CassFuture* f, const std::string& ctx) {
        cass_future_wait(f);
        CassError rc = cass_future_error_code(f);
        std::string err;
        if (rc != CASS_OK) {
            const char* msg; size_t len;
            cass_future_error_message(f, &msg, &len);
            err = std::string(msg, len);
        }
        cass_future_free(f);
        if (rc != CASS_OK)
            throw std::runtime_error(ctx + ": " + err);
    }
};
