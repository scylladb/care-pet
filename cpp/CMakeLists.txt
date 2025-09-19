cmake_minimum_required(VERSION 3.30)
project(care-pet)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(Boost 1.83.0 REQUIRED COMPONENTS program_options system filesystem json url)

include_directories(/usr/include)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/src/common)
link_directories(/usr/lib64)

add_subdirectory(src/common)
add_subdirectory(src/migrate)
add_subdirectory(src/sensor)
add_subdirectory(src/server)

add_executable(care-pet src/main.cpp)
target_include_directories(care-pet PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_link_libraries(care-pet
    migrate
    sensor
    server
    common
    Boost::program_options
)
