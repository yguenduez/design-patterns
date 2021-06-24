cmake_minimum_required(VERSION 3.0.0)
project(singleton VERSION 0.1.0)

add_executable(singleton main.cpp)
set_property(TARGET singleton PROPERTY CXX_STANDARD 14)