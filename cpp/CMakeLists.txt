cmake_minimum_required(VERSION 3.29.1)
project(ParRandomSampling)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_FLAGS "-Wall -Wextra -pedantic -Werror -O3")
# set(CMAKE_BUILD_TYPE Release)

add_executable("${PROJECT_NAME}" sampling_main.cpp)

add_subdirectory(test)
add_subdirectory(benchmark)
