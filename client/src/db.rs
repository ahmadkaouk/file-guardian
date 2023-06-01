// src/db.rs
use serde_json::json;
use std::fs::OpenOptions;
use std::path::PathBuf;

/// A databse that stores file uploads with their corresponding root hashes.
/// The database is persisted to a JSON file.
pub struct Db {
    json_path: PathBuf,
}

impl Db {
    /// Creates a new `Db` instance
    pub fn new(json_path: PathBuf) -> Self {
        Self { json_path }
    }

    /// Persists filenames and their corresponding root hashes to a JSON file.
    /// Returns an `Err` if the JSON file can't be opened or written.
    pub fn persist(
        &self,
        root_hash: &str,
        files: &[PathBuf],
    ) -> anyhow::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.json_path)?;

        println!("root_hash: {}", root_hash);
        let files: Vec<String> = files
            .iter()
            .map(|path| {
                path.file_name()
                    .and_then(|n| n.to_str().map(|s| s.to_string()))
            })
            .flatten()
            .collect();

        let json = json!({
            "root_hash": root_hash,
            "files": files,
        });

        serde_json::to_writer_pretty(file, &json)?;
        Ok(())
    }

    /// Reads filenames and their corresponding root hashes from a JSON file.
    pub fn get_all(&self) -> anyhow::Result<()> {
        let Ok(file) = OpenOptions::new().read(true).open(&self.json_path) else {
            return Ok(());
        };
        let json: serde_json::Value = serde_json::from_reader(file)?;
        let root_hash = json["root_hash"].as_str().unwrap();
        let files = json["files"].as_array().unwrap();
        println!("root_hash: {}", root_hash);
        println!("files: {:?}", files);
        Ok(())
    }
}
