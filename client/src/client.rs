use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpStream;

pub(crate) struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn new(address: String) -> Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(address)?,
        })
    }

    pub fn send_files(&mut self, files: Vec<Vec<u8>>) -> Result<()> {
        // send upload command
        self.stream.write_all(b"upload\0\0\0\0")?;

        // Send the number of files to be uploaded
        self.stream.write_all(&files.len().to_be_bytes())?;

        // send file names
        for file in files {
            self.stream.write_all(&file.len().to_be_bytes())?;
            self.stream.write_all(&file)?;
            self.stream.write_all(b"\0\0\0\0")?;
        }
        Ok(())
    }

    fn download_file(&mut self, file_name: &str) -> std::io::Result<()> {
        // send download command
        self.stream.write_all(b"download")?;
        // send file name
        self.stream.write_all(file_name.as_bytes())?;

        // receive file data and write it to a local file
        let mut file_data = vec![];
        self.stream.read_to_end(&mut file_data)?;

        let mut local_file = File::create(file_name)?;
        local_file.write_all(&file_data)?;
        Ok(())
    }
}
