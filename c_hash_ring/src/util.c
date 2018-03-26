#include <hash_ring.h>

uint8_t *get_hash_ring_node_name(hash_ring_node_t *node) {
    return node->name;
}

uint32_t get_hash_ring_node_name_len(hash_ring_node_t *node) {
    return node->nameLen;
}