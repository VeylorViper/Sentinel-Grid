#include "block.h"
#include <stdio.h>
#include <stdint.h>

void mine_block(Block* block, uint32_t difficulty) {
    //placeholder mining logic - Sentinel nonce logic is work in progress
    for (uint32_t n = 0; n < 1000000; n++) {
        block->nonce = n;
        if (n % 100000 == 0) {
            printf("Mining... nonce=%u\n", n);
        }
    }
}
