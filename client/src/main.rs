use clap::Parser;
use cli::{Args, SubCommand};
use db::Db;
use merkle_tree::MerkleTree;
use std::{fs, path::PathBuf};

mod cli;
mod client;
mod db;
mod error;

#[macro_use]
mod utils;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut db = Db::new(PathBuf::from("client_store"), "uploads.json")?;

    match args.subcmd {
        SubCommand::List => {
            let uploads = db.get_uploads();
            utils::print_uploads(uploads);
        }
        SubCommand::Upload { files, server_addr } => {
            upload(files, &server_addr, &mut db)?;
        }
        SubCommand::Download {
            root_hash,
            file,
            server_addr,
        } => {
            download(&root_hash, &file, &server_addr, &db)?;
        }
    }

    Ok(())
}

fn upload(
    files: Vec<PathBuf>,
    server_addr: &str,
    db: &mut Db,
) -> Result<(), anyhow::Error> {
    // Remove duplicates
    let files = utils::dedup(files);

    // Read the files
    let data = files
        .iter()
        .map(utils::read)
        .collect::<Result<Vec<Vec<u8>>, _>>()?;

    // Compute the root hash
    let root_hash = MerkleTree::new(&data)?
        .root()
        .map(utils::bytes_to_hex_string)
        .ok_or(anyhow::anyhow!("Root Hash could not be computed"))?;

    let mut client = client::TcpClient::new(server_addr)?;
    client.send_files(data)?;
    db.persist(&root_hash, &files)?;

    for file in files {
        std::fs::remove_file(file)?;
    }
    Ok(())
}

fn download(
    root_hash: &str,
    filename: &str,
    server_addr: &str,
    db: &Db,
) -> Result<(), anyhow::Error> {
    // Get the index of the file
    let index = db.get_index(root_hash, filename).ok_or(anyhow::anyhow!(
        "File {} not found in root hash {}",
        filename,
        root_hash
    ))?;

    // Get the file from the server
    let mut client = client::TcpClient::new(server_addr)?;
    let file = client.get_file(root_hash, index)?;

    // write the file to disk
    let dir = PathBuf::from("client_store");
    // create the directory if it doesn't exist
    if !dir.exists() {
        std::fs::create_dir(&dir)?;
    }
    // write the file
    fs::write(dir.join(filename), file)?;

    Ok(())
}
