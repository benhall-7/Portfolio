use yew::prelude::*;

// saves me some typing
type S = &'static str;

pub struct Project {
    pub title: S,
    pub deployment: Option<S>, // hyperlinks on the title
    pub summary: S,
    pub bullets: &'static [S],
    pub sources: &'static [(S, S)],
}

pub const PROJECTS: &'static [Project] = &[
    Project {
        title: "paracobNET",
        deployment: Some("https://github.com/BenHall-7/paracobNET/releases/tag/v3.0"),
        summary: "Open source game modding tools for SSBU parameters",
        bullets: &[
            "Alter character stats, playlists, and much more",
            "Code library to interact with '.prc' filetype (C#)",
            "User interface for easy editing capability (WPF, XML)",
        ],
        sources: &[("Source", "https://github.com/BenHall-7/paracobNET/")],
    },
    Project {
        title: "TidyHive",
        deployment: Some("https://tidyhive-fe.herokuapp.com/"),
        summary: "Task management app for groups",
        bullets: &[
            "Interact with other users and todo's (React)",
            "Secure, feature-rich API (NodeJS, PostgreSQL)",
            "Small 6 member development team",
        ],
        sources: &[
            (
                "Frontend Source",
                "https://github.com/Lambda-School-Labs/homerun-fe",
            ),
            (
                "Backend Source",
                "https://github.com/Lambda-School-Labs/homerun-be",
            ),
        ],
    },
    Project {
        title: "This Portfolio!",
        deployment: None,
        summary: "A super cool, dynamic, terminal-powered single page app of awesomeness",
        bullets: &["Powered by Rust, and cool libraries like StructOpt (Rust, Yew, WASM)"],
        sources: &[("Source", "https://github.com/BenHall-7/Portfolio")],
    },
    Project {
        title: "Potluck Planner",
        deployment: Some("https://potluck-planner-lambda.github.io/ui-ben/"),
        summary: "Potluck coordination app; don't eat just desserts!",
        bullets: &["Designed landing pages (HTML, CSS, JavaScript)"],
        sources: &[("Source", "https://github.com/Potluck-Planner-Lambda/ui-ben")],
    },
];

pub const CURRENT_PROJECTS: &'static [Project] = &[
    Project {
        title: "diff-struct",
        deployment: None,
        summary: "Diffing functionality for generic structs (Rust)",
        bullets: &[],
        sources: &[("Source", "https://github.com/BenHall-7/diff-struct")],
    },
    Project {
        title: "musicli",
        deployment: None,
        summary: "A terminal-based MIDI file editor (Rust, TUI)",
        bullets: &[],
        sources: &[("Source", "https://github.com/BenHall-7/musicli")],
    },
];

pub fn render_projects(projects: &[Project]) -> Html {
    html! {
        for projects.iter().map(|project| {
            html! {<li>
                <h1>{ match project.deployment {
                    None => html! {project.title},
                    Some(dep) => html! {<a href=dep>{project.title}</a>},
                }}</h1>
                <p>{project.summary}</p>
                {
                    if !project.bullets.is_empty() {
                        html! {<ul>{ for project.bullets.iter().map(|b| html! {<li>{b}</li>}) }</ul>}
                    } else {
                        html! {}
                    }
                }
                {
                    if !project.sources.is_empty() {
                        project.sources
                            .iter()
                            .enumerate()
                            .map(|(i, (t, url))| html! { <>
                                {if i > 0 { " / " } else { "" }}
                                <a href=*url>{t}</a>
                            </>})
                            .collect::<Html>()
                    } else {
                        html! {}
                    }
                }
            </li>}
        })
    }
}
