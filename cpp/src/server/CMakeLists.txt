add_library(server
    server.cpp
    handlers.cpp
)

target_link_libraries(server PRIVATE common scylla-cpp-driver Boost::program_options Boost::url)
