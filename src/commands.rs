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
}
