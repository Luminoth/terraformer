mod apply;
mod commands;
mod init;
mod plan;
mod prompt;
mod terraform;

use structopt::StructOpt;

use crate::commands::Commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = Commands::from_args();
    match command {
        Commands::Init => init::run(false).await?,
        Commands::InitAll => init::run(true).await?,
        Commands::Plan => plan::run(false).await?,
        Commands::PlanAll => plan::run(true).await?,
        Commands::Apply => apply::run(false).await?,
        Commands::ApplyAll => apply::run(true).await?,
    };

    Ok(())
}
