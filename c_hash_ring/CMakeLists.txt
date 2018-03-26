cmake_minimum_required(VERSION 3.8)
project(hash_ring)

set(CMAKE_CXX_STANDARD 11)
#set(CMAKE_BUILD_TYPE Debug)

include_directories(src)

add_library(hash_ring STATIC
        src/hash_ring.c
        src/md5.c
        src/sha1.c
        src/util.c
        )
set_property(TARGET hash_ring PROPERTY POSITION_INDEPENDENT_CODE ON)