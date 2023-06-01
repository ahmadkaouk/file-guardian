use anyhow::Result;
use merkle_tree::MerkleTree;
use std::{fs, path::PathBuf};

/// A struct that represents a file store.
pub struct FileStore {
    root_dir: PathBuf,
}

impl FileStore {
    /// Creates a new instance of `FileStore` with the given root directory.
    pub fn new(root_dir: PathBuf) -> Result<Self> {
        // Create the root directory if it doesn't exist
        if !root_dir.exists() {
            fs::create_dir_all(&root_dir)?;
        }

        Ok(Self { root_dir })
    }

    /// Stores the given files in the file store and returns the root hash of
    /// the Merkle tree.
    ///
    /// # Arguments
    ///
    /// * `files` - A `HashMap` containing the file names and their data as
    ///   `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the root hash of the Merkle tree as a `String` if
    /// the operation was successful, or a `Box<dyn std::error::Error>` if
    /// an error occurred.
    pub fn store_files(&self, files: Vec<Vec<u8>>) -> anyhow::Result<String> {
        // Compute the Merkle tree
        let tree = MerkleTree::new(&files)?;

        // Compute the root hash, and convert it to a hex string
        let root_hash = tree
            .root()
            .map(|r| r.iter().map(|byte| format!("{:02x}", byte)).collect())
            .ok_or(anyhow::anyhow!("Root Hash could not be computed"))?;

        // Create a new directory for the files, named after the root hash
        let dir = self.root_dir.join(&root_hash);
        fs::create_dir_all(&dir)?;

        // Store the files
        for (i, file_data) in files.iter().enumerate() {
            fs::write(dir.join(i.to_string()), file_data)?;
        }

        // Serialize and store the Merkle tree
        let tree_json = serde_json::to_string(&tree)?;
        fs::write(dir.join("tree.json"), tree_json)?;

        Ok(root_hash)
    }
}
