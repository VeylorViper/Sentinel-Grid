#ifndef BLOCK_H
#define BLOCK_H
#include <stdint.h>
#define MAX_TX_SIZE 256
#define MAX_TX_COUNT 10

typedef struct {
    uint32_t index;
    uint64_t timestamp;
    char prev_hash[65];
    char merkle_root[65];
    uint32_t tx_count;
    char transactions[MAX_TX_COUNT][MAX_TX_SIZE];
    uint32_t difficulty;
    uint32_t nonce;
} Block;

void mine_block(Block* block, uint32_t difficulty);  //placeholder

#endif

