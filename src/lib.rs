#![recursion_limit = "1024"]

mod cli;
mod components;
mod skills;
mod utils;

use clap::{Error, Parser};
use components::about::About;
use components::projects::Projects;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::{Renderer, prelude::*};

use cli::*;
use components::conway::Conway;
use components::differ::Differ;
use components::history::History;
use std::ops::Add;
use std::rc::Rc;
use utils::ansi_html::convert;
use utils::history_store::HistoryStore;

use crate::utils::autocomplete::get_autocomplete;

#[derive(Debug, Clone)]
pub struct App {
    input: String,
    cmd: Option<Result<Cli, Rc<Error>>>,
    autocomplete: Vec<(String, Option<String>)>,
    autocomplete_open: bool,
    autocomplete_selection: Option<usize>,
    history: HistoryStore,
}

#[derive(Debug)]
pub enum AppMsg {
    InputSet(String),
    InputFocus(bool),
    AutocompleteHover(usize),
    AutocompleteShift(isize),
    AutocompleteSelect,
    FormSubmit,
    None,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            input: String::new(),
            cmd: None,
            autocomplete: get_autocomplete("".into()),
            autocomplete_open: false,
            autocomplete_selection: None,
            history: HistoryStore::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::InputSet(s) => {
                self.input = s;
                self.autocomplete = get_autocomplete(self.input.clone());
                self.autocomplete_selection = None;
                true
            }
            AppMsg::InputFocus(focused) => {
                self.autocomplete_open = focused;
                self.autocomplete_selection = None;
                true
            }
            AppMsg::AutocompleteHover(index) => {
                self.autocomplete_selection = Some(index);
                true
            }
            AppMsg::AutocompleteShift(shift) => {
                if !self.autocomplete.is_empty() {
                    let size = self.autocomplete.len() as isize;
                    let prev = self
                        .autocomplete_selection
                        .map(|s| s as isize)
                        .unwrap_or_else(|| if shift < 0 { size } else { -1 });

                    self.autocomplete_selection = Some(prev.add(shift).rem_euclid(size) as usize);

                    true
                } else {
                    false
                }
            }
            AppMsg::AutocompleteSelect => {
                if let Some(selected) = self.autocomplete_selection {
                    if let Some(autocomplete) = self.autocomplete.get(selected) {
                        let mut parts: Vec<String> = self
                            .input
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect();

                        if self.input.ends_with(' ') {
                            // User just finished a token, so append new one
                            parts.push(autocomplete.0.clone());
                        } else if let Some(last) = parts.last_mut() {
                            // Replace last token
                            *last = autocomplete.0.clone();
                        } else {
                            // No tokens yet
                            parts.push(autocomplete.0.clone());
                        }

                        self.input = parts.join(" ");
                        self.input.push(' ');
                        self.autocomplete = get_autocomplete(self.input.clone());
                        self.autocomplete_selection = None;
                        return true;
                    }
                }
                false
            }
            AppMsg::FormSubmit => {
                if let Some(selected) = self.autocomplete_selection {
                    if let Some(autocomplete) = self.autocomplete.get(selected) {
                        self.input = autocomplete.0.clone();
                        self.autocomplete = get_autocomplete(self.input.clone());
                        self.autocomplete_selection = None;
                    } else {
                        return false;
                    }
                } else {
                    // add to history
                    self.history.push(self.input.clone());

                    // looks messy, fix up later
                    let cmd =
                        Cli::try_parse_from(self.input.to_lowercase().split_ascii_whitespace())
                            .map_err(|e| Rc::new(e));
                    let is_ok = cmd.is_ok();
                    self.cmd = Some(cmd);
                    if is_ok {
                        self.execute_args();
                    }
                }

                true
            }
            AppMsg::None => false,
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
                    <p>{"(For the full list of commands and descriptions, type and enter 'help' into the console)"}</p>
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
        let show_autocomplete = self.autocomplete_open && !self.autocomplete.is_empty();
        let selected_autocomplete = show_autocomplete && self.autocomplete_selection.is_some();
        let oninput = link.callback(|e: InputEvent| {
            AppMsg::InputSet(
                e.target_dyn_into::<HtmlInputElement>()
                    .map(|elem| elem.value())
                    .unwrap_or_default(),
            )
        });
        let onfocus = link.callback(|_| AppMsg::InputFocus(true));
        let onblur = link.callback(|_| AppMsg::InputFocus(false));

        let onkeydown = link.callback(move |e: KeyboardEvent| match e.key().as_str() {
            "ArrowDown" => {
                e.prevent_default();
                AppMsg::AutocompleteShift(1)
            }
            "ArrowUp" => {
                e.prevent_default();
                AppMsg::AutocompleteShift(-1)
            }
            "Enter" => {
                if selected_autocomplete {
                    e.prevent_default();
                    AppMsg::AutocompleteSelect
                } else {
                    AppMsg::None
                }
            }
            _ => AppMsg::None,
        });

        let get_option_class = |index: usize| {
            (Some(index) == self.autocomplete_selection)
                .then(|| "option-selected")
                .unwrap_or("")
        };

        html! {
            <form
                id="console-form"
                onsubmit={link.callback(|e: SubmitEvent| {
                    e.prevent_default();
                    AppMsg::FormSubmit
                })}
            >
                <label for="console">{"Command"}</label>
                <div id="console-wrapper">
                    <input
                        id="console"
                        autofocus=true
                        placeholder="..."
                        autocomplete="off"
                        value={self.input.clone()}
                        oninput={oninput}
                        onfocus={onfocus}
                        onblur={onblur}
                        onkeydown={onkeydown}
                    />
                    {if show_autocomplete {
                        html! {
                            <ul class="autocomplete">
                                { for self.autocomplete.iter().enumerate().map(|(index, completion)| {
                                    let onmouseover = link.callback(move |_| AppMsg::AutocompleteHover(index));
                                    let onmousedown = link.callback(|e: MouseEvent| {
                                        e.prevent_default();
                                        AppMsg::AutocompleteSelect
                                    });
                                    html! {
                                        <li
                                            class={get_option_class(index)}
                                            onmouseover={onmouseover}
                                            onmousedown={onmousedown}
                                        >
                                            <div class="option-value">
                                                {&completion.0}
                                            </div>
                                            <div class="option-description">
                                                {completion.1.as_ref().map(|s| s.as_str()).unwrap_or("")}
                                            </div>
                                        </li>
                                    }
                                })}
                            </ul>
                        }
                    } else { html!() }}
                </div>
            </form>
        }
    }

    fn view_main(&self) -> Html {
        html! { <main role="main"> {
            match self.cmd.as_ref() {
                Some(Ok(args)) => html! {self.view_cmd(args)},
                // TODO: better error rendering
                Some(Err(err)) => convert(Rc::as_ref(err)),
                None => html! {},
            }
        } </main> }
    }

    fn view_cmd(&self, args: &Cli) -> Html {
        match &args.command {
            Command::About => html! { <About /> },
            Command::Contact => {
                html! { <>
                    <h2>{"You can send me emails here:"}</h2>
                    <a href="mailto:Benjaminjahall@gmail.com">{"Benjaminjahall@gmail.com"}</a>
                    <br />
                    <br />

                    <h2>{"My projects and code are documented here:"}</h2>
                    <a href="https://github.com/benhall-7">{"benhall-7"}</a>
                    <br />
                    <br />

                    <h2>{"Obligatory LinkedIn:"}</h2>
                    <a href="https://www.linkedin.com/in/benjaminjahall/">{"benjaminjahall"}</a>
                    <br />
                    <br />

                    <h2>{"Check out my observations on iNaturalist:"}</h2>
                    <a href="https://www.inaturalist.org/observations?user_id=benhall-7&verifiable=any">{"benhall-7"}</a>
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
            Command::Projects => html! { <Projects /> },
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

    /// logic pertaining to state-altering command side effects
    fn execute_args(&mut self) {
        match &self.cmd {
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
