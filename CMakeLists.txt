cmake_minimum_required(VERSION 3.20)

project(cpp-playground LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(CTest)

if(BUILD_TESTING)
  find_package(GTest REQUIRED)
  include(GoogleTest)

  function(add_leetcode_problem source_file)
    get_filename_component(problem_directory "${source_file}" DIRECTORY)
    get_filename_component(problem_name "${problem_directory}" NAME)
    set(target_name "${problem_name}-test")

    add_executable("${target_name}" "${source_file}")
    target_include_directories("${target_name}"
                               PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/include")
    target_link_libraries("${target_name}" PRIVATE GTest::gtest_main)

    if(MSVC)
      target_compile_options("${target_name}" PRIVATE /W4 /permissive-)
    else()
      target_compile_options("${target_name}" PRIVATE -Wall -Wextra -Wpedantic)
    endif()

    gtest_discover_tests("${target_name}")
  endfunction()

  file(GLOB problem_sources CONFIGURE_DEPENDS
       "${CMAKE_CURRENT_SOURCE_DIR}/problems/*/main.cpp")

  foreach(problem_source IN LISTS problem_sources)
    add_leetcode_problem("${problem_source}")
  endforeach()
endif()
