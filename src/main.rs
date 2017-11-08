/*
 *
 * Main:
 *      - This file is mostly used for testing the modules 
 *
 */

// crate block
extern crate ring;

// mod and use block;
mod merkle_tree;
pub use merkle_tree::MerkleTree;

mod hash_utilities;
pub use hash_utilities::{ Hashable, HashUtilities };

mod tree;
pub use tree::LeavesIterator;

use std::hash::{ Hash, Hasher };
use ring::digest::{ Algorithm, Context, SHA512 };

// Main function
fn main()
{
    
    //test_all();
    
}

// Wrapper test function
fn test_all()
{
    
    //block::block_test();
    
}
