use anyhow::Result;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::store::{self, FileStore};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Server {
        Server {
            address: address.to_string(),
        }
    }

    async fn handle_upload(stream: &mut TcpStream) -> Result<Vec<Vec<u8>>> {
        let mut res = vec![];
        let number_of_files: usize = stream.read_u64().await? as usize;

        for _ in 0..number_of_files {
            let file_size = stream.read_u64().await? as usize;
            let mut file = vec![0; file_size];
            stream.read_exact(&mut file).await?;
            res.push(file);
        }
        Ok(res)
    }

    async fn handle_download(
        stream: &mut TcpStream,
        store: &FileStore,
    ) -> Result<()> {
        // receive root hash
        let mut root_hash = [0; 64];
        stream.read_exact(&mut root_hash).await?;
       
        // convert root hash to hex string, every 2 bytes is a hex digit
        let root_hash = std::str::from_utf8(&root_hash)?;

        // receive index
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
