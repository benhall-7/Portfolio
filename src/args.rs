use structopt::clap::AppSettings::NoBinaryName;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(settings(&[NoBinaryName]))]
pub enum Args {
    #[structopt(about = "Introduction of myself")]
    About,
    #[structopt(about = "Links to use to get in contact with me")]
    Contact,
    #[structopt(about = "Curated subjects or frameworks I have experience with")]
    Skills,
    #[structopt(about = "Curated list of projects that I've spent a lot of time on")]
    Projects,
    #[structopt(about = "History of inputs to this terminal")]
    History(HistoryArg),
    #[structopt(about = "Custom diff implementation on strings")]
    Diff,
    #[structopt(about = "Conway's Game of Life implementation")]
    Conway(ConwayArg),
}

#[derive(Debug, Clone, StructOpt)]
pub struct HistoryArg {
    #[structopt(subcommand)]
    pub sub: Option<HistorySubCommand>,
}

#[derive(Debug, Clone, StructOpt)]
pub enum HistorySubCommand {
    #[structopt(about = "Clears all of this terminal's history")]
    Clear,
    #[structopt(about = "Gets the history of this terminal at a specific index.")]
    Index { num: usize },
}

#[derive(Debug, Clone, StructOpt)]
pub struct ConwayArg {
    #[structopt(long, short, about = "Background explanation for the Game of Life")]
    pub about: bool,
}
