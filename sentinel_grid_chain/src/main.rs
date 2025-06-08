mod ffi;
mod chain;
mod storage;
mod network; // coming next

use chain::create_block;
use storage::write_block_to_file;

fn main() {
    let block = create_block();
    write_block_to_file(&block, "data/chain.sgblk");
    println!("Block created and saved.");

    // Start networking (peer sync)
    network::start_networking();
}
