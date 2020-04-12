mod commands;

use structopt::StructOpt;

use crate::commands::Commands;

fn main() -> anyhow::Result<()> {
    let command = Commands::from_args();
    match command {
        Commands::Init => {}
        Commands::Apply => {}
    }

    Ok(())
}
