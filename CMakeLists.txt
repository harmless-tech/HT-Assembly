cmake_minimum_required(VERSION 3.15)

# Env Vars
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED True)
#set(CMAKE_CXX_STANDARD 11)
#set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_TOOLCHAIN_FILE ${CMAKE_CURRENT_SOURCE_DIR}/vendors/vcpkg/scripts/buildsystems/vcpkg.cmake CACHE STRING "Vcpkg toolchain file")
if(WIN32)
	# Windows
	set(VCPKG_TARGET_TRIPLET x64-windows-static CACHE STRING "Vcpkg triplet")
elseif(APPLE)
	# MacOS
	set(VCPKG_TARGET_TRIPLET x64-osx CACHE STRING "Vcpkg triplet")
	set(THREADS_PREFER_PTHREAD_FLAG ON)
elseif(UNIX)
	# Linux
	set(VCPKG_TARGET_TRIPLET x64-linux CACHE STRING "Vcpkg triplet")
	set(THREADS_PREFER_PTHREAD_FLAG ON)
else()
	message(FATAL_ERROR "Unknown platform.")
endif()

project(hta-compiler LANGUAGES C VERSION 0.0.1)
project(hta-runtime LANGUAGES C VERSION 0.0.1)
project(hta-shared LANGUAGES C VERSION 0.0.1)

project(ht-assembly LANGUAGES C VERSION 0.0.1)

# CMake Check
if(NOT CMAKE_GENERATOR MATCHES "Ninja")
	message(WARNING "It is highly suggested to use '-G Ninja'. (https://ninja-build.org/)")
endif()

if(NOT CMAKE_C_COMPILER_ID MATCHES "Clang")
	message(WARNING "It is highly suggested to use clang to compile. (https://clang.llvm.org/)")
endif()

# Packages
find_package(unofficial-parson CONFIG REQUIRED)
#find_package(binn CONFIG REQUIRED)

if(WIN32)
	# Windows
	find_package(pthreads CONFIG REQUIRED)
else()
	# Linux and MacOS
	find_package(Threads REQUIRED)
endif()

#TODO Should headers be included in the source files??
# Internal Libs
file(GLOB_RECURSE COMPILER_SOURCES LIST_DIRECTORIES true src/hta_compiler/*.c)# src/hta_compiler/*.h)
file(GLOB_RECURSE RUNTIME_SOURCES LIST_DIRECTORIES true src/hta_runtime/*.c)# src/hta_runtime/*.h)
file(GLOB_RECURSE SHARED_SOURCES LIST_DIRECTORIES true src/hta_shared/*.c)# src/hta_shared/*.h)
add_library(hta-compiler STATIC ${COMPILER_SOURCES})
add_library(hta-runtime STATIC ${RUNTIME_SOURCES})
add_library(hta-shared STATIC ${SHARED_SOURCES})

# Exe
file(GLOB_RECURSE SOURCES LIST_DIRECTORIES true src/exe/*.c)# src/exe/*.h) #src/*.h src/*.c src/*cpp)
add_executable(ht-assembly ${SOURCES})

# Internal Libs Link
target_link_libraries(ht-assembly PRIVATE hta-compiler)
target_link_libraries(ht-assembly PRIVATE hta-runtime)
target_link_libraries(ht-assembly PRIVATE hta-shared)

# Libs
target_link_libraries(ht-assembly PRIVATE unofficial::parson::parson)

find_path(binn_INCLUDE_DIR binn/binn.h)
find_library(binn_LIBRARY binn)
target_include_directories(ht-assembly PRIVATE ${binn_INCLUDE_DIR}/binn)
target_link_libraries(ht-assembly PRIVATE ${binn_LIBRARY})

if(WIN32)
	# Windows
	target_link_libraries(ht-assembly PRIVATE PThreads4W::PThreads4W)
else()
	# Linux and MacOS
	target_link_libraries(ht-assembly PRIVATE Threads::Threads)
endif()
