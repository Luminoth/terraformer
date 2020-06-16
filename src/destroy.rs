use crate::terraform;

pub async fn run(all: bool) -> anyhow::Result<()> {
    terraform::command_in_dir(".", "destroy", all).await
}
