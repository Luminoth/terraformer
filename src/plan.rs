use crate::terraform;

pub fn run() -> anyhow::Result<()> {
    terraform::command_in_dir(".", "plan", true)
}
