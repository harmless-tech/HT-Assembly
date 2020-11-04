#include <stdlib.h>
#include "../hta_shared/logging.h"
#include "../hta_shared/utils.h"
#include "hta_compiler.h"

int hta_compile_export(char *file_path, char *export_file_path) {
    //TODO
    return 0;
}

HTA_Database *hta_compile(char *file_path) {
    HTA_Database *database = malloc(sizeof(HTA_Database));
    TEST_ALLOCATION(database, "HTA_Database in compiler")
    init_hta_database(database);

    //TODO
}