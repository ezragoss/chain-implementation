extern crate sha3;

use sha3::{ Digest, Sha3_256 };
use std::time::{ Instant };
use std::io;
use std::string::String;

/* Block 
 *
 */
struct Block<T: ToString> {
    index: u8,
    previous_hash: String,
    timestamp: Instant,
    data: T,
    hash: String,
}

fn main() {
    block_test();
}

/* Generate the hash with SHA3-256
 *
 * TODO: Hash should be generated based on adding the contents of the block together
 */
fn generate_hash( string: &String ) -> String {

    // Break string into byte array
    let bytes : &[u8] = string.as_bytes();
    
    // Generate the Sha-256 hasher
    let mut hasher = Sha3_256::default();
    
    // Write the input message
    hasher.input( bytes );
    
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
fn build_block<T: ToString >( data: T, index: u8, previous_hash: String ) -> Block<T> {
    let timestamp = Instant::now();
    let concatenation: String = "".to_owned(); 
    let block = Block::<T> {
        index: index,
        previous_hash: previous_hash,
        timestamp: timestamp,
        data: data,
        hash: generate_hash(
            &format!("{}", index.to_string() )
        ),
    };
    return block;
}

/*
Tests
*/
fn block_test() {
    let str_ : &String = &String::from("testing string");
    let str_hash : &[u8] = str_.as_bytes();
    let mut previous_hash: String = generate_hash( str_ );
    for x in 1..10 {
        let block = build_block(x, x, previous_hash);
        println!("Index: {x}, Previous Hash: {previous}, data: {data}, hash: {hash}", x = x, previous = block.previous_hash, data = block.data, hash=block.hash);
        previous_hash = block.hash;        
    }
} 
