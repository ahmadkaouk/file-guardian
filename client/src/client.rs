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
            self.stream.write_all(b"\0\0\0\0")?;
        }
        Ok(())
    }

    pub fn get_file(
        &mut self,
        root_hash: &str,
        index: usize,
    ) -> anyhow::Result<Vec<u8>> {
        // send download command
        self.stream.write_all(b"download")?;
        // send root hash
        self.stream.write_all(root_hash.as_bytes())?;
        // send index
        self.stream.write_all(&index.to_be_bytes())?;

        // receive file
        let mut file = vec![];
        self.stream.read_to_end(&mut file)?;

        Ok(file)
    }
}
