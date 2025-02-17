use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct About;

const DOG: &'static str = include_str!("dog.txt");

impl Component for About {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <div class="comp-about">
            <div class="about-header">
                <div class="img-container">
                    <div class="img-border" />
                    <img class="icon"
                        src="img/me.jpg"
                        alt="Photo of my face, showing blond hair, blue eyes, a red shirt, and a dark background"
                    />
                </div>
                <pre>{DOG}</pre>
            </div>

            <hr />

            <p>
                {"Hi! My name is "}<span class="emph">{"Benjamin Hall"}</span>
                {", I'm a software engineer and full-stack web developer. Although born and raised in Northern CA, I'm currently living in Harrisburg, PA."}
                <br/><br/>
                {"Growing up, I expected to become a mathematician. The subject was deeply meaningful to me. But when life took some unexpected turns, I reoriented \
                  my career path and became a computer programmer."}
                <br/><br/>
                {"Professionally I write web apps, and am experienced with various frameworks and systems ("}
                <span class="emph">{"React, Ruby on Rails, TypeScript, etc"}</span>{")."}
                <br/><br/>
                {"Outside of my professional experience, I prefer to write in Rust. I'm frequently developing a combination of systems or web programming projects. \
                  For example, I've written more than half a dozen apps to help users mod games (in particular: Super Smash Bros. Ultimate)."}
                <br/><br/>
                {"I have an adorable, smart labradoodle named Lady 🐶"}
                <br/><br/>
                {"I have too many hobbies! If I had to rank them, I would do so as follows:"}
            </p>
            <ul>
                <li>{"Piano / Classical music (since 2009)"}</li>
                <li>{"Cycling (road & gravel)"}</li>
                <li>{"Botany (1600+ observations on iNat)"}</li>
                <li>{"Photography (I've sold some prints!)"}</li>
                <li>{"At some point I will be adding woodworking to this list..."}</li>
            </ul>
        </div> }
    }
}
