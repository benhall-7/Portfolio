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
    History,
    Diff,
    Conway,
}

impl Args {
    pub fn view_content(&self) -> Html {
        match self {
            Args::About => {
                html! { "This is the about page" }
            }
            _ => html! { "This is a test" }
            // Args::Contact => {

            // }
            // Args::Skills => {}
            // Args::Projects => {}
            // Args::History => {}
            // Args::Diff => {}
            // Args::Conway => {}
        }
    }
}