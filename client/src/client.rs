use anyhow::Result;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpStream;

pub(crate) struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn new(address: &str) -> Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(address)?,
        })
    }

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

        let root_hash = hex::decode(root_hash)?
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid hash length"))?;

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
