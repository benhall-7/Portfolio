use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Args {
    About,
    Contact,
    Skills,
    Projects,
    History,
    Diff,
    Conway,
}