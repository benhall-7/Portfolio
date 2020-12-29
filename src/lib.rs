#![recursion_limit = "512"]

mod args;
mod components;
mod utils;

use structopt::clap;
use structopt::StructOpt;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::start_app;

use args::*;
use components::differ::Differ;
use components::history::History;
use std::rc::Rc;
use utils::history_store::HistoryStore;

#[derive(Debug, Clone)]
pub struct App {
    link: ComponentLink<Self>,
    input: String,
    args_input: Option<Result<Args, Rc<clap::Error>>>,
    args_error: Option<Rc<clap::Error>>,
    args: Option<Args>,
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
            args_error: None,
            args: None,
            history: HistoryStore::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetInput(s) => {
                self.input = s;
                self.args_input = Some(
                    <Args as StructOpt>::from_iter_safe(self.input.split_ascii_whitespace())
                        .map_err(|e| Rc::new(e)),
                );
                true
            }
            Msg::SubmitInput => {
                // add to history
                self.history.push(self.input.clone());

                if let Some(some) = &self.args_input {
                    match some {
                        Ok(args) => {
                            self.args = Some(args.clone());
                            self.args_error = None;
                            self.execute_args();
                        }
                        Err(e) => {
                            self.args = None;
                            self.args_error = Some(e.clone())
                        }
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

    fn view(&self) -> Html {
        html! {<>
            <header><h1>{"Portfolio Terminal"}</h1></header>
            <section>
                {self.view_input()}
                {self.view_main()}

                <div class="help">
                    <div class="short-border"></div>
                    <p>{"(For a list of commands, press, or type and enter the \"<span class=\"cmd\" tabindex=\"0\" onclick=\"submitCmd('help')\">help</span>\" command)"}</p>
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
        html! {
            <main id="content" role="main">{
                self.args
                    .as_ref()
                    .map(|a| self.view_args(a))
                    .unwrap_or(html! {})
            }{
                self.args_error
                    .as_ref()
                    .map(|e| html! { <>
                        {format!("MESSAGE: {}", e.message)}
                        <br/>
                        {format!("KIND: {:?}", e.kind)}
                        <br/>
                        {format!("INFO: {:?}", e.info)}
                    </> })
                    .unwrap_or(html! {})
            }</main>
        }
    }

    pub fn view_args(&self, args: &Args) -> Html {
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
                        },
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
            Args::Diff => {
                html! {
                    <Differ />
                }
            }
            Args::Conway(ConwayArg { about: _ }) => {
                html! {

                }
            },
        }
    }

    pub fn execute_args(&mut self) {
        if self.args.is_some() {
            match self.args.as_ref().unwrap() {
                Args::History(HistoryArg { sub: Some(HistorySubCommand::Clear) }) => {
                    self.history.clear();
                }
                _ => {}
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    start_app::<App>();
}
