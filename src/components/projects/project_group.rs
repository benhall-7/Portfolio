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

        let group_data_class = match props.content_side {
            Side::Left => "project-group-data row",
            Side::Right => "project-group-data row-reverse",
        };

        html! {<div class="comp-project-group">
            <h1>{props.title}</h1>
            <div class={group_data_class}>
                {if !props.content.is_empty() {
                    html! {<div class="project-group-content">
                        {for props.content.iter().map(|image_props| html!{
                            <img src={image_props.src} alt={image_props.alt} />
                        })}
                    </div>}
                } else {
                    html!{}
                }}
                <div class="project-group-details">
                    {for props.projects.iter().map(|project| html! {
                        <Project ..project.clone()/>
                    })}
                </div>
            </div>
        </div>}
    }
}
