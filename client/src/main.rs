use clap::Parser;
use cli::{Args, SubCommand};
mod cli;
mod error;
mod file;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::List => {
            println!("List command");
        }
        SubCommand::Upload { files } => {
            let files = file::read_files(&files)?;
        }
        SubCommand::Download { file } => {
            println!("Download command");
        }
    }

    println!("Hello, world!");
    Ok(())
}
