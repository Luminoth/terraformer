#![deny(warnings)]

mod apply;
mod commands;
mod destroy;
mod init;
mod output;
mod plan;
mod prompt;
mod state;
mod terraform;
mod validate;

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
        Commands::Destroy => destroy::run(false).await?,
        Commands::DestroyAll => destroy::run(true).await?,
        Commands::Output => output::run(false).await?,
        Commands::OutputAll => output::run(true).await?,
        Commands::State { args } => state::run(args, false).await?,
        Commands::StateAll { args } => state::run(args, true).await?,
        Commands::Validate { args } => validate::run(args, false).await?,
        Commands::ValidateAll { args } => validate::run(args, true).await?,
    };

    Ok(())
}
