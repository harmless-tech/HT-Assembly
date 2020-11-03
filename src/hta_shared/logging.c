#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <time.h>
#include "logging.h"

#define FMT_STR "[%s - %s]: "

FILE *file_ptr;

void init_logger() {
    // Dir
    system("mkdir logs");

    // File
    FILE *file = fopen("logs/hta.log", "w");
    if(file == NULL) {
        perror("Could not create log file!");
        exit(EXIT_FAILURE);
    }

    fflush(file);

    file_ptr = file;
}

void print(const char *tag, const char *format, ...) {
    char s[51];
    time_t t = time(NULL);
    struct tm *p = localtime(&t);

    strftime(s, 50, "%D %H:%M:%S %Z", p);

    va_list args;
    va_list args2;
    va_start(args, format);
    va_copy(args2, args);

    printf(FMT_STR, s, tag);
    vprintf(format, args);

    fprintf(file_ptr, FMT_STR, s, tag);
    vfprintf(file_ptr, format, args2);
    fflush(file_ptr);

    va_end(args);
    va_end(args2);
}

void close_logger() {
    fclose(file_ptr);
}
