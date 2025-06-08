use std::os::raw::{c_char, c_uint, c_ulonglong};

#[repr(C)]
pub struct Block {
    pub index: c_uint,
    pub timestamp: c_ulonglong,
    pub prev_hash: [c_char; 65],
    pub merkle_root: [c_char; 65],
    pub tx_count: c_uint,
    pub transactions: [[c_char; 256]; 10],  // match block.h MAX_TX_COUNT & MAX_TX_SIZE
    pub difficulty: c_uint,
    pub nonce: c_uint,
}

unsafe extern "C" {
    pub unsafe fn mine_block(block: *mut Block, difficulty: c_uint);
}

