cmake_minimum_required(VERSION 3.25)

# Compiler settings
set(CMAKE_CXX_COMPILER "icx")
project(FileNewer LANGUAGES CXX)

# ImGui library
set(IMGUI_DIR "${CMAKE_CURRENT_SOURCE_DIR}/imgui")
file(GLOB IMGUI_SOURCES ${IMGUI_DIR}/*.cpp ${IMGUI_DIR}/backends/imgui_impl_win32.cpp ${IMGUI_DIR}/backends/imgui_impl_dx11.cpp)
add_library(imgui ${IMGUI_SOURCES})
target_include_directories(imgui PUBLIC ${IMGUI_DIR})

# Build settings
set(SOURCES main.cpp)
add_executable(FileNewer ${SOURCES})
target_link_libraries(FileNewer PRIVATE imgui d3d11)

# Run twice if attempts to build with VS
set(CMAKE_GENERATOR "Ninja" CACHE INTERNAL "" FORCE)

