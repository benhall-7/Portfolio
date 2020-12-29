use structopt::clap::AppSettings::NoBinaryName;
use structopt::StructOpt;
use yew::prelude::*;

use super::components::history::History;
use super::components::differ::Differ;

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
    sub: Option<HistorySubCommand>,
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
            Args::Contact => {
                html! { <>
                    <h2>{"Contact links:"}</h2>
                    <ul>
                        <li><a href="mailto:Benjaminjahall@gmail.com">{"Email"}</a></li>
                        <li><a href="https://github.com/BenHall-7">{"GitHub"}</a></li>
                        <li><a href="https://www.linkedin.com/in/benjaminjahall/">{"LinkedIn"}</a></li>
                    </ul>
                </> }
            }
            Args::Skills => {
                html! { <>
                    <h2>{"Web development related:"}</h2>
                    <ul>
                        <li>{"HTML5, CSS, Javascript, WebAssembly"}</li>
                        <li>{"Web frameworks: React, Express, LESS, Yew"}</li>
                        <li>{"Modern design paradigms; components, flexbox, APIs, AJAX"}</li>
                    </ul>
                    <h2>{"Software related"}</h2>
                    <ul>
                        <li>{"Emphasis on cross-platform technologies (where possible)"}</li>
                        <li>{"C# &amp; .NET: writing CLIs, Libraries, WinForms and WPF"}</li>
                        <li>{"Rust: applications, file IO libraries, TUI, Yew, proc-macros"}</li>
                        <li>{"Utilizing Lua as an application scripting language"}</li>
                    </ul>
                </> }
            }
            Args::Projects => {
                html! { <ul>
                    <li>
                        <h1><a href="https://github.com/BenHall-7/paracobNET/releases/latest">{"paracobNET"}</a></h1>
                        <p>{"A 4-tool suite of programs for reading and editing the proprietary '.prc' filetype in Super Smash Bros. Ultimate"}</p>
                        <ul>
                            <li>{"Written in C#, utilizing the .NET Core 3.0 framework"}</li>
                            <li>{"Features a class library, multi-threaded WPF editor, XML converter, and Lua scripting environment"}</li>
                            <li>{"Written independently, with no other authors"}</li>
                            <li>{"Able to run on any platform supported by .NET Core (e.g. Windows, Linux, MacOS, etc)"}</li>
                            <li>{"Automatic uploads of builds via Appveyor continuous integration"}</li>
                        </ul>
                        <a href="https://github.com/BenHall-7/paracobNET">{"Source"}</a>
                    </li>
                    <li>
                        <h1><a href="https://prod-homerun-fe.herokuapp.com/">{"TidyHive"}</a></h1>
                        <p>{"TidyHive is a full-fledged organizational tool for groups both large and small to manage their todos"}</p>
                        <ul>
                            <li>{"Frontend built with React and TailwindCSS"}</li>
                            <li>{"Supported by a NodeJS backend implemented with postgres"}</li>
                            <li>{"Follows a multi-stage release canvas cycle, with 6 team members"}</li>
                            <li>{"Tested across the stack with Jest, with coverage reports from CodeClimate and continuous integration"}</li>
                        </ul>
                        <a href="https://github.com/Lambda-School-Labs/homerun-fe">{"Frontend Source"}</a>{" / "}<a href="https://github.com/Lambda-School-Labs/homerun-be">{"Backend Source"}</a>
                    </li>
                    <li>
                        <h1><a href="https://potluck-planner-lambda.github.io/ui-ben/">{"Potluck Planner"}</a></h1>
                        <p>{"Potluck Planner provides users a way to manage potlucks; see guests attending, what food they want to bring, and more"}</p>
                        <ul>
                            <li>{"Home and About page of this project designed through pure semantic HTML and CSS"}</li>
                            <li>{"Uses javascript AJAX requests to load team member info directly from GitHub"}</li>
                            <li>{"Dynamic page views supporting everything from mobile to desktop widths"}</li>
                            <li>{"The bulk of the application handled by other team members is written with ReactJS"}</li>
                        </ul>
                        <a href="https://github.com/Potluck-Planner-Lambda">{"Source"}</a>
                    </li>
                </ul> }
            }
            Args::History(history) => {
                if let Some(sub) = &history.sub {
                    match sub {
                        HistorySubCommand::Clear => html! { "History cleared" },
                        HistorySubCommand::Index { num } => {
                            html! {
                                <History items=vec![String::from("test1"), String::from("test")] index=num />
                            }
                        }
                    }
                } else {
                    html! {
                        <History items=vec![String::from("test1"), String::from("test")] />
                    }
                }
            }
            Args::Diff => {
                html! {
                    <Differ />
                }
            }
            _ => html! {},
            // Args::Conway => {}
        }
    }
}
