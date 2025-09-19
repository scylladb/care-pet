#include <cassandra.h>
#include <fstream>
#include <iostream>
#include <string>

#include "database.hpp"
#include "migrate.hpp"

void execute_cql_file(Database& db, const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) {
        std::cerr << "Error: Could not open CQL file " << path << std::endl;
        return;
    }

    std::string query;
    std::string line;
    while (std::getline(file, line)) {
        if (line.empty() || line.rfind("--", 0) == 0) {
            continue;
        }
        query += line;
        query += "\n";
        if (line.back() == ';') {
            Statement statement(query.c_str());
            std::cout << "Executing: " << query << std::endl;
            db.execute(statement);
            query.clear();
        }
    }
}

void run_migrate(const boost::program_options::variables_map& vm) {
    Database db(vm);
    for (auto file : vm["ddl-file"].as<std::vector<std::string>>()) {
        execute_cql_file(db, file);
    }
}
