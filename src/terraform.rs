use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::bail;

use crate::prompt::prompt_error_yes_no;

fn run_command<P, S>(path: P, command: S) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let path = path.as_ref();

    println!(
        "Terraforming {} with {}...",
        path.display(),
        command.as_ref()
    );

    if !Command::new("terraform")
        .current_dir(path)
        .args(&[command.as_ref()])
        .status()?
        .success()
        && !prompt_error_yes_no(
            format!("Terraform failed in {}, continue?", path.display()),
            false,
        )?
    {
        bail!("Terraform failed in {}", path.display());
    }

    Ok(())
}

fn run_command_in_dir<P, S>(dir: P, command: S, include_subdirs: bool) -> anyhow::Result<bool>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut command_run = false;
    let mut command_run_subdirs = false;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if include_subdirs && run_command_in_dir(&path, command.as_ref(), include_subdirs)? {
                command_run_subdirs = true;
            }
            continue;
        }

        if command_run {
            continue;
        }

        match path.extension() {
            Some(extension) => {
                if extension == "tf" {
                    run_command(path.parent().unwrap(), command.as_ref())?;
                    command_run = true;

                    // early out if we won't descend into subdirs
                    if !include_subdirs {
                        break;
                    }
                }
            }
            None => continue,
        }
    }

    Ok(command_run || command_run_subdirs)
}

/// Runs the given terraform command recursively starting in the given directory
pub fn command_in_dir<P, S>(dir: P, command: S, include_subdirs: bool) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    if !run_command_in_dir(dir, command, include_subdirs)? {
        bail!("No terraform files found!");
    }

    Ok(())
}
