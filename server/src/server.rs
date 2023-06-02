use anyhow::Result;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::store::{self, FileStore};

/// A server that listens for incoming connections and handles file uploads and
/// downloads.
pub struct Server {
    address: String,
}

impl Server {
    /// Creates a new `Server` instance.
    ///
    /// # Arguments
    ///
    /// * `address` - The address that the server listens on.
    pub fn new(address: &str) -> Server {
        Server {
            address: address.to_string(),
        }
    }

    /// Handles a file upload request from a client.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream that connects the server to the client.
    ///
    /// # Returns
    ///
    /// Returns a vector of vectors of bytes that represent the uploaded files.
    ///
    /// # Errors
    ///
    /// Returns an error if the file upload fails.
    async fn handle_upload(stream: &mut TcpStream) -> Result<Vec<Vec<u8>>> {
        // Read the number of files from the client
        let number_of_files: usize = stream.read_u64().await? as usize;

        // Read each file from the client and store it in a vector
        let mut res = vec![];
        for _ in 0..number_of_files {
            let file_size = stream.read_u64().await? as usize;
            let mut file = vec![0; file_size];
            stream.read_exact(&mut file).await?;
            res.push(file);
        }
        Ok(res)
    }

    /// Handles a file download request from a client.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream that connects the server to the client.
    /// * `store` - The file store that contains the files.
    ///
    /// # Errors
    ///
    /// Returns an error if the file download fails.
    async fn handle_download(
        stream: &mut TcpStream,
        store: &FileStore,
    ) -> Result<()> {
        // Read the root hash from the client
        let mut root_hash = [0; 64];
        stream.read_exact(&mut root_hash).await?;

        // Convert the root hash to a hex string
        let root_hash = std::str::from_utf8(&root_hash)?;

        // Read the index from the client
        let index = stream.read_u64().await? as usize;
        // get file from store
        let file = store.get_file(&root_hash, index)?;

        // Generate proof for file and export it as a vector of bytes
        let proof = store
            .get_tree(&root_hash)?
            .proof(index)?
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>();

        // send file size
        stream.write_all(&(file.len().to_be_bytes())).await?;
        // send file
        stream.write_all(&file).await?;
        // send proof
        stream.write_all(&proof).await?;

        Ok(())
    }

    async fn handle_client(
        stream: &mut TcpStream,
        store: &FileStore,
    ) -> Result<()> {
        let mut command = [0; 10];
        stream.read(&mut command).await?;
        let command =
            std::str::from_utf8(&command)?.trim_end_matches(char::from(0));

        match command {
            "upload" => {
                let files = Self::handle_upload(stream).await?;
                store.store_files(files)?;
            }
            "download" => {
                Self::handle_download(stream, store).await?;
            }
            _ => println!("Unknown command"),
        }

        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        loop {
            let (mut socket, _) = listener.accept().await?;
            let store = store::FileStore::new(PathBuf::from("server_store"))?;
            tokio::spawn(async move {
                Self::handle_client(&mut socket, &store)
                    .await
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
            });
        }
    }
}
