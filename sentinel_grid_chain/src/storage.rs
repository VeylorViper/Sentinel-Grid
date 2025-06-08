use std::fs::File;
use std::io::{Write, BufWriter};
use crate::ffi::Block;
use aes_gcm::aead::{Aead, KeyInit, generic_array::GenericArray};
use flate2::{Compression, write::ZlibEncoder};
use aes_gcm::{Aes256Gcm, Key, Nonce};
// use aes_gcm::aead::{Aead, KeyInit};
// use aes_gcm::{Aes256Gcm, Nonce}; 

pub fn write_block_to_file(block: &Block, path: &str) {
    let file = File::create(path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    //serialize block to raw bytes
    let raw_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            (block as *const Block) as *const u8,
            std::mem::size_of::<Block>(),
        )
    };

    //compress the serialized block
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(raw_bytes).unwrap();
    let compressed_bytes = encoder.finish().unwrap();

    //encryption key & nonce
    let key_bytes = [0u8; 32];
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = [0u8; 12];
    let nonce = GenericArray::from_slice(&nonce_bytes);

    let encrypted_data = cipher.encrypt(nonce, compressed_bytes.as_ref())
        .expect("Encryption failure!");


    writer.write_all(&0x5347424Cu32.to_be_bytes()).unwrap(); //SGBL
    writer.write_all(&1u32.to_be_bytes()).unwrap(); //version
    writer.write_all(&1u64.to_be_bytes()).unwrap(); //block count

    writer.write_all(&encrypted_data).unwrap();

    writer.flush().unwrap();
}
