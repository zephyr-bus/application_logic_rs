# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.13.1)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(hello_world)

target_sources(app PRIVATE src/main.c)

# Add the library
add_library(application_logic_rs STATIC IMPORTED GLOBAL)

# Set the paths
set_target_properties(application_logic_rs PROPERTIES IMPORTED_LOCATION             ${CMAKE_CURRENT_SOURCE_DIR}/../../target/thumbv7em-none-eabi/release/libapplication_logic_rs.a)
set_target_properties(application_logic_rs PROPERTIES INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_CURRENT_SOURCE_DIR}/../../generated/)

# Link them!
target_link_libraries(app PUBLIC application_logic_rs -Wl,--allow-multiple-definition)
