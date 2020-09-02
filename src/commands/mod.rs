pub mod apply;
pub mod destroy;
pub mod init;
pub mod output;
pub mod plan;
pub mod state;
pub mod validate;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "terraformer")]
pub enum Commands {
    Init,
    InitAll,
    Plan,
    PlanAll,
    Apply,
    ApplyAll,
    Destroy,
    DestroyAll,
    Output,
    OutputAll,
    State { args: Vec<String> },
    StateAll { args: Vec<String> },
    Validate { args: Vec<String> },
    ValidateAll { args: Vec<String> },
}
