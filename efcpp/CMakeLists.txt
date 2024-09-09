cmake_minimum_required(VERSION 3.22)
project(efcpp)

set(CMAKE_CXX_STANDARD 17)
add_definitions(-std=c++17)

include_directories(/usr/local/include/imgui-for-imtui)
include_directories(/usr/local/include/imtui)

add_executable(efcpp src/main.cpp)
