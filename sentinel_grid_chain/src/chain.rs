use crate::ffi::Block;
use std::ffi::CString;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub fn create_block() -> Block {
    let prev = CString::new("000000...GENESIS").unwrap();
    let root = CString::new("FAKE_MERKLE_HASH").unwrap();

    let mut block = Block {
        index: 0,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64,
        prev_hash: [0; 65],
        merkle_root: [0; 65],
        tx_count: 5,  // number of random txns
        transactions: [[0; 256]; 10],
        difficulty: 1,
        nonce: 0,
    };

    for (i, b) in prev.as_bytes().iter().enumerate() {
        block.prev_hash[i] = *b as i8;
    }

    for (i, b) in root.as_bytes().iter().enumerate() {
        block.merkle_root[i] = *b as i8;
    }

    // Generate random transactions
    let mut rng = rand::thread_rng();
    for i in 0..block.tx_count {
        for j in 0..256 {
            block.transactions[i as usize][j] = rng.r#gen::<i8>();
        }
    }

    unsafe {
        crate::ffi::mine_block(&mut block, block.difficulty);
    }

    block
}
