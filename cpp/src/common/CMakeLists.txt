add_library(common
    database.cpp
    json.cpp
)
target_link_libraries(common PRIVATE Boost::json Boost::program_options scylla-cpp-driver)
