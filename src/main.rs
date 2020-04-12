mod options;

use structopt::StructOpt;

use crate::options::Command;

fn main() -> anyhow::Result<()> {
    let command = Command::from_args();
    match command {
        Command::Init => {}
        Command::Apply => {}
    }

    Ok(())
}
