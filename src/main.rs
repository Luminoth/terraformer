mod apply;
mod commands;
mod init;
mod plan;
mod prompt;
mod terraform;

use structopt::StructOpt;

use crate::commands::Commands;

fn main() -> anyhow::Result<()> {
    let command = Commands::from_args();
    match command {
        Commands::Init => init::run()?,
        Commands::Plan => plan::run()?,
        Commands::Apply => apply::run()?,
    };

    Ok(())
}
