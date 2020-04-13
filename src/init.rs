use crate::terraform;

pub fn run(all: bool) -> anyhow::Result<()> {
    if all {
        terraform::command_in_dir(".", "init", true)
    } else {
        terraform::command_in_dir(".", "init", false)
    }
}
