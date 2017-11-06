use std::io;
use std::string::String;
use std::ops;
use tree::LeavesIterator;
use std::hash::{Hash, Hasher};
use ring::digest::Algorithm;

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
        ( self.algorithm as *const Algorithm ).hash( state );
    }
    
}

// Merkle Tree impl
impl<T> MerkleTree<T>
{

    // Constructs a new Merkle Tree using a vector to store blocks
    pub fn new_tree( algorithm: &'static Algorithm, values: Vec<T> ) -> self
    where
        T: Hashable,
    
    {
        // If the vector is empty -> construct a new Merkle Tree
        if values.is_empty()
        {
            // Creates an empty Merkle Tree and initializes fields to base values
            return MerkleTree
            {
                algorithm: algorithm,
                root: Tree::empty( algorithm.hash_empty() ),
                height: 0,
                count: 0
            };
        }
        // Number of leaves
        let count: usize = values.len();
        // Height of the tree
        let mut height: usize = 0;
        // Current vector
        let mut current: Vec = Vec::with_capacity( count );
        // Iterate through the values in the vector and add the leaves
        for v in values
        {
            // Creates a new leaf and pushes it to the vector
            let leaf: Tree = Tree::new_leaf( algorithm, v );
            current.push( leaf );
        }
        // While current vector has a length greater than 1
        while( current.len() > 1 )
        {
            // Holder next vector
            let mut next: Vec = Vec::new();
            // While current vector has components
            while( !current.is_empty() )
            {
                // If current vector has a length of 1, push the first item in current to next
                if( current.len() == 1 )
                    next.push( current.remove( 0 ) );
                else
                {
                    // Set left and right vectors to be the first two elements in current
                    let left: Vec = current.remove( 0 );
                    let right: Vec = current.remove( 0 );
                    // Combine their hash
                    let combined_hash = algorithm.hash_nodes( left.hash(), right.hash() );
                    // Set up this node with the combined hash and left and right as the respective children
                    let node = Tree::Node
                    {
                        hash: combined_hash.as_ref().into(),
                        left: Box::new( left ),
                        right: Box::new( right ),
                    };
                    // Push this node to next
                    next.push( node );
                }
            }
            // Increment heigh and refocus current
            ++height;
            current = next; 
        }
        // Store the root as the first element in current
        let root: Vec = current.remove( 0 );
        // Establish the MerkleTree with the calculated values
        MerkleTree
        {
            algorith: algorithm,
            root: root,
            height: height,
            count: count
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
    // Returns an iterator over the tree
    pub fn iterator( &self ) -> LeavesIterator<T>
    {
        return self.root.iter()
    }
    pub fn 
    // Hashes the root of the tree
    pub fn hash( &mut self ) -> &Vec<u8>
    {
        self.root.digest()
    }
    
}
