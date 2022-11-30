cmake_minimum_required(VERSION 3.16)

add_subdirectory(raylib)

project(thePresenter)

add_executable(${PROJECT_NAME}
  src/main.cpp
)

target_link_libraries(${PROJECT_NAME} PUBLIC raylib)
target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_CURRENT_LIST_DIR}/raygui/src/)