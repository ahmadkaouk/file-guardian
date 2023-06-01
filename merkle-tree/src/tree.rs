use crate::error::MerkleTreeError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A Binary Merkle Tree.
///
/// The Merkle Tree struct consists a vector of vectors, where each inner
/// vector represents a level of the tree. The levels vector is populated
/// by iterating over the data blocks and hashing them to create the leaf
/// nodes, and then recursively hashing pairs of nodes to create the
/// non-leaf nodes until the root node is reached. The struct also provides
/// methods to retrieve the root hash of the tree, generate and verify
/// Merkle proofs, and compute the hash of the concatenation of two hashes.

type Hash = [u8; 32];

#[derive(Debug, Serialize, Deserialize)]
pub struct MerkleTree {
    levels: Vec<Vec<Hash>>,
}

impl MerkleTree {
    /// Creates a new Merkle Tree from the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of byte vectors representing the data blocks.
    ///
    /// # Errors
    ///
    /// Returns an error if the input data is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use merkle_tree::MerkleTree;
    ///
    /// let data = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    /// let tree = MerkleTree::new(&data).unwrap();
    /// ```
    pub fn new(data: &[impl AsRef<[u8]>]) -> Result<Self, MerkleTreeError> {
        if data.is_empty() {
            return Err(MerkleTreeError::EmptyData);
        }
        let mut levels: Vec<Vec<Hash>> =
            Vec::with_capacity((data.len() as f64).log2().ceil() as usize);

        levels.extend(std::iter::successors(
            Some(data.into_iter().map(Self::hash).collect::<Vec<Hash>>()),
            |level| match level.len() {
                0 | 1 => None,
                _ => Some(
                    level
                        .chunks(2)
                        .map(|chunk| match chunk.len() {
                            1 => Self::hash(&chunk[0]),
                            _ => Self::hash_nodes(&chunk[0], &chunk[1]),
                        })
                        .collect(),
                ),
            },
        ));

        Ok(Self { levels })
    }

    /// Returns the root hash of the Merkle Tree.
    pub fn root(&self) -> Option<&Hash> {
        self.levels.last().and_then(|level| level.first())
    }

    /// Returns the Merkle proof for the data block at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the data block to generate the proof for.
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use merkle_tree::MerkleTree;
    ///
    /// let data = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    /// let tree = MerkleTree::new(&data).unwrap();
    /// let proof = tree.proof(1).unwrap();
    /// ```
    pub fn proof(&self, index: usize) -> Result<Vec<Hash>, MerkleTreeError> {
        if index >= self.levels[0].len() {
            return Err(MerkleTreeError::InvalidIndex);
        }

        Ok(self.levels[0..self.levels.len() - 1]
            .iter()
            .fold(
                (index, Vec::with_capacity(self.levels.len())),
                |(i, mut proof), level| {
                    let sibling_index = if i % 2 == 0 { i + 1 } else { i - 1 };
                    proof.push(level[sibling_index].clone());
                    (i / 2, proof)
                },
            )
            .1)
    }

    /// Verifies the Merkle proof for the data block at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the data block to verify the proof for.
    /// * `data` - The data block to verify the proof for.
    /// * `root` - The root hash of the Merkle tree.
    /// * `proof` - The Merkle proof for the data block.
    ///
    /// # Returns
    ///
    /// Returns a boolean indicating whether the proof is valid or not.
    ///
    /// # Examples
    pub fn verify(
        index: usize,
        data: &[u8],
        root: &Hash,
        proof: &[Hash],
    ) -> bool {
        let (_, hash) = proof.iter().fold(
            (index, Self::hash(data)),
            |(i, hash), sibling| match i % 2 {
                0 => (i / 2, Self::hash_nodes(&hash, sibling)),
                1 => (i / 2, Self::hash_nodes(sibling, &hash)),
                _ => unreachable!(),
            },
        );

        hash == *root
    }

    /// Computes the hash of the concatenation of two hashes.
    fn hash_nodes(left: &Hash, right: &Hash) -> Hash {
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(left);
        combined[32..].copy_from_slice(right);
        Self::hash(&combined)
    }

    ///
    fn hash<T: AsRef<[u8]>>(data: T) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty_data() {
        let data: &[&[u8]] = &[];
        let result = MerkleTree::new(&data);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MerkleTreeError::EmptyData);
    }

    #[test]
    fn test_new_single_leaf() {
        let data = vec![vec![1, 2, 3]];
        let tree = MerkleTree::new(&data).unwrap();
        assert_eq!(tree.root().unwrap(), &MerkleTree::hash(&data[0]));
    }

    #[test]
    fn test_new_odd_number_of_leaves() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let tree = MerkleTree::new(&data).unwrap();
        assert_eq!(
            tree.root().unwrap(),
            &MerkleTree::hash_nodes(
                &MerkleTree::hash_nodes(
                    &MerkleTree::hash(&data[0]),
                    &MerkleTree::hash(&data[1])
                ),
                &MerkleTree::hash(&MerkleTree::hash(&data[2])),
            )
        );
    }

    #[test]
    fn test_invalid_index() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let tree = MerkleTree::new(&data).unwrap();
        let result = tree.proof(3);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), MerkleTreeError::InvalidIndex);
    }

    #[test]
    fn test_merkle_tree_verify() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];
        let tree = MerkleTree::new(&data).unwrap();
        let root = tree.root().unwrap();
        // Check the verification of the second leaf node
        let proof = tree.proof(1).unwrap();
        let verified = MerkleTree::verify(1, &[4, 5, 6], root, &proof);
        assert!(verified);
    }
}
