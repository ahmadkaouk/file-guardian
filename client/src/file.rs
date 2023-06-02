use anyhow::Result;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
/// Reads the contents of the file specified by `path`.
///
/// # Arguments
///
/// * `path` - A `PathBuf` that contains the path of the file to read.
///
/// # Errors
///
/// Returns an `Err` if the file can't be opened or read.
pub fn read(path: &PathBuf) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
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
    use super::*;
    use std::fs;
    use std::io::Write;
    #[test]
    fn test_read_files() {
        // Write a temporary file to read

        let mut file = File::create("/tmp/testfile").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        let paths = vec![PathBuf::from("/tmp/testfile")];
        let contents = paths
            .iter()
            .map(|path| read(path).unwrap())
            .collect::<Vec<Vec<u8>>>();
        // Clean up
        fs::remove_file("/tmp/testfile").unwrap();

        assert_eq!(contents[0], b"Hello, world!");
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
