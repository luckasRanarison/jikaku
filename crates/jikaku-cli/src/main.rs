mod db;
mod kanjivg;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::db::{BuildDbArgs, build_db};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    BuildDb(BuildDbArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliCommand::BuildDb(args) => build_db(args),
    }
}
