cmake_minimum_required(VERSION 3.25)

set(CMAKE_CXX_COMPILER "icx")

project(FileNewer LANGUAGES CXX)
#set(CMAKE_PREFIX_PATH "C:\Program Files (x86)\Intel\oneAPI\compiler\2024.1\lib\cmake\IntelSYCL\IntelSYCLConfig.cmake")

#find_package(IntelSYCL REQUIRED)
add_executable(FileNewer main.cpp)
#add_sycl_to_target(TARGET FileNewer SOURCES main.cpp)

#Run twice if atemptps to build with VS
set(CMAKE_GENERATOR "Ninja" CACHE INTERNAL "" FORCE)
