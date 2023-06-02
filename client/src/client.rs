use anyhow::Result;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpStream;

/// A TCP client for uploading and downloading files to/from a server.
pub(crate) struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    /// Creates a new `TcpClient` that connects to the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to connect to, in the format `host:port`.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection fails.
    pub fn new(address: &str) -> Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(address)?,
        })
    }

    /// Sends the specified files to the server.
    ///
    /// # Arguments
    ///
    /// * `files` - A vector of byte vectors, where each byte vector represents
    ///   a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the upload fails.
    pub fn send_files(&mut self, files: Vec<Vec<u8>>) -> Result<()> {
        // send upload command
        self.stream.write_all(b"upload\0\0\0\0")?;

        // Send the number of files to be uploaded
        self.stream.write_all(&files.len().to_be_bytes())?;

        // Send each file
        for file in files {
            self.stream.write_all(&file.len().to_be_bytes())?;
            self.stream.write_all(&file)?;
        }
        Ok(())
    }

    /// Gets the file at the specified index from the server.
    ///
    /// # Arguments
    ///
    /// * `root_hash` - The root hash of the Merkle tree that contains the file.
    /// * `index` - The index of the file in the Merkle tree.
    ///
    /// # Errors
    ///
    /// Returns an error if the download fails.
    pub fn get_file(
        &mut self,
        root_hash: &str,
        index: usize,
    ) -> Result<Vec<u8>> {
        // send download command
        self.stream.write_all(b"download\0\0")?;
        // send root hash
        self.stream.write_all(root_hash.as_bytes())?;
        // send index
        self.stream.write_all(&index.to_be_bytes())?;

        // receive file size
        let mut file_size = [0; std::mem::size_of::<u64>()];
        self.stream.read_exact(&mut file_size)?;
        // receive file
        let mut file = vec![0; u64::from_be_bytes(file_size) as usize];
        self.stream.read_exact(&mut file)?;

        // decode root hash from hex string and convert to [u8; 32]
        let root_hash = hex::decode(root_hash)?
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid hash length"))?;

        // receive proof
        let mut proof = vec![];
        self.stream.read_to_end(&mut proof)?;
        let proof = proof
            .chunks_exact(32)
            .map(|chunk| {
                chunk
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid hash length"))
            })
            .collect::<Result<Vec<_>>>()?;

        // verify proof
        if !merkle_tree::MerkleTree::verify(index, &file, &root_hash, &proof) {
            return Err(anyhow::anyhow!("Invalid proof"));
        }

        Ok(file)
    }
}
