use clap::Parser;
use cli::{Args, SubCommand};
use db::Db;
use merkle_tree::{MerkleTree, Sha256Hasher};
use std::{collections::HashSet, path::PathBuf};

mod cli;
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
            utils::print_uploads(&uploads);
        }
        SubCommand::Upload { files } => {
            let files: HashSet<_> = files.into_iter().collect();
            // Read the files and compute the root hash
            let data = file::read_files(&files)?;
            let root_hash = MerkleTree::<Sha256Hasher>::new(&data)?
                .root()
                .map(|r| utils::bytes_to_hex_string(r))
                .ok_or(anyhow::anyhow!("No root hash"))?;

            db.persist(&root_hash, &files)?;

            // Delete the files
            file::delete_files(&files)?;
        }
        SubCommand::Download { file: _ } => {
            // download(&file)?;
        }
    }

    Ok(())
}
