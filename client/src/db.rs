// src/db.rs
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::utils;

/// A database that stores the root hash and the files. It persists the
/// root hash and the files to a JSON file.
pub struct Db {
    json_path: PathBuf,
    uploads: HashMap<String, Vec<String>>,
}

impl Db {
    /// Creates a new `Db` instance.
    pub fn new(json_path: PathBuf) -> Self {
        let uploads = Self::read_uploads(&json_path).unwrap_or_default();
        Self { json_path, uploads }
    }

    /// Persists the root hash and the files to the database.
    pub fn persist(
        &mut self,
        root_hash: &str,
        files: &[PathBuf],
    ) -> anyhow::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.json_path)?;

        self.uploads
            .insert(root_hash.to_string(), utils::get_filenames(files));

        Ok(serde_json::to_writer_pretty(file, &self.uploads)?)
    }

    /// Returns all the uploaded files.
    pub fn get_uploads(&self) -> &HashMap<String, Vec<String>> {
        &self.uploads
    }

    /// Reads the uploads from the JSON file.
    fn read_uploads(
        json_path: &PathBuf,
    ) -> anyhow::Result<HashMap<String, Vec<String>>> {
        let file = OpenOptions::new().read(true).open(json_path)?;
        Ok(serde_json::from_reader(file)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::remove_file, io::Write};
    use tempfile::tempdir;

    // Macro to create a HashMap
    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
            let mut _map = HashMap::new();
            $( _map.insert($key.to_string(), $val.to_vec()); )*
            _map
        }}
    }

    #[test]
    fn test_persist_and_get_uploads() {
        // Create a temporary directory for the JSON file
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("db.json");

        // Create a new Db instance
        let mut db = Db::new(json_path.clone());

        // Persist some files to the JSON file
        let root_hash = "abcd1234";
        let files =
            vec!(PathBuf::from("file1.txt"), PathBuf::from("file2.txt"));

        db.persist(root_hash, &files).unwrap();

        // Get the uploads from the JSON file
        let uploads = db.get_uploads();

        // Verify that the root hash and files are correct
        let expected_uploads = hashmap! {
            root_hash.to_string() => vec![
                "file2.txt".to_string(),
                "file1.txt".to_string(),
            ]
        };
        assert_eq!(*uploads, expected_uploads);

        // Clean up the temporary directory
        remove_file(json_path).unwrap();
    }

    #[test]
    fn test_persist_with_no_files() {
        // Create a temporary directory for the JSON file
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("db.json");

        // Create a new Db instance
        let mut db = Db::new(json_path.clone());

        // Persist an empty list of files to the JSON file
        let root_hash = "abcd1234";
        db.persist(root_hash, &[]).unwrap();

        // Get the uploads from the JSON file
        let uploads = db.get_uploads();

        // Verify that the root hash and files are correct
        let expected_uploads = hashmap! {
            root_hash.to_string() => vec![]
        };
        assert_eq!(*uploads, expected_uploads);

        // Clean up the temporary directory
        remove_file(json_path).unwrap();
    }

    #[test]
    fn test_get_uploads_with_no_json_file() {
        // Create a temporary directory for the JSON file
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("db.json");

        // Create a new Db instance
        let db = Db::new(json_path.clone());

        // Get the uploads from the non-existent JSON file
        let uploads = db.get_uploads();

        // Verify that the uploads are empty
        let expected_uploads = hashmap! {};
        assert_eq!(*uploads, expected_uploads);
    }

    #[test]
    fn test_read_uploads_with_invalid_json_file() {
        // Create a temporary directory for the JSON file
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("db.json");

        // Create an empty JSON file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&json_path)
            .unwrap();
        file.write_all(b"invalid json").unwrap();

        // Try to read the uploads from the invalid JSON file
        let result = Db::read_uploads(&json_path);

        // Verify that an error is returned
        assert!(result.is_err());

        // Clean up the temporary directory
        remove_file(json_path).unwrap();
    }
}
