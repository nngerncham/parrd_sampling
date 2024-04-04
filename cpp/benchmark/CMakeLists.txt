cmake_minimum_required(VERSION 3.29.1)
project(ParSamplingBenchmarks)

find_package(benchmark REQUIRED)
set(CMAKE_BUILD_TYPE Release)

add_executable(g_benchmark sampling_benchmarks.cpp)
target_link_libraries(g_benchmark benchmark::benchmark)
