/*
 *
 * Main:
 *      - This file is used for including libraries  
 *
 */

// crate block
extern crate ring;

// mod and use blocks;
mod merkle_tree;
pub use merkle_tree::MerkleTree;

mod hash_utilities;
pub use hash_utilities::{ Hashable, HashUtilities };

mod tree;
pub use tree::LeavesIterator;

use std::hash::{ Hash, Hasher };
use ring::digest::{ Algorithm, Context, SHA512 };
