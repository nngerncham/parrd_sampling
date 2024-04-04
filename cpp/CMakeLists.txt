cmake_minimum_required(VERSION 3.10)
project(ParRandomSampling)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# for testing and benchmarking
# include(FetchContent)
# FetchContent_Declare(
#   Catch2
#   GIT_REPOSITORY https://github.com/catchorg/Catch2.git
#   GIT_TAG v3.5.3
# )
# FetchContent_MakeAvailable(Catch2)
# list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
#
# add_executable(tests test.cpp)
# target_link_libraries(tests PRIVATE Catch2::Catch2WithMain)
#
# add_executable(custom-main-tests test.cpp test-main.cpp)
# target_link_libraries(custom-main-tests PRIVATE Catch2::Catch2)

add_executable(Sampling sampling_main.cpp)
