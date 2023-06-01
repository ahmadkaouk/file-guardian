use anyhow::Result;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
/// Reads the contents of the files specified by `paths`. If a duplicate path
/// is specified, it will be read once.
///
/// # Arguments
///
/// * `paths` - A slice of `PathBuf` that contain the paths of the files to
///   read.
///
/// # Errors
///
/// Returns an `Err` if any file can't be opened or read.
pub fn read_files(paths: &HashSet<PathBuf>) -> Result<Vec<Vec<u8>>> {
    paths
        .iter()
        .map(|path| {
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(buffer)
        })
        .collect()
}

/// Deletes the files specified by `paths`.
///
/// # Arguments
///
/// * `paths` - A slice of `PathBuf` that contain the paths of the files to
///   delete.
///
/// # Errors
///
/// Returns an `Err` if any file can't be deleted.
pub fn delete_files(paths: &[PathBuf]) -> Result<()> {
    for path in paths {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Writes `data` to the file specified by `path`.
///
/// # Arguments
///
/// * `path` - A `PathBuf` that contains the path of the file to write to.
/// * `data` - A byte slice that contains the data to write.
///
/// # Errors
///
/// Returns an `Err` if the file can't be created or written to.
pub fn write_file(path: &PathBuf, data: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::hashset;
    use super::*;
    use std::fs;
    use std::io::Write;
    #[test]
    fn test_read_files() {
        // Write a temporary file to read

        let mut file = File::create("/tmp/testfile").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        let paths = hashset![PathBuf::from("/tmp/testfile")];
        let contents = read_files(&paths).unwrap();
        // Clean up
        fs::remove_file("/tmp/testfile").unwrap();

        assert_eq!(contents[0], b"Hello, world!");
    }

    #[test]
    fn test_delete_files() {
        // Create a temporary file to delete
        File::create("/tmp/testfile1").unwrap();

        delete_files(&[PathBuf::from("/tmp/testfile1")]).unwrap();

        // Test that the file was deleted
        assert!(File::open("/tmp/testfile1").is_err());
    }

    #[test]
    fn test_write_file() {
        write_file(&PathBuf::from("/tmp/testfile2"), b"Hello, world!").unwrap();

        let mut buffer = Vec::new();
        // Test that the file was written correctly
        let mut file = File::open("/tmp/testfile2").unwrap();
        file.read_to_end(&mut buffer).unwrap();

        // Clean up
        fs::remove_file("/tmp/testfile2").unwrap();
        assert_eq!(buffer, b"Hello, world!");
    }
}
