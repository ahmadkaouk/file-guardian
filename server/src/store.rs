use anyhow::{anyhow, Result};
use merkle_tree::MerkleTree;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

/// A struct that represents a file store.
#[derive(Clone)]
pub struct FileStore {
    root_dir: PathBuf,
}

impl FileStore {
    /// Creates a new instance of `FileStore` with the given root directory.
    ///
    /// # Arguments
    ///
    /// * `root_dir` - The root directory for the file store.
    pub fn new(root_dir: impl AsRef<Path>) -> Result<Self> {
        // Create the root directory if it doesn't exist
        if !root_dir.as_ref().exists() {
            fs::create_dir_all(&root_dir)?;
        }

        Ok(Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        })
    }

    /// Stores the given files in the file store and returns the root hash of
    /// the Merkle tree.
    ///
    /// # Arguments
    ///
    /// * `files` - A vector containing the file data as `Vec<u8>`.
    pub fn store_files(&self, files: Vec<Vec<u8>>) -> Result<String> {
        // Compute the Merkle tree
        let tree = MerkleTree::new(&files)?;

        // Compute the root hash, and convert it to a hex string
        let root_hash = tree
            .root()
            .map(|r| hex::encode(r))
            .ok_or_else(|| anyhow!("Root Hash could not be computed"))?;

        // Create a new directory for the files, named after the root hash
        let dir = self.root_dir.join(&root_hash);
        fs::create_dir_all(&dir)?;

        // Store the files
        for (i, file_data) in files.iter().enumerate() {
            fs::write(dir.join(i.to_string()), file_data)?;
        }

        // Serialize and store the Merkle tree
        let tree_json = serde_json::to_string(&tree)?;
        let mut file = File::create(dir.join("tree.json"))?;
        file.write_all(tree_json.as_bytes())?;

        Ok(root_hash)
    }

    /// Returns the Merkle tree with the given root hash.
    ///
    /// # Arguments
    ///
    /// * `root_hash` - The root hash of the Merkle tree to retrieve.
    pub fn get_tree(&self, root_hash: &str) -> Result<MerkleTree> {
        let dir = self.root_dir.join(root_hash);
        let tree_json = fs::read_to_string(dir.join("tree.json"))?;
        let tree: MerkleTree = serde_json::from_str(&tree_json)?;
        Ok(tree)
    }

    /// Returns the file with the given index and root hash.
    ///
    /// # Arguments
    ///
    /// * `root_hash` - The root hash of the Merkle tree containing the file.
    /// * `index` - The index of the file to retrieve.
    pub fn get_file(&self, root_hash: &str, index: usize) -> Result<Vec<u8>> {
        let dir = self.root_dir.join(root_hash);
        println!("dir: {:?}", dir);
        let file_path = dir.join(index.to_string());
        Ok(fs::read(file_path)?)
    }
}
