#include <boost/program_options.hpp>
#include <iostream>

#include "migrate/migrate.hpp"
#include "sensor/sensor.hpp"
#include "server/server.hpp"

namespace po = boost::program_options;

int main(int argc, char* argv[]) {
    po::options_description desc("Allowed options");
    // clang-format off
    desc.add_options()
        ("help,h", "produce help message")
        ("mode", po::value<std::string>(), "run mode: migrate, sensor, or server")
        ("scylla-host", po::value<std::string>()->default_value("127.0.0.1"), "Scylla host")
        ("host", po::value<std::string>()->default_value("127.0.0.1"), "[Mode: server] Server host")
        ("port", po::value<unsigned short>()->default_value(8080), "[Mode: server] Server port")
        ("seconds", po::value<int>()->default_value(60), "[Mode: sensor] Sensor run time in seconds")
        ("ddl-file", po::value<std::vector<std::string>>()->multitoken()->default_value({"./data/care-pet-ddl.cql"}, "./data/care-pet-ddl.cql"),
            "[Mode: migrate] Files with CQL commands to run (accepts multiple values)");
    // clang-format on

    po::positional_options_description p;
    p.add("mode", 1);

    po::variables_map vm;
    po::store(
        po::command_line_parser(argc, argv).options(desc).positional(p).run(),
        vm);
    po::notify(vm);

    if (vm.count("help")) {
        std::cout << desc << "\n";
        return 1;
    }

    if (vm.count("mode")) {
        std::string mode = vm["mode"].as<std::string>();
        if (mode == "migrate") {
            run_migrate(vm);
        } else if (mode == "sensor") {
            run_sensor(vm);
        } else if (mode == "server") {
            run_server(vm);
        } else {
            std::cerr << "Error: Unknown mode '" << mode << "'\n";
            std::cerr << desc << "\n";
            return 1;
        }
    } else {
        std::cerr << "Error: Mode not specified.\n";
        std::cerr << desc << "\n";
        return 1;
    }

    return 0;
}
