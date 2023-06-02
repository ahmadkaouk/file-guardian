use std::{collections::{HashMap, HashSet}, path::PathBuf, fs::File, io::Read};
use anyhow::Result;

/// Remove duplicate elements from a vector.
pub fn dedup<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut set = HashSet::new();
    vec.into_iter().filter(|e| set.insert(e.clone())).collect()
}

/// Pretty print of a HashMap of root hashes and files.
pub fn print_uploads(uploads: &HashMap<String, Vec<String>>) {
    println!("Root hashes and files:");
    for (root_hash, files) in uploads {
        println!("  {}: {:?}", root_hash, files);
    }
}

/// Read a file from a path and return its content as a vector of bytes.
pub fn read(path: &PathBuf) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
