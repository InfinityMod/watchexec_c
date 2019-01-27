#include <stdint.h>
#include <stdio.h>

#ifndef WATCHEXEC_C_H
#define WATCHEXEC_C_H

struct _DynArray {
    struct PathOp{
        char *path;
        char *op;
        uint32_t cookie;
        int64_t time;
        uint32_t time_micro;
    };
    size_t length;
    size_t capacity;
    PathOp *array[];
};

typedef struct _DynArray DynArray;
int32_t run_watcher(const char* arguments, void(*echofn)(uint8_t, DynArray));

#endif