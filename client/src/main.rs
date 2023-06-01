use clap::Parser;
use cli::{Args, SubCommand};
use db::Db;
use merkle_tree::MerkleTree;
use std::path::PathBuf;

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
            // Remove duplicates
            let files = utils::dedup(files);
            // Read the files and compute the root hash
            let data = file::read_files(&files)?;
            let root_hash = MerkleTree::new(&data)?
                .root()
                .map(|r| utils::bytes_to_hex_string(r))
                .ok_or(anyhow::anyhow!("Root Hash could not be computed"))?;

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
