use clap::{ColorChoice, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(no_binary_name = true, color = ColorChoice::Always)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
// #[clap(settings(&[NoBinaryName, ColorAlways, DisableVersion]))]
pub enum Command {
    #[command(about = "Introduction of myself")]
    About,
    #[command(about = "Links to use to get in contact with me")]
    Contact,
    #[command(about = "Curated subjects or frameworks I have experience with")]
    Skills,
    #[command(about = "Curated list of projects that I've spent a lot of time on")]
    Projects,
    #[command(about = "History of inputs to this terminal")]
    History(HistoryArg),
    #[command(about = "Custom diff implementation on strings")]
    Diff,
    #[command(about = "Conway's Game of Life implementation")]
    Conway(ConwayArg),
}

#[derive(Debug, Clone, Parser)]
pub struct HistoryArg {
    #[command(subcommand)]
    pub command: Option<HistorySubcommand>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum HistorySubcommand {
    #[command(about = "Clears all of this terminal's history")]
    Clear,
    #[command(about = "Gets the history of this terminal at a specific index.")]
    Index { num: usize },
}

#[derive(Debug, Clone, Parser)]
pub struct ConwayArg {
    #[arg(long, short, help = "Background explanation for the Game of Life")]
    pub about: bool,
}
