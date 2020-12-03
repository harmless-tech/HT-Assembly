#ifndef HTA_COMPILER_H
#define HTA_COMPILER_H

#include "../hta_shared/hta.h"

int hta_compile_export(char *file_path, char *export_file_path); //TODO Change arg to allow multiple files or a build file.
HTA_Database *hta_compile(char *file_path); //TODO Change arg to allow multiple files or a build file.

#endif // HTA_COMPILER_H
