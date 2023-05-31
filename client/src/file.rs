use anyhow::Result;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
/// Reads the contents of the files specified by `paths`.
///
/// # Arguments
///
/// * `paths` - A slice of `PathBuf` that contain the paths of the files to
///   read.
///
/// # Errors
///
/// Returns an `Err` if any file can't be opened or read.
pub fn read_files(paths: &[PathBuf]) -> Result<Vec<Vec<u8>>> {
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
