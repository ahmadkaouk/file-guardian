use std::path::PathBuf;

use anyhow::Result;
use merkle_tree::MerkleTree;
use tokio::io::AsyncReadExt;
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

    async fn handle_upload(stream: TcpStream) -> Result<Vec<Vec<u8>>> {
        let mut res = vec![];

        let mut stream = stream;

        let mut number_of_files = [0_u8; std::mem::size_of::<usize>()];
        stream.read_exact(&mut number_of_files).await?;
        let number_of_files = usize::from_be_bytes(number_of_files);

        for _ in 0..number_of_files {
            let mut file_size = [0_u8; std::mem::size_of::<usize>()];
            stream.read_exact(&mut file_size).await?;
            let file_size = usize::from_be_bytes(file_size);
            let mut file = vec![0; file_size];
            stream.read_exact(&mut file).await?;
            res.push(file);
         }
        Ok(res)
    }

    async fn handle_download(
        _stream: TcpStream,
        _filename: String,
    ) -> Result<()> {
        // implement download functionality here
        println!("Downloading file");
        Ok(())
    }

    async fn handle_client(mut stream: TcpStream, store: FileStore) -> Result<()> {
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
                let mut filename = String::new();
                stream.read_to_string(&mut filename).await?;
                Self::handle_download(stream, filename).await?;
            }
            _ => println!("Unknown command"),
        }

        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            let store = store::FileStore::new(PathBuf::from("server_store"))?;
            tokio::spawn(async move {
                Self::handle_client(socket, store)
                    .await
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
            });
        }
    }
}
