#############################################################################
## MIT License
## 
## Author	: xiaofeng.zhu
## Support	: zxffffffff@outlook.com, 1337328542@qq.com
## 
#############################################################################

message("[start-rust-vcpkg] -------- start")
cmake_minimum_required (VERSION 3.16)

# 使用 vcpkg 工具链，需要在 project 之前设置
set(CMAKE_TOOLCHAIN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")
message(STATUS "[start-rust-vcpkg] CMAKE_TOOLCHAIN_FILE : ${CMAKE_TOOLCHAIN_FILE}")

# 空的工程，只使用 vcpkg 工具链
project ("start-rust-vcpkg")

# 引入 zlib
find_package(ZLIB REQUIRED)
message(STATUS "[${PROJECT_NAME}] ZLIB_VERSION_STRING : ${ZLIB_VERSION_STRING}")
message(STATUS "[${PROJECT_NAME}] ZLIB_INCLUDE_DIRS : ${ZLIB_INCLUDE_DIRS}")
message(STATUS "[${PROJECT_NAME}] ZLIB_LIBRARIES : ${ZLIB_LIBRARIES}")

message("[start-rust-vcpkg] -------- done")