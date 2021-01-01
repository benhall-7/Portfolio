use yew::prelude::*;
// use yew::services::StorageService;

#[derive(Debug, Clone)]
pub enum History {
    All(Vec<String>),
    One(String),
    OOB,
}

#[derive(Debug, Clone, Properties)]
pub struct HistoryProps {
    pub items: Vec<String>,
    #[prop_or(None)]
    pub index: Option<usize>,
}

impl Component for History {
    type Message = ();
    type Properties = HistoryProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        if let Some(i) = props.index {
            if i >= props.items.len() {
                History::OOB
            } else {
                History::One(props.items[i].clone())
            }
        } else {
            History::All(props.items)
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        *self = if let Some(i) = props.index {
            if i >= props.items.len() {
                History::OOB
            } else {
                History::One(props.items[i].clone())
            }
        } else {
            History::All(props.items)
        };
        true
    }

    fn view(&self) -> Html {
        match self {
            History::All(h) => {
                html! { <>
                    <h2>{"Command History"}</h2>
                    <p>{format!("count: {}", h.len())}</p>
                    <ul>{
                        h.iter()
                            .enumerate()
                            .map(|(i, v)| History::view_item(Some(i), v))
                            .collect::<Html>()
                    }</ul>
                </> }
            }
            History::One(h) => {
                html! { <>
                    <h2>{"Command History"}</h2>
                    <ul>{History::view_item(None, &h)}</ul>
                </> }
            }
            History::OOB => {
                html! { <p>{"Index was out of bounds"}</p> }
            }
        }
    }
}

impl History {
    fn view_item(index: Option<usize>, item: &str) -> Html {
        if let Some(i) = index {
            html! { <li>
                {format!("{} - ", i)}
                {item}
            </li> }
        } else {
            html!(item)
        }
    }
}
