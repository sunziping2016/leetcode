cmake_minimum_required(VERSION 4.0)

project(playground)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(GTest REQUIRED)

include_directories("${CMAKE_CURRENT_SOURCE_DIR}/include")

enable_testing()

add_executable(two-sum-test problems/0001-two-sum/main.cpp)
target_link_libraries(two-sum-test GTest::gtest_main)
gtest_discover_tests(two-sum-test)
