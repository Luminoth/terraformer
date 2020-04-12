use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "terraformer")]
pub enum Commands {
    Init,
    Plan,
    Apply,
}
