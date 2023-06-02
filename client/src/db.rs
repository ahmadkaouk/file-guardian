use anyhow::Result;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
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
        let mut uploads = HashMap::new();
        if !db_path.exists() {
            // Create the database directory
            std::fs::create_dir_all(&db_path)?;
            // Create the JSON file
            std::fs::File::create(db_path.join(db))?.write_all(b"{}")?;
        } else {
            let file = OpenOptions::new().read(true).open(db_path.join(db))?;
            uploads = serde_json::from_reader(file)?;
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

    /// Returns the path to the database.
    pub fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;

    #[test]
    fn test_persist() {
        let db_path = PathBuf::from("test_db1");
        let db_name = "test_db.json";
        let mut db = Db::new(db_path.clone(), db_name).unwrap();
        let root_hash = "root_hash";
        let files =
            vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        db.persist(root_hash, &files).unwrap();

        let db_file = db_path.join(db_name);
        let file = std::fs::File::open(db_file).unwrap();
        let uploads: HashMap<String, Vec<String>> =
            serde_json::from_reader(file).unwrap();
        assert_eq!(
            uploads.get(root_hash),
            Some(&vec!["file1.txt".to_string(), "file2.txt".to_string()])
        );

        remove_dir_all(db_path).unwrap();
    }

    #[test]
    fn test_get_uploads() {
        let db_path = PathBuf::from("test_db2");
        let db = "test_db.json";
        let mut db = Db::new(db_path.clone(), db).unwrap();
        let root_hash = "root_hash";
        let files =
            vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        db.persist(root_hash, &files).unwrap();

        let uploads = db.get_uploads();
        assert_eq!(
            uploads.get(root_hash),
            Some(&vec!["file1.txt".to_string(), "file2.txt".to_string()])
        );

        remove_dir_all(db_path).unwrap();
    }

    #[test]
    fn test_get_index() {
        let db_path = PathBuf::from("test_db3");
        let db = "test_db.json";
        let mut db = Db::new(db_path.clone(), db).unwrap();
        let root_hash = "root_hash";
        let files =
            vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        db.persist(root_hash, &files).unwrap();

        let index = db.get_index(root_hash, "file1.txt");
        assert_eq!(index, Some(0));

        let index = db.get_index(root_hash, "file2.txt");
        assert_eq!(index, Some(1));

        let index = db.get_index(root_hash, "file3.txt");
        assert_eq!(index, None);

        remove_dir_all(db_path).unwrap();
    }
}
