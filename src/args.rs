use structopt::clap::AppSettings::NoBinaryName;
use structopt::StructOpt;
use yew::prelude::*;

#[derive(Debug, Clone, StructOpt)]
#[structopt(settings(&[NoBinaryName]))]
pub enum Args {
    About,
    Contact,
    Skills,
    Projects,
    #[structopt(about = "History of inputs to this terminal")]
    History(HistoryArg),
    Diff,
    #[structopt(about = "Conway's game of life")]
    Conway(ConwayArg),
}

#[derive(Debug, Clone, StructOpt)]
pub struct HistoryArg {
    #[structopt(subcommand)]
    sub: Option<HistorySubCommand>,
}

#[derive(Debug, Clone, StructOpt)]
pub enum HistorySubCommand {
    #[structopt(about = "Clears all of this terminal's history")]
    Clear,
    #[structopt(about = "Gets the history of this terminal at a specific index.")]
    Index { num: i32 },
}

#[derive(Debug, Clone, StructOpt)]
pub struct ConwayArg {
    #[structopt(long, short)]
    about: bool,
}

impl Args {
    pub fn view_content(&self) -> Html {
        match self {
            Args::About => {
                html! { <>
                    <div><img class="icon" src="img/self.jpg" alt="Photo of myself in front of pink blossoms"/></div>
                    <p>
                        {"Hi! My name is "}<span class="emph">{"Benjamin Hall"}</span>
                        {", I'm a software engineer and full-stack web developer. Although born and raised in Northern California, I'm currently living near Pittsburgh, PA."}<br/><br/>
                        {"As a lifelong advocate (and prior student) for Mathematics, I seek to find creative and generic solution to problems that not only improve consistency in user experience, but to ease future development too."}
                    </p>
                    // guarantees the containing div at least matches the height of the floating image
                    <div style="clear:both;"></div>
                </> }
            }
            _ => html! { "" },
            // Args::Contact => {}
            // Args::Skills => {}
            // Args::Projects => {}
            // Args::History => {}
            // Args::Diff => {}
            // Args::Conway => {}
        }
    }
}
