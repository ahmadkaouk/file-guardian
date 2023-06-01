use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// List all the uploaded files
    List,
    /// Upload one or more files(s) to the server
    Upload {
        #[arg(short, long, value_name = "FILE", action = clap::ArgAction::Append)]
        #[clap(required = true)]
        files: Vec<PathBuf>,
    },
    /// Download a file from the server
    Download {
        #[arg(short, long, value_name = "FILE")]
        #[clap(required = true)]
        file: String,
    },
}
