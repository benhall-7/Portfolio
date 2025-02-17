use yew::prelude::*;

pub use project::{Project, ProjectInfo};

mod project;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageProps {
    pub src: &'static str,
    pub alt: &'static str,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ProjectGroupProps {
    pub title: &'static str,
    pub projects: Vec<ProjectInfo>,
    pub content_side: Side,
    pub content: Vec<ImageProps>,
}

#[derive(Debug, Clone)]
pub struct ProjectGroup;

impl Component for ProjectGroup {
    type Message = ();

    type Properties = ProjectGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let float_class = match props.content_side {
            Side::Left => "float-left",
            Side::Right => "float-right",
        };
        let props_content_class = format!("project-group-content {}", float_class);

        html! {<div class="comp-project-group">
            <h1>{props.title}</h1>
            {if !props.content.is_empty() {
                html! {<div class={props_content_class}>
                    {for props.content.iter().map(|image_props| html!{
                        <img src={image_props.src} alt={image_props.alt} />
                    })}
                </div>}
            } else {
                html!{}
            }}
            <div>
                {for props.projects.iter().map(|project| html! {
                    <Project ..project.clone()/>
                })}
            </div>
        </div>}
    }
}
