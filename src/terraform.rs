use std::path::{Path, PathBuf};

use anyhow::bail;
use futures::future::{BoxFuture, FutureExt};
use tokio::fs;
use tokio::process::Command;
use tokio::stream::StreamExt;

use crate::prompt::prompt_error_yes_no;

async fn run_command<P, S>(path: P, command: S) -> anyhow::Result<()>
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
        .status()
        .await?
        .success()
        && !prompt_error_yes_no(
            format!("Terraform failed in {}, continue?", path.display()),
            false,
        )
        .await?
    {
        bail!("Terraform failed in {}", path.display());
    }

    Ok(())
}

fn run_command_in_dir<P, S>(
    dir: P,
    command: S,
    include_subdirs: bool,
) -> BoxFuture<'static, anyhow::Result<bool>>
where
    P: Into<PathBuf>,
    S: Into<String>,
{
    let dir = dir.into();
    let command = command.into();

    // TODO: pinning might fix the allocating / cloning here?
    // either that or stop recursing
    async move {
        let mut command_run = false;
        let mut command_run_subdirs = false;

        let mut entries = fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if include_subdirs
                    && run_command_in_dir(&path, command.clone(), include_subdirs).await?
                {
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
                        run_command(path.parent().unwrap(), &command).await?;
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
    .boxed()
}

/// Runs the given terraform command recursively starting in the given directory
pub async fn command_in_dir<P, S>(dir: P, command: S, include_subdirs: bool) -> anyhow::Result<()>
where
    P: Into<PathBuf>,
    S: Into<String>,
{
    if !run_command_in_dir(dir, command, include_subdirs).await? {
        bail!("No terraform files found!");
    }

    Ok(())
}
