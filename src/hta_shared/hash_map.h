#ifndef HASH_MAP_H
#define HASH_MAP_H

#include <xxhash.h>

typedef struct hash_map_node {
    XXH64_hash_t hash;
    void *data;
    struct hash_map_node *next;
} Hash_Map_Node;

typedef struct {
    Hash_Map_Node *head;
} Hash_Map;

void init_hash_map(Hash_Map *const map);
void hash_map_insert(Hash_Map *const map, char *const name, void *data);
void *hash_map_remove(Hash_Map *const map, char *const name);
void *get_hash_map_get(Hash_Map *const map, char *const name);
//void change_hash_map(Hash_Map *map, char *name, void *data); Use hash_map_insert to change data also.

#endif // HASH_MAP_H
