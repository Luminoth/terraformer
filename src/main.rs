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
        Commands::Init => init::run(false)?,
        Commands::InitAll => init::run(true)?,
        Commands::Plan => plan::run(false)?,
        Commands::PlanAll => plan::run(true)?,
        Commands::Apply => apply::run(false)?,
        Commands::ApplyAll => apply::run(true)?,
    };

    Ok(())
}
