extern crate sha3;

use sha3::{ Digest, Sha3_256 };
use std::time::{ Instant };
use std::io;
use std::string::String;

/* Block 
 *
 */
struct Block<T> {
    index: u8,
    previous_hash: String,
    timestamp: Instant,
    data: T,
    hash: String,
}

/* Generate the hash with SHA3-256
 *
 * TODO: Hash should be generated based on adding the contents of the block together
 */
fn generate_hash(index: u8) -> String {
    // Generate the Sha-256 hasher
    let mut hasher = Sha3_256::default();
    
    // Write the input message
    hasher.input(&[index]);
    
    // read hash digest
    let out = hasher.result();

    // Return a hex string of the hash
    return format!("{:x}", out);
}

/* Build a block
 *
 * Generic data type T can be stored
 * Index and previous hash are passed in
 *
 */
fn build_block<T>(data: T, index: u8, previous_hash: String) -> Block<T> {
    let timestamp = Instant::now();
    let block = Block::<T> {
        index: index,
        previous_hash: previous_hash,
        timestamp: timestamp,
        data: data,
        hash: generate_hash(index),
    };
    return block;
}

/*
Tests
*/
fn block_test() {
    let mut previous_hash: String = generate_hash(0);
    for x in 1..10 {
        let block = build_block(x, x, previous_hash);
        println!("Index: {x}, Previous Hash: {previous}, data: {data}, hash: {hash}", x = x, previous = block.previous_hash, data = block.data, hash=block.hash);
        previous_hash = block.hash;        
    }
} 

fn main() {
    block_test();
}
