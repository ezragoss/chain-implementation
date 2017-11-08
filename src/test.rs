#![cfg(test)]

use ring::digest::{Algorithm, Context, SHA512};
use merkletree::MerkleTree;
use hashutils::{Hashable, HashUtils};

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

#[test]
fn test_from_vec1() {
    let values = vec!["hello, world".to_string()];
    let tree = MerkleTree::new_tree(digest, values);

    let root_hash = &digest.leaf_hash(&"hello, world".as_bytes());

    assert_eq!(tree.leaf_count(), 1);
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.hash().as_slice(), hash.as_ref());
}
