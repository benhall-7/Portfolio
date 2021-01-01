use diff::VecDiffType;
use yew::prelude::*;

use crate::utils::diff::get_diff;

#[derive(Debug, Clone)]
pub struct Differ {
    link: ComponentLink<Self>,
    a: String,
    b: String,
}

#[derive(Debug, Clone)]
pub enum DifferMessage {
    SetA(String),
    SetB(String),
}

impl Component for Differ {
    type Message = DifferMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Differ {
            link,
            a: String::new(),
            b: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DifferMessage::SetA(a) => self.a = a,
            DifferMessage::SetB(b) => self.b = b,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        *self = Self::create(props, self.link.clone());
        true
    }

    fn view(&self) -> Html {
        html! { <>
            <h2>{"Diff inputs:"}</h2>
            <form id="diff-form">
                <div>
                    <label for="diff-a">{"A"}</label>
                    <input
                        type="text"
                        id="diff-a"
                        value={&self.a}
                        oninput=self.link.callback(|e: InputData| DifferMessage::SetA(e.value))
                    />
                </div>
                <div>
                    <label for="diff-b">{"B"}</label>
                    <input
                        type="text"
                        id="diff-b"
                        value={&self.b}
                        oninput=self.link.callback(|e: InputData| DifferMessage::SetB(e.value))
                    />
                </div>
            </form>
            <h2>{"Diff output:"}</h2>
            <p id="diff-output">{self.view_diff()}</p>
        </> }
    }
}

impl Differ {
    fn view_diff(&self) -> Html {
        let diff = get_diff(self.a.clone(), self.b.clone());

        let mut output: Vec<Html> = Vec::new();
        let mut position = 0_usize;
        
        for d in &diff.0 {
            match d {
                VecDiffType::Removed { index, len } => {
                    if *index > position {
                        output.push(html! { &self.a[position..*index] });
                        position = *index;
                    }
                    output.push(html! {
                        <span class="diff-remove">
                            { &self.a[position..position + len] }
                        </span>
                    });
                    position += len;
                }
                VecDiffType::Inserted { index, changes } => {
                    if *index > position {
                        output.push(html! { &self.a[position..*index] });
                        position = *index;
                    }
                    output.push(html! {
                        <span class="diff-insert">
                            { changes.iter().map(|o| o.unwrap()).collect::<String>() }
                        </span>
                    });
                }
                VecDiffType::Altered { index, changes } => {
                    if *index > position {
                        output.push(html! { &self.a[position..*index] });
                        position = *index;
                    }
                    output.push(html! { <>
                        <span class="diff-alter-a">
                            { &self.a[position..position + changes.len()] }
                        </span>
                        <span class="diff-alter-b">
                            { changes.iter().map(|o| o.unwrap()).collect::<String>() }
                        </span>
                    </> });
                    position += changes.len();
                }
            }
        }
        if position < self.a.len() {
            output.push(html! { &self.a[position..] });
        }
        html! { <>{output}</> }
    }
}
