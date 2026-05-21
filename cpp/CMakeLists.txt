cmake_minimum_required(VERSION 3.14)
project(care-pet-cpp VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
add_compile_definitions(_GNU_SOURCE)

include(FetchContent)

# cpp-httplib
FetchContent_Declare(
    httplib
    GIT_REPOSITORY https://github.com/yhirose/cpp-httplib.git
    GIT_TAG v0.15.3
)
FetchContent_MakeAvailable(httplib)

# nlohmann/json
FetchContent_Declare(
    nlohmann_json
    GIT_REPOSITORY https://github.com/nlohmann/json.git
    GIT_TAG v3.11.3
)
FetchContent_MakeAvailable(nlohmann_json)

# ScyllaDB C++ driver
# The scylladb/cpp-driver installs as libscylla-cpp-driver on Linux.
find_library(CASSANDRA_LIB
    NAMES scylla-cpp-driver cassandra
    PATHS
        /usr/local/lib/x86_64-linux-gnu
        /usr/local/lib
        /usr/lib/x86_64-linux-gnu
        /usr/lib
    REQUIRED)
find_path(CASSANDRA_INCLUDE_DIR
    NAMES cassandra.h
    PATHS /usr/local/include /usr/include
    REQUIRED)

get_filename_component(CASSANDRA_LIB_DIR "${CASSANDRA_LIB}" DIRECTORY)
link_directories(${CASSANDRA_LIB_DIR})

add_library(cassandra_driver INTERFACE)
target_include_directories(cassandra_driver INTERFACE "${CASSANDRA_INCLUDE_DIR}")
# Use the stem (strip lib prefix and .so suffix) so the linker gets -lscylla-cpp-driver
get_filename_component(CASSANDRA_LIB_NAME "${CASSANDRA_LIB}" NAME_WE)
string(REPLACE "lib" "" CASSANDRA_LINK_NAME "${CASSANDRA_LIB_NAME}")
target_link_libraries(cassandra_driver INTERFACE ${CASSANDRA_LINK_NAME})

include_directories(${CMAKE_SOURCE_DIR}/include)

# migrate
add_executable(migrate cmd/migrate/main.cpp)
target_link_libraries(migrate PRIVATE cassandra_driver)

# sensor
add_executable(sensor cmd/sensor/main.cpp)
target_link_libraries(sensor PRIVATE cassandra_driver)

# server
add_executable(server cmd/server/main.cpp)
target_link_libraries(server PRIVATE cassandra_driver httplib::httplib nlohmann_json::nlohmann_json)
