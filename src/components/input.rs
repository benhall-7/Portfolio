use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct Input {
    link: ComponentLink<Self>,
    value: String,
}

#[derive(Debug, Clone)]
pub enum InputMessage {
    SetValue(String),
}

#[derive(Debug, Clone, Properties)]
pub struct InputProps {}

impl Component for Input {
    type Message = InputMessage;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Input {
            link,
            value: String::new(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            InputMessage::SetValue(s) => {
                self.value = s;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <form id="console-wrapper">
                <label for="console">{"Command"}</label>
                <input
                    id="console"
                    autofocus=true
                    placeholder="..."
                    value=self.value
                    oninput=self.link.callback(|e: InputData| {
                        InputMessage::SetValue(e.value)
                    })
                />
            </form>
        }
    }
}