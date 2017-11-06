use std::io;
use std::string::String;
use std::ops;
use tree::LeavesIterator;
use std::hash::{Hash, Hasher};

mod block;

// Merkle Tree struct
pub struct MerkleTree<T>
{

    // The algorithm used for hashing
    pub algorithm: &'static Algorithm,
    // The root of the tree
    root: Tree<T>,
    // The height of the tree
    height: usize,
    // The number of leaves
    count: usize

}

// Hash impl
impl<T: Hash> Hash for MerkleTree<T>
{
    
    pub fn hash<H: Hasher>( &self, state: &mut H )
    {
        <Tree<T> as Hash>::hash( &self.root, state );
        self.height.hash( state );
        self.count.hash( state );
        ( self.algorithm as *const Algorithm )
            .hash( state );
    }
    
}

// Merkle Tree impl
impl<T> MerkleTree<T>
{

    // Constructs a new Merkle Tree
    pub fn new_tree -> self( )
    {
        // Creates an empty Merkle Tree and initializes base values
        MerkleTree
        {
            root: root,
            height: 0,
            count: 0,
            hash: String::new()
        }
    }
    // Returns the number of leaves in the tree
    pub fn leaf_count( &self ) -> usize
    {
        return self.count
    }
    // Returns the height of the tree
    pub fn height( &self ) -> usize
    {
        return self.height
    }
    // Returns true if the tree is empty (has no leaves) and false otherwise
    pub fn is_empty( &self ) -> bool
    {
        return self.count == 0
    }
    // Verifies a hash passed through with the tree's hash
    pub fn verify_hash( hash_to_compare: String ) -> bool 
    {
        return  self.hash == hash_to_compare 
    }
    // Returns an iterator over the tree
    pub fn iterator( &self ) -> LeavesIterator<T>
    {
        return self.root.iter()
    }
    pub fn 
    // Hashes the root of the tree
    pub fn hash( &mut self )
    {
        // Generates hash
        self.root.digest();
        self.hash = self.root.hash;
    }
    
}
