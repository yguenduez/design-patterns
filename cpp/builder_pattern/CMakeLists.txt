cmake_minimum_required(VERSION 3.0.0)
project(builder_pattern VERSION 0.1.0)

include(CTest)
enable_testing()

add_executable(builder_pattern main.cpp)
set_property(TARGET builder_pattern PROPERTY CXX_STANDARD 14)
