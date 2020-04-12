use crate::run_tf_cmd_in_dir;

pub fn run() -> anyhow::Result<()> {
    run_tf_cmd_in_dir(".", "init")
}
