cmake_minimum_required(VERSION 3.24)

project(cftw-cli)

add_executable(cftw-cli 
    src/main.cpp
    src/cli.cpp
    src/processor.cpp
    src/globals.cpp
)

target_include_directories(cftw-cli PRIVATE include)
