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
                {", I'm a software engineer and full-stack web developer. I was born and raised in Northern CA, but I currently live in \
                  Harrisburg, PA."}
                <br/><br/>
                {"Growing up, I wanted to be a mathematician. When I entered the professional context, I applied my knowledge of math and \
                  computer programming and became a "}<span class="emph">{"Full-Stack Web and Software Engineer."}</span>
                <br/><br/>
                {"Professionally I write web apps, and am experienced with various frameworks and systems ("}
                <span class="emph">{"React, Ruby on Rails, TypeScript, etc"}</span>{")."}
                <br/><br/>
                {"Although I can write in a number of frameworks and languages, I prefer to write in Rust. I'm frequently developing a \
                  combination of systems or web programming projects. For example, I've written more than half a dozen apps to help users \
                  mod games (in particular: Super Smash Bros. Ultimate)."}
                <br/><br/>
                {"I have an adorable, smart labradoodle named Lady 🐶"}
                <br/><br/>
                {"I have a fair number of hobbies! If I had to rank them, I would do so as follows:"}
            </p>
            <ul>
                <li>{"Piano / Classical music (15+ years)"}</li>
                <li>{"Cycling (road & gravel)"}</li>
                <li>{"Botany (2000+ observations on iNat)"}</li>
                <li>{"Photography (I've sold some prints!)"}</li>
            </ul>
        </div> }
    }
}
