use crate::terraform;

pub async fn run(args: impl Into<Vec<String>>, all: bool) -> anyhow::Result<()> {
    terraform::command_in_dir_with_args(".", "apply", args, all).await
}
