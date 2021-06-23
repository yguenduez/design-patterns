cmake_minimum_required(VERSION 3.0.0)
project(factory_method VERSION 0.1.0)

include(CTest)
enable_testing()

add_executable(factory_method main.cpp)
set_property(TARGET factory_method PROPERTY CXX_STANDARD 14)
