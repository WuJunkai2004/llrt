cmake_minimum_required(VERSION 3.16)
project(llrt_wamr C CXX ASM)

include(FetchContent)

FetchContent_Declare(
  wamr
  URL https://github.com/bytecodealliance/wasm-micro-runtime/archive/refs/heads/main.tar.gz
)
FetchContent_GetProperties(wamr)
if(NOT wamr_POPULATED)
  FetchContent_Populate(wamr)
endif()

set(WAMR_ROOT_DIR ${wamr_SOURCE_DIR})
set(WAMR_BUILD_PLATFORM "linux")
set(WAMR_BUILD_INTERP 1)
set(WAMR_BUILD_AOT 1)
set(WAMR_BUILD_JIT 0)
set(WAMR_BUILD_FAST_INTERP 1)
set(WAMR_BUILD_LIBC_BUILTIN 1)
set(WAMR_BUILD_LIBC_WASI 1)
set(WAMR_BUILD_MULTI_MODULE 0)
set(WAMR_BUILD_MINI_LOADER 0)
set(WAMR_BUILD_SIMD 0)

include(${WAMR_ROOT_DIR}/build-scripts/runtime_lib.cmake)

add_library(vmlib STATIC ${WAMR_RUNTIME_LIB_SOURCE})
add_library(iwasm SHARED ${WAMR_RUNTIME_LIB_SOURCE})
target_link_libraries(iwasm PRIVATE m dl pthread)
