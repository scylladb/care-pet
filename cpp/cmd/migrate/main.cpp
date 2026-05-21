#include "config.hpp"

#include <iostream>
#include <string>
#include <utility>
#include <vector>

// ---------------------------------------------------------------------------
// DDL statements executed in order.
// ---------------------------------------------------------------------------
static const std::string KEYSPACE_DDL =
    "CREATE KEYSPACE IF NOT EXISTS carepet"
    " WITH replication = {'class':'NetworkTopologyStrategy','replication_factor':'3'}"
    " AND durable_writes = TRUE";

static const std::vector<std::pair<std::string, std::string>> TABLE_DDLS = {
    {"owner",
     "CREATE TABLE IF NOT EXISTS carepet.owner ("
     " owner_id UUID,"
     " address TEXT,"
     " name TEXT,"
     " PRIMARY KEY (owner_id)"
     ")"},

    {"pet",
     "CREATE TABLE IF NOT EXISTS carepet.pet ("
     " owner_id UUID,"
     " pet_id UUID,"
     " chip_id TEXT,"
     " species TEXT,"
     " breed TEXT,"
     " color TEXT,"
     " gender TEXT,"
     " age INT,"
     " weight FLOAT,"
     " address TEXT,"
     " name TEXT,"
     " PRIMARY KEY (owner_id, pet_id)"
     ")"},

    {"sensor",
     "CREATE TABLE IF NOT EXISTS carepet.sensor ("
     " pet_id UUID,"
     " sensor_id UUID,"
     " type TEXT,"
     " PRIMARY KEY (pet_id, sensor_id)"
     ")"},

    {"measurement",
     "CREATE TABLE IF NOT EXISTS carepet.measurement ("
     " sensor_id UUID,"
     " ts TIMESTAMP,"
     " value FLOAT,"
     " PRIMARY KEY (sensor_id, ts)"
     ") WITH compaction = {'class':'TimeWindowCompactionStrategy'}"},

    {"sensor_avg",
     "CREATE TABLE IF NOT EXISTS carepet.sensor_avg ("
     " sensor_id UUID,"
     " date DATE,"
     " hour INT,"
     " value FLOAT,"
     " PRIMARY KEY (sensor_id, date, hour)"
     ") WITH compaction = {'class':'TimeWindowCompactionStrategy'}"},
};

int main(int argc, char** argv) {
    try {
        Config cfg;
        cfg.parse_args(argc, argv);

        std::cout << "Connecting to " << cfg.hosts << " ...\n";

        // Step 1: connect without a keyspace and create it.
        {
            Session s;
            s.connect(cfg);
            std::cout << "Creating keyspace 'carepet' ...\n";
            s.execute(KEYSPACE_DDL);
            std::cout << "  OK\n";
        }

        // Step 2: connect to the keyspace and create all tables.
        {
            Session s;
            s.connect_keyspace(cfg, "carepet");
            for (auto& [name, ddl] : TABLE_DDLS) {
                std::cout << "Creating table '" << name << "' ...\n";
                s.execute(ddl);
                std::cout << "  OK\n";
            }
        }

        std::cout << "Migration complete.\n";
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }
    return 0;
}
