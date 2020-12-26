#![recursion_limit="256"]

mod components;
mod args;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::start_app;

#[derive(Debug, Clone)]
pub struct App {
    link: ComponentLink<Self>,
    input: String,
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetInput(s) => {
                self.input = s;
                true
            }
            Msg::SubmitInput => {
                false // temp
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {<>
            <header><h1>{"Portfolio Terminal"}</h1></header>
            <section>
                {self.view_input()}
                <main id="content" role="main"></main>

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
}

#[wasm_bindgen(start)]
pub fn main() {
    start_app::<App>();
}
