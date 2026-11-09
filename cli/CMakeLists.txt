cmake_minimum_required(VERSION 3.24)

project(cftw-cli)

add_executable(cftw-cli 
    src/main.cpp
    src/cli.cpp
    src/processor.cpp
    src/globals.cpp
)

target_include_directories(cftw-cli PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(cftw-cli PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/libbmp.a)