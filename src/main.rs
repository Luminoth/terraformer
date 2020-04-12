mod apply;
mod commands;
mod init;
mod plan;

use std::fs;
use std::path::Path;
use std::process::Command;

use structopt::StructOpt;

use crate::commands::Commands;

pub fn run_tf_cmd<P, S>(path: P, command: S) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    println!(
        "Terraforming {} with {}...",
        path.as_ref().display(),
        command.as_ref()
    );

    let _status = Command::new("terraform")
        .current_dir(path)
        .args(&[command.as_ref()])
        .status()?;

    Ok(())
}

/// Runs the given terraform command recursively starting in the given directory
pub fn run_tf_cmd_in_dir<P, S>(dir: P, command: S) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut command_run = false;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            run_tf_cmd_in_dir(&path, command.as_ref())?;
            continue;
        }

        if command_run {
            continue;
        }

        match path.extension() {
            Some(extension) => {
                if extension == "tf" {
                    run_tf_cmd(path.parent().unwrap(), command.as_ref())?;
                    command_run = true;
                }
            }
            None => continue,
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let command = Commands::from_args();
    match command {
        Commands::Init => init::run()?,
        Commands::Plan => plan::run()?,
        Commands::Apply => apply::run()?,
    };

    Ok(())
}
