#ifndef HTA_H
#define HTA_H

#include "hash_map.h"

typedef struct {
    char *entry_frame;
    Hash_Map map;
} HTA_Database;

typedef struct {
    char temp; //TODO Replace.
} Frame;

void init_hta_database(HTA_Database *database);

#endif // HTA_H
