use crate::{error::MerkleTreeError, hasher::Hasher};

/// A Merkle Tree is a binary tree where each leaf node represents a data block
/// and each non-leaf node represents a hash of its children nodes. This struct
/// represents a Merkle Tree with a generic Hasher type parameter.
#[derive(Debug)]
pub struct MerkleTree<H: Hasher> {
    levels: Vec<Vec<H::Hash>>,
}

impl<H: Hasher> MerkleTree<H> {
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
    /// use merkle_tree::{MerkleTree, Sha256Hasher};
    ///
    /// let data = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    /// let tree = MerkleTree::<Sha256Hasher>::new(&data).unwrap();
    /// ```
    pub fn new(data: &Vec<Vec<u8>>) -> Result<Self, MerkleTreeError> {
        if data.is_empty() {
            return Err(MerkleTreeError::EmptyData);
        }
        let mut levels: Vec<Vec<H::Hash>> =
            Vec::with_capacity((data.len() as f64).log2().ceil() as usize);

        levels.extend(std::iter::successors(
            Some(data.into_iter().map(H::hash).collect::<Vec<H::Hash>>()),
            |level| match level.len() {
                0 | 1 => None,
                _ => Some(
                    level
                        .chunks(2)
                        .map(|chunk| match chunk.len() {
                            1 => H::hash(&chunk[0]),
                            _ => Self::hash_nodes(&chunk[0], &chunk[1]),
                        })
                        .collect(),
                ),
            },
        ));

        Ok(Self { levels })
    }

    /// Returns the root hash of the Merkle Tree.
    pub fn root(&self) -> Option<&H::Hash> {
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
    /// use merkle_tree::{MerkleTree, Sha256Hasher};
    ///
    /// let data = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    /// let tree = MerkleTree::<Sha256Hasher>::new(&data).unwrap();
    /// let proof = tree.proof(1).unwrap();
    /// ```
    pub fn proof(&self, index: usize) -> Result<Vec<H::Hash>, MerkleTreeError> {
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
        root: &H::Hash,
        proof: &[H::Hash],
    ) -> bool {
        let (_, hash) =
            proof
                .iter()
                .fold((index, H::hash(data)), |(i, hash), sibling| {
                    match i % 2 {
                        0 => (i / 2, Self::hash_nodes(&hash, sibling)),
                        1 => (i / 2, Self::hash_nodes(sibling, &hash)),
                        _ => unreachable!(),
                    }
                });

        hash == *root
    }

    /// Computes the hash of the concatenation of two hashes.
    fn hash_nodes(left: &H::Hash, right: &H::Hash) -> H::Hash {
        let mut combined =
            Vec::with_capacity(left.as_ref().len() + right.as_ref().len());
        combined.extend(left.as_ref());
        combined.extend(right.as_ref());
        H::hash(&combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::Sha256Hasher;

    #[test]
    fn test_merkle_tree_creation() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];
        let tree = MerkleTree::<Sha256Hasher>::new(&data).unwrap();
        assert_eq!(tree.levels.len(), 3);
    }

    #[test]
    fn test_merkle_tree_verify() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];
        let tree = MerkleTree::<Sha256Hasher>::new(&data).unwrap();
        let root = tree.root().unwrap();
        // Check the verification of the second leaf node
        let proof = tree.proof(1).unwrap();
        let verified =
            MerkleTree::<Sha256Hasher>::verify(1, &[4, 5, 6], root, &proof);
        assert!(verified);
    }
}