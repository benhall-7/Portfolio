#![recursion_limit = "256"]

mod args;
mod components;

use structopt::clap;
use structopt::StructOpt;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::start_app;

use args::Args;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct App {
    link: ComponentLink<Self>,
    input: String,
    args_input: Option<Result<Args, Rc<clap::Error>>>,
    args: Option<Args>,
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
                if let Some(some) = &self.args_input {
                    if let Ok(args) = some {
                        self.args = Some(args.clone());
                        return true;
                    }
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
                {if self.args_input.as_ref().is_some() && self.args_input.as_ref().unwrap().is_err() {
                    let some = self.args_input.as_ref().unwrap();
                    // get out of the RC, then convert the &Result<T, E> to Result<&T, &E>
                    let err = (*some).as_ref().unwrap_err();
                    format!("{}", err)
                } else {
                    String::from("")
                }}

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
                    oninput=self.link.callback(|e: InputData| {
                        Msg::SetInput(e.value)
                    })
                />
            </form>
        }
    }

    fn view_main(&self) -> Html {
        html! {
            <main id="content" role="main">
                {self.args.as_ref().map(|a| a.view_content()).unwrap_or(self.view_default_main())}
            </main>
        }
    }

    fn view_default_main(&self) -> Html {
        html! {
            "Type in a command to get started... (TODO)"
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    start_app::<App>();
}
