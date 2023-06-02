use clap::Parser;
use cli::{Args, SubCommand};
use db::Db;
use merkle_tree::MerkleTree;
use std::path::PathBuf;

mod cli;
mod client;
mod db;
mod error;
mod file;

#[macro_use]
mod utils;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut db = Db::new(PathBuf::from("db.json"));

    match args.subcmd {
        SubCommand::List => {
            let uploads = db.get_uploads();
            // Print the root hash and the files
            utils::print_uploads(uploads);
        }
        SubCommand::Upload { files, server_addr } => {
            // Remove duplicates
            let files = utils::dedup(files);

            // Read the files
            let data = files
                .iter()
                .map(|f| file::read(f))
                .collect::<Result<Vec<Vec<u8>>, _>>()?;

            // Compute the root hash
            let root_hash = MerkleTree::new(&data)?
                .root()
                .map(utils::bytes_to_hex_string)
                .ok_or(anyhow::anyhow!("Root Hash could not be computed"))?;

            let mut client = client::TcpClient::new(server_addr)?;
            client.send_files(data)?;
            db.persist(&root_hash, &files)?;
            // Delete the files

            for file in files {
                std::fs::remove_file(file)?;
            }
        }
        SubCommand::Download {
            file: _,
            server_addr: _,
        } => {
            // download(&file)?;
        }
    }

    Ok(())
}
