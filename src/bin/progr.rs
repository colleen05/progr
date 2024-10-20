use std::io;

mod subcommands;
use subcommands::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about = "Progr is a Git-friendly, filesystem-based command-line utility for kanban style progress tracking.\nDeveloped by Colleen (@colleen05 on GitHub).\nDistributed under the MIT license.",
    long_about = None
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialises a .progress directory with starter stages (concept, todo, progress, and done).
    Init(InitArgs),
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => init(args)?,
    }

    Ok(())
}
