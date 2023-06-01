use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

/// Remove duplicate elements from a vector.
pub fn dedup<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut set = HashSet::new();
    vec.into_iter().filter(|e| set.insert(e.clone())).collect()
}

/// Converts a byte array of length 32 to a hexadecimal string.    
pub fn bytes_to_hex_string(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

/// Get file names from a set of paths.
pub fn get_filenames(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|path| {
            path.file_name()
                .and_then(|n| n.to_str().map(|s| s.to_string()))
        })
        .flatten()
        .collect()
}

/// Pretty print of a HashMap of root hashes and files.
pub fn print_uploads(uploads: &HashMap<String, Vec<String>>) {
    println!("Root hashes and files:");
    for (root_hash, files) in uploads {
        println!("  {}: {:?}", root_hash, files);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_hex_string() {
        let bytes = [
            0x00, 0x01, 0x0a, 0x0f, 0x10, 0x1f, 0x7f, 0xff, 0xab, 0xcd, 0xef,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xa5, 0x5a, 0x3c, 0x7e, 0x8f, 0x9b,
            0xd0, 0xe1, 0xf2, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
        ];
        let expected =
            "00010a0f101f7fffabcdef1234567890a55a3c7e8f9bd0e1f233445566778899"
                .to_string();
        assert_eq!(bytes_to_hex_string(&bytes), expected);
    }

    #[test]
    fn test_get_filenames() {
        let paths = vec![
            PathBuf::from("foo.txt"),
            PathBuf::from("bar.txt"),
            PathBuf::from("/usr/baz"),
        ];
        let expected = vec![
            "foo.txt".to_string(),
            "bar.txt".to_string(),
            "baz".to_string(),
        ];
        assert_eq!(get_filenames(&paths), expected);
    }
}
