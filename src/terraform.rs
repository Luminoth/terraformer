use std::fs;
use std::path::Path;
use std::process::Command;

use crate::prompt::prompt_yes_no;

fn run_command<P, S>(path: P, command: S) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    println!(
        "Terraforming {} with {}...",
        path.as_ref().display(),
        command.as_ref()
    );

    if !Command::new("terraform")
        .current_dir(path)
        .args(&[command.as_ref()])
        .status()?
        .success()
    {
        if !prompt_yes_no("Terraform failed, continue", false)? {}
    }

    Ok(())
}

/// Runs the given terraform command recursively starting in the given directory
pub fn command_in_dir<P, S>(dir: P, command: S, include_subdirs: bool) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut command_run = false;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if include_subdirs {
                command_in_dir(&path, command.as_ref(), include_subdirs)?;
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
                        return Ok(());
                    }
                }
            }
            None => continue,
        }
    }

    Ok(())
}
