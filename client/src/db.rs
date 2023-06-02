use anyhow::Result;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::PathBuf;

/// A database that stores the root hash and the files. It persists the
/// root hash and the files to a JSON file.
pub struct Db {
    db_path: PathBuf,
    db: String,
    uploads: HashMap<String, Vec<String>>,
}

impl Db {
    /// Creates a new `Db` instance.
    pub fn new(db_path: PathBuf, db: &str) -> Result<Self> {
        let uploads = HashMap::new();
        if !db_path.exists() {
            // Create the database directory
            std::fs::create_dir_all(&db_path)?;
            // Create the JSON file
            std::fs::File::create(db_path.join(db))?;
        } else {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(db_path.join(db))?;
            serde_json::to_writer_pretty(file, &uploads)?;
        }
        Ok(Self {
            db_path,
            db: db.to_string(),
            uploads,
        })
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
            .open(&self.db_path.join(self.db.clone()))?;

        self.uploads.insert(
            root_hash.to_string(),
            files
                .iter()
                .filter_map(|f| {
                    f.file_name()
                        .and_then(|n| n.to_str().map(|s| s.to_string()))
                })
                .collect(),
        );

        Ok(serde_json::to_writer_pretty(file, &self.uploads)?)
    }

    /// Returns all the uploaded files.
    pub fn get_uploads(&self) -> &HashMap<String, Vec<String>> {
        &self.uploads
    }

    /// Get index of the file in the list of files.
    pub fn get_index(&self, root_hash: &str, file_name: &str) -> Option<usize> {
        self.uploads
            .get(root_hash)
            .and_then(|files| files.iter().position(|f| f == file_name))
    }

    #[cfg(test)]
    /// Reads the list of uploaded files from the DB.
    fn read_uploads(&self) -> anyhow::Result<HashMap<String, Vec<String>>> {
        let file = OpenOptions::new().read(true).open(self.db.clone())?;
        Ok(serde_json::from_reader(file)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
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
        let temp_dir = tempdir().unwrap().into_path();
        // Create a new Db instance
        let mut db = Db::new(temp_dir, "uploads.json").unwrap();

        // Persist some files to the JSON file
        let root_hash = "abcd1234";
        let files =
            vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];

        db.persist(root_hash, &files).unwrap();

        // Get the uploads from the JSON file
        let uploads = db.get_uploads();

        // Verify that the root hash and files are correct
        let expected_uploads = hashmap! {
            root_hash.to_string() => vec![
                "file1.txt".to_string(),
                "file2.txt".to_string(),
            ]
        };
        assert_eq!(*uploads, expected_uploads);
    }

    #[test]
    fn test_persist_with_no_files() {
        // Create a temporary directory for the JSON file
        let temp_dir = tempdir().unwrap().into_path();
        // Create a new Db instance
        let mut db = Db::new(temp_dir, "uploads.json").unwrap();

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
    }

    #[test]
    fn test_get_uploads_with_no_json_file() {
        // Create a temporary directory for the JSON file
        let temp_dir = tempdir().unwrap().into_path();
        // Create a new Db instance
        let db = Db::new(temp_dir, "uploads.json").unwrap();

        // Get the uploads from the non-existent JSON file
        let uploads = db.get_uploads();

        // Verify that the uploads are empty
        let expected_uploads = hashmap! {};
        assert_eq!(*uploads, expected_uploads);
    }

    #[test]
    fn test_read_uploads_with_invalid_json_file() {
        // Create a temporary directory for the JSON file
        let temp_dir = tempdir().unwrap().into_path();

        // Create an empty JSON file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&temp_dir.join("uploads.json"))
            .unwrap();
        file.write_all(b"invalid json").unwrap();

        // Try to read the uploads from the invalid JSON file
        let db = Db::new(temp_dir, "uploads.json").unwrap();
        let result = db.read_uploads();

        // Verify that an error is returned
        assert!(result.is_err());
    }
}
