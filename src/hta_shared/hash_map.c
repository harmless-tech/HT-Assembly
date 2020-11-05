#include <string.h>
#include <xxhash.h>
#include "hash_map.h"

#define SEED 63245872362345346
#define HASH(name) XXH64(name, strlen(name) + 1, 63245872362345346)

static int does_hash_exist(Hash_Map *const map, XXH64_hash_t hash);
/*static inline XXH64_hash_t hash_string(char *const name);*/

void init_hash_map(Hash_Map *const map) {
    if(map != NULL)
        map->head = NULL;
}

void hash_map_insert(Hash_Map *const map, char *const name, void *data) {
    if(map != NULL && name != NULL) {
        XXH64_hash_t hash = HASH(name);

        // Create
        // Change
    }
}

void *hash_map_remove(Hash_Map *const map, char *const name) {
    if(map != NULL && name != NULL) {
        // Free
    }
}

void *get_hash_map_data(Hash_Map *const map, char *const name) {
    if(map != NULL && name != NULL) {

    }
}

static int does_hash_exist(Hash_Map *const map, XXH64_hash_t hash) {
    Map_Node *node = map->head;

    while(node != NULL) {
        if(node->hash == hash)
            return 1;

        node = node->next;
    }

    return 0;
}

/*static inline XXH64_hash_t hash_string(char *const name) {
    return XXH64(name, strlen(name) + 1, SEED);
}*/
