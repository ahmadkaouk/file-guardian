use crate::file_handler::FileHandler;
use crate::merkle_tree::{MerkleTree, Proof};
use sha2::{Digest, Sha256};
use std::io::{self, Read};
use std::path::PathBuf;

pub struct Verifier {
    file_handler: FileHandler,
}

impl Verifier {
    /// Creates a new `Verifier`.
    pub fn new() -> Self {
        let file_handler = FileHandler;
        Self { file_handler }
    }

    /// Generates a Merkle proof for a set of files.
    ///
    /// # Arguments
    ///
    /// * `paths` - A slice of `PathBuf` that contains the paths of the files to
    ///   verify.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if any file can't be opened or read.
    pub fn generate_proof(
        &self,
        data: &[&[u8]],
    ) -> io::Result<(Vec<u8>, Proof)> {
        let contents = self.file_handler.read_files(paths)?;
        let hashes = contents
            .iter()
            .map(|content| {
                let mut hasher = Sha256::new();
                hasher.update(content);
                hasher.finalize().to_vec()
            })
            .collect::<Vec<_>>();

        let tree = MerkleTree::new(&hashes);
        let root_hash = tree.root_hash();
        let proof = tree.generate_proof();

        Ok((root_hash, proof))
    }

    /// Verifies a file against a Merkle proof.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file to verify.
    /// * `root_hash` - The root hash of the Merkle tree.
    /// * `proof` - The Merkle proof.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the file can't be opened or read.
    pub fn verify(
        &self,
        path: &PathBuf,
        root_hash: &[u8],
        proof: &Proof,
    ) -> io::Result<bool> {
        let content = self.file_handler.read_file(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hasher.finalize().to_vec();

        Ok(MerkleTree::verify_proof(&hash, root_hash, proof))
    }
}
