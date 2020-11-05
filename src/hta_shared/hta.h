#include "hash_map.h"

#ifndef HTA_H
#define HTA_H

typedef struct {
    char *entry_frame;
    Hash_Map map;
} HTA_Database;

typedef struct {
    int temp; //TODO Replace.
} Frame;

void init_hta_database(HTA_Database *database);

#endif //HTA_H
