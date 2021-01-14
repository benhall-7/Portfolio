#![recursion_limit = "1024"]

mod args;
mod components;
mod utils;

use structopt::clap;
use structopt::StructOpt;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::start_app;

use args::*;
use components::conway::Conway;
use components::differ::Differ;
use components::history::History;
use std::rc::Rc;
use utils::ansi_html::{convert, start_regex};
use utils::history_store::HistoryStore;

#[derive(Debug, Clone)]
pub struct App {
    link: ComponentLink<Self>,
    input: String,
    args_input: Option<Result<Args, Rc<clap::Error>>>,
    args: Option<Result<Args, Rc<clap::Error>>>,
    history: HistoryStore,
}

pub enum Msg {
    SetInput(String),
    SubmitInput,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            input: String::new(),
            args_input: None,
            args: None,
            history: HistoryStore::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetInput(s) => {
                self.input = s;
                self.args_input = Some(
                    <Args as StructOpt>::from_iter_safe(
                        self.input.to_lowercase().split_ascii_whitespace(),
                    )
                    .map_err(|e| Rc::new(e)),
                );
                true
            }
            Msg::SubmitInput => {
                // add to history
                self.history.push(self.input.clone());

                if let Some(some) = &self.args_input {
                    self.args = Some(some.clone());
                    if some.is_ok() {
                        self.execute_args();
                    }
                    return true;
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, _: bool) {
        // static value initialization
        start_regex();
    }

    fn view(&self) -> Html {
        html! {<>
            <header><h1>{"Portfolio Terminal"}</h1></header>
            <section id="content">
                {self.view_input()}
                {self.view_main()}

                <div class="help">
                    <div class="short-border"></div>
                    <p>{"(For a list of other commands, type and enter 'help' into the console)"}</p>
                </div>
            </section>
            <footer>
                <p>{"© Benjamin Hall 2020"}</p>
                <p><a href="https://github.com/BenHall-7/Portfolio/blob/master/LICENSE">{"License"}</a></p>
            </footer>
        </>}
    }
}

impl App {
    fn view_input(&self) -> Html {
        html! {
            <form
                id="console-wrapper"
                onsubmit=self.link.callback(|e: FocusEvent| {
                    e.prevent_default();
                    Msg::SubmitInput
                })
            >
                <label for="console">{"Command"}</label>
                <input
                    id="console"
                    autofocus=true
                    placeholder="..."
                    value=self.input
                    oninput=self.link.callback(|e: InputData| Msg::SetInput(e.value))
                />
            </form>
        }
    }

    fn view_main(&self) -> Html {
        html! { <main role="main"> {
            match self.args.as_ref() {
                Some(Ok(args)) => html! {self.view_args(args)},
                Some(Err(err)) => convert(&err.message),
                None => html! {},
            }
        } </main> }
    }

    fn view_args(&self, args: &Args) -> Html {
        match args {
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
                        HistorySubCommand::Clear => {
                            html! { <p>{"History cleared"}</p> }
                        }
                        HistorySubCommand::Index { num } => {
                            html! {
                                <History items=self.history.history() index=num />
                            }
                        }
                    }
                } else {
                    html! {
                        <History items=self.history.history() />
                    }
                }
            }
            Args::Diff => html! { <Differ /> },
            Args::Conway(ConwayArg { about }) => {
                if !about {
                    html! { <Conway /> }
                } else {
                    html! { <div class="conway-about">
                        <h2>{"Conway's Game of Life"}</h2>
                        <p>
                            {"Devised by the late British mathematician "}
                            <a href="https://en.wikipedia.org/wiki/John_Horton_Conway">{"John Horton Conway"}</a>
                            {", the Game of Life is a simple 2D simulation of cells treated as organisms. These cells follow a simple set of mathematical rules meant to emulate basic rules of populations, determining whether they are considered \"alive\" or \"dead\". The rules are as follows:"}
                        </p>
                        <ul>
                            <li>
                                {"If a living cell has 2 or 3 neighbors ('neighbors' includes diagonals), it remains living. Otherwise, it dies"}
                            </li>
                            <li>
                                {"If a dead cell (or rather, empty cell) has 3 neighbors exactly, it becomes alive. Otherwise it remains dead."}
                            </li>
                        </ul>
                        <p>
                            {"In this way, a strategically placed set of cells may remain in existence forever, in a loop. Many configurations of cells die off completely; many explode first and can lead into either outcome."}
                            <br />
                            {"The Game of Life is only one such example of a broader topic of interest, known as "}
                            <a href="https://en.wikipedia.org/wiki/Cellular_automaton">{"Cellular Automata."}</a>
                            {" Cellular Automata (CA for short) describes systems of these cells in grids (which may not necessarily be 2D) and how they behave under certain other rulesets. These are useful in some branches of science for examining behavior of real systems such as those studied under Biology or Chemistry."}
                            <br />
                            {"Another interesting property of the game is that it exhibits Turing Completeness, a quality of a system by which you can perform arbitrary calculations. This is made possible through cell patterns such as the Gosper Gun, which if set up correctly may emulate a NAND gate instruction, an essential logical operator seen in hardware necessary to run machine instructions. Note that we also give ourselves infinite time and grid space when discussing the game's Turing Completeness. This means also that it is possible to replicate the Game of Life "}
                            <a href="https://www.youtube.com/watch?v=xP5-iIeKXE8">{"inside the Game of Life itself"}</a>
                            {" with huge blocks of cells that work to calculate the results of their neighbor blocks."}
                        </p>
                        <h2>{"Implementation"}</h2>
                        <p>
                            {"The implementation on this website was designed initially with a Rust backend and Javascript frontend. Since then, it has been rewritten entirely in Rust using the Yew framework. This is made possible using a recent browser standard called "}
                            <a href="https://webassembly.org/">{"WebAssembly (WASM)"}</a>
                            {", which provides a compilation target for languages designed for native performance apps such as C, C++, and of course Rust."}
                            <br />
                        </p>
                    </div> }
                }
            }
        }
    }

    fn execute_args(&mut self) {
        match &self.args {
            Some(Ok(Args::History(HistoryArg {
                sub: Some(HistorySubCommand::Clear),
            }))) => {
                self.history.clear();
            }
            _ => {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    start_app::<App>();
}
