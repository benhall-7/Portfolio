use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ProjectInfo {
    pub title: &'static str,
    pub deployment: Option<&'static str>, // hyperlinks on the title
    pub summary: &'static str,
    pub bullets: Vec<&'static str>,
    pub sources: Vec<(&'static str, &'static str)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Project;

impl Component for Project {
    type Message = ();

    type Properties = ProjectInfo;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {<div class="comp-project">
            <h2>{ match props.deployment {
                None => html! {props.title},
                Some(dep) => html! {<a href={dep}>{props.title}</a>},
            }}</h2>
            <p>{props.summary}</p>
            {
                if !props.bullets.is_empty() {
                    html! {<ul>{ for props.bullets.iter().map(|b| html! {<li>{b}</li>}) }</ul>}
                } else {
                    html! {}
                }
            }
            {
                for props.sources
                    .iter()
                    .enumerate()
                    .map(|(i, (t, url))| html! { <>
                        {if i > 0 { " / " } else { "" }}
                        <a href={*url}>{t}</a>
                    </>})
            }
        </div>}
    }
}
