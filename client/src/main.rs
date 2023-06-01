use clap::Parser;
use cli::{Args, SubCommand};
use db::Db;
use merkle_tree::{MerkleTree, Sha256Hasher};

use std::path::PathBuf;

mod cli;
mod db;
mod error;
mod file;

/// List all the uploaded files.
fn list(db: &Db) -> anyhow::Result<()> {
    db.get_all()?;
    Ok(())
}

pub fn bytes_to_hex_string(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let db = Db::new(PathBuf::from("db.json"));
    match args.subcmd {
        SubCommand::List => {
            list(&db)?;
        }
        SubCommand::Upload { files } => {
            // Read the files and compute the root hash
            let data = file::read_files(&files)?;
            let root_hash = MerkleTree::<Sha256Hasher>::new(&data)?
                .root()
                .map(|r| bytes_to_hex_string(r))
                .ok_or(anyhow::anyhow!("No root hash"))?;
            
            db.persist(&root_hash, &files)?;
        }
        SubCommand::Download { file: _ } => {
            // download(&file)?;
        }
    }

    Ok(())
}
