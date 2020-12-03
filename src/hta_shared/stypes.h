#ifndef STYPES_H
#define STYPES_H

#include <stdint.h>
#include <stdbool.h>

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

typedef float f32; // Size is not guaranteed.
typedef double f64; // Size is not guaranteed.

typedef size_t usize;
//typedef ssize_t isize;

typedef intptr_t ptr;
typedef uintptr_t uptr;

#endif // STYPES_H
