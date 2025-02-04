#![recursion_limit = "1024"]

mod args;
mod components;
mod projects;
mod skills;
mod utils;

use clap::{Error, Parser};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::{prelude::*, Renderer};

use args::*;
use components::conway::Conway;
use components::differ::Differ;
use components::history::History;
use std::rc::Rc;
use utils::ansi_html::convert;
use utils::history_store::HistoryStore;

#[derive(Debug, Clone)]
pub struct App {
    input: String,
    args_input: Option<Result<Cli, Rc<Error>>>,
    args: Option<Result<Cli, Rc<Error>>>,
    history: HistoryStore,
}

pub enum Msg {
    SetInput(String),
    SubmitInput,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            input: String::new(),
            args_input: None,
            args: None,
            history: HistoryStore::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInput(s) => {
                self.input = s;
                self.args_input = Some(
                    <Cli as Parser>::try_parse_from(
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

    fn view(&self, context: &Context<Self>) -> Html {
        html! {<>
            <header><h1>{"Portfolio Terminal"}</h1></header>
            <section id="content">
                {self.view_input(context.link())}
                {self.view_main()}

                <div class="help">
                    <div class="short-border"></div>
                    <p>{"(For a list of other commands, type and enter 'help' into the console)"}</p>
                </div>
            </section>
            <footer>
                <p>{"© Benjamin Hall 2025"}</p>
                <p><a href="https://github.com/benhall-7/Portfolio/blob/master/LICENSE">{"License"}</a></p>
            </footer>
        </>}
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let oninput = link.callback(|e: InputEvent| {
            Msg::SetInput(
                e.target_dyn_into::<HtmlInputElement>()
                    .map(|elem| elem.value())
                    .unwrap_or_default(),
            )
        });
        html! {
            <form
                id="console-wrapper"
                onsubmit={link.callback(|e: SubmitEvent| {
                    e.prevent_default();
                    Msg::SubmitInput
                })}
            >
                <label for="console">{"Command"}</label>
                <input
                    id="console"
                    autofocus=true
                    placeholder="...test"
                    value={self.input.clone()}
                    oninput={oninput}
                />
            </form>
        }
    }

    fn view_main(&self) -> Html {
        html! { <main role="main"> {
            match self.args.as_ref() {
                Some(Ok(args)) => html! {self.view_args(args)},
                Some(Err(err)) => convert(Rc::as_ref(err)),
                None => html! {},
            }
        } </main> }
    }

    fn view_args(&self, args: &Cli) -> Html {
        match &args.command {
            Command::About => {
                html! { <>
                    <div><img class="icon" src="img/me.jpg" alt="Photo of my face with a lighthouse in the distance"/></div>
                    <p>
                        {"Hi! My name is "}<span class="emph">{"Benjamin Hall"}</span>
                        {", I'm a software engineer and full-stack web developer. Although born and raised in Northern CA, I'm currently living in Harrisburg, PA."}<br/><br/>
                        {"As a lifelong advocate (and prior student) for Mathematics, I seek to find creative and generic solution to problems that not only improve consistency in user experience, but to ease future development too."}
                    </p>
                    // guarantees the containing div at least matches the height of the floating image
                    <div style="clear:both;"></div>
                </> }
            }
            Command::Contact => {
                html! { <>
                    <h2>{"Contact links:"}</h2>
                    <ul>
                        <li><a href="mailto:Benjaminjahall@gmail.com">{"Email"}</a></li>
                        <li><a href="https://github.com/benhall-7">{"GitHub"}</a></li>
                        <li><a href="https://www.linkedin.com/in/benjaminjahall/">{"LinkedIn"}</a></li>
                    </ul>
                </> }
            }
            Command::Skills => {
                html! {
                    for skills::SKILLS.iter().map(|skill| {
                        html! { <>
                            <h2>{skill.category}</h2>
                            <ul>{ for skill.bullets.iter().map(|b| html! {<li>{b}</li>}) }</ul>
                        </> }
                    })
                }
            }
            Command::Projects => {
                html! {<div>
                    {projects::render_projects(projects::PROJECTS)}
                    <h1 class="cent">{"Current projects:"}</h1>
                    {projects::render_projects(projects::CURRENT_PROJECTS)}
                    <h1 class="cent">{"Other projects:"}</h1>
                    {projects::render_projects(projects::OTHER_PROJECTS)}
                </div>}
            }
            Command::History(history) => {
                if let Some(sub) = &history.command {
                    match sub {
                        HistorySubcommand::Clear => {
                            html! { <p>{"History cleared"}</p> }
                        }
                        HistorySubcommand::Index { num } => {
                            html! {
                                <History items={self.history.history()} index={num} />
                            }
                        }
                    }
                } else {
                    html! {
                        <History items={self.history.history()} />
                    }
                }
            }
            Command::Diff => html! { <Differ /> },
            Command::Conway(ConwayArg { about }) => {
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
            Some(Ok(Cli {
                command:
                    Command::History(HistoryArg {
                        command: Some(HistorySubcommand::Clear),
                    }),
            })) => {
                self.history.clear();
            }
            _ => {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Renderer::<App>::new().render();
}
