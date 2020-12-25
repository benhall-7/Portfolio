#![recursion_limit="256"]

mod components;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::start_app;

use components::input::Input;

pub struct App;

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header><h1>{"Portfolio Terminal"}</h1></header>
                <section>
                    <Input/>
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
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    start_app::<App>();
}
