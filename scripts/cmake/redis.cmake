# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

if(NOT REDIS_DOT_CMAKE_INCLUDED)
set(REDIS_DOT_CMAKE_INCLUDED YES)

include(ExternalProject)

function(add_redis REDIS_TARGET LIBOS_TARGET REDIS_SOURCE_DIR)
    set(REDIS_BINARY_DIR ${CMAKE_BINARY_DIR}/submodules/${REDIS_TARGET})

    if(CMAKE_BUILD_TYPE MATCHES "Rel")
        set(OPT_CFLAGS -O3)
    else(CMAKE_BUILD_TYPE MATCHES "Rel")
        set(OPT_CFLAGS "-O0 -D_FORTIFY_SOURCE -fstack-protector-strong")
    endif(CMAKE_BUILD_TYPE MATCHES "Rel")

    get_property(
        HOARD_TARGET
        TARGET ${LIBOS_TARGET}
        PROPERTY HOARD
    )
    if(DEFINED HOARD_TARGET)
        #message("${REDIS_TARGET} => ${LIBOS_TARGET}:HOARD=${HOARD_TARGET}")
        ExternalProject_Get_Property(${HOARD_TARGET} SOURCE_DIR)
        set(DEMETER_MALLOC ${SOURCE_DIR}/src/libhoard.so)
    else(DEFINED HOARD_TARGET)
        set(DEMETER_MALLOC libc)
    endif(DEFINED HOARD_TARGET)

    if(CMAKE_VERBOSE_MAKEFILE)
        set(REDIS_VERBOSE_MAKEFILE "V=1")
    endif(CMAKE_VERBOSE_MAKEFILE)

    add_custom_target(${REDIS_TARGET}
        make install PREFIX=${REDIS_BINARY_DIR} MALLOC=${DEMETER_MALLOC} DEMETER_INCLUDE=${CMAKE_SOURCE_DIR}/include DEMETER_LIBOS_SO=$<TARGET_FILE:${LIBOS_TARGET}> DEMETER_COMMON_A=$<TARGET_FILE:dmtr-libos-common> DEMETER_BASE_A=$<TARGET_FILE:dmtr-libos-base> DEMETER_LATENCY_A=$<TARGET_FILE:dmtr-latency> OPTIMIZATION=${OPT_CFLAGS} ${REDIS_VERBOSE_MAKEFILE}
        WORKING_DIRECTORY ${REDIS_SOURCE_DIR}
        DEPENDS ${LIBOS_TARGET} ${HOARD_TARGET}
    )
endfunction(add_redis)

function(add_catnip_redis REDIS_TARGET REDIS_SOURCE_DIR)
    set(REDIS_BINARY_DIR ${CMAKE_BINARY_DIR}/submodules/${REDIS_TARGET})

    if(CMAKE_BUILD_TYPE MATCHES "Rel")
        set(OPT_CFLAGS -O3)
    else(CMAKE_BUILD_TYPE MATCHES "Rel")
        set(OPT_CFLAGS "-O0 -D_FORTIFY_SOURCE -fstack-protector-strong")
    endif(CMAKE_BUILD_TYPE MATCHES "Rel")

    set(DEMETER_MALLOC libc)

    if(CMAKE_VERBOSE_MAKEFILE)
        set(REDIS_VERBOSE_MAKEFILE "V=1")
    endif(CMAKE_VERBOSE_MAKEFILE)

    set(LIBOS_SO_PATH ${CMAKE_BINARY_DIR}/src/rust/catnip_libos/lib/libcatnip_libos.so)
    set(DPDK_LDFLAGS "-ldl -pthread -Wl,-rpath=${CMAKE_BINARY_DIR}/ExternalProject/dpdk/install/lib/x86_64-linux-gnu -Wl,--disable-new-dtags -Wl,@${CMAKE_BINARY_DIR}/ExternalProject/dpdk/install/ldflags.txt")

    add_custom_target(${REDIS_TARGET}
	    make install PREFIX=${REDIS_BINARY_DIR} MALLOC=${DEMETER_MALLOC} DEMETER_INCLUDE=${CMAKE_SOURCE_DIR}/include DEMETER_LIBOS_SO=${LIBOS_SO_PATH} DEMETER_COMMON_A=$<TARGET_FILE:dmtr-libos-common> DEMETER_BASE_A=$<TARGET_FILE:dmtr-libos-base> DEMETER_LATENCY_A=$<TARGET_FILE:dmtr-latency> OPTIMIZATION=${OPT_CFLAGS} DPDK_LDFLAGS=${DPDK_LDFLAGS} V=1
        WORKING_DIRECTORY ${REDIS_SOURCE_DIR}
	DEPENDS dpdk
    )

endfunction(add_catnip_redis)

endif(NOT REDIS_DOT_CMAKE_INCLUDED)
