use std::{cell::LazyCell, ops::Deref};

use yew::prelude::*;

#[derive(Debug, Clone)]
struct Project(ProjectProps);

// saves me some typing
type S = &'static str;

#[derive(Debug, Clone, PartialEq, Properties)]
struct ProjectProps {
    pub title: S,
    pub deployment: Option<S>, // hyperlinks on the title
    pub iframe: Option<Html>,
    pub summary: S,
    pub bullets: Vec<S>,
    pub sources: Vec<(S, S)>,
}

// in-progress: emulation, (TODO) iNat quiz
// prc-related projects
// web (portfolio, cube-ts)
// misc (diff-struct, musicli, tidyhive)

const CURRENT_PROJECTS: LazyCell<Vec<ProjectProps>> = LazyCell::new(|| {
    vec![ProjectProps {
        title: "melon-rs",
        deployment: None,
        iframe: Some(html! {
        <iframe
            width="100%"
            height="100%"
            src="https://www.youtube.com/embed/vTKrAoVcFmA"
            title="YouTube video player"
            frameborder="0"
            allow="accelerometer; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
        >
        </iframe>
            }),
        summary: "An experimental frontend for the DS emulator, based on melonDS",
        bullets: vec![
            "Utilizes FFI to C++ using CXX",
            "60 fps & clean audio playback",
            "Savestates, Input recording, and playback for TAS creation",
        ],
        sources: vec![("Source", "https://github.com/benhall-7/diff-struct")],
    }]
});

const WEB_PROJECTS: LazyCell<Vec<ProjectProps>> = LazyCell::new(|| {
    vec![
        ProjectProps {
            title: "cube-ts",
            deployment: Some("https://www.npmjs.com/package/@benhall-7/cube-ts"),
            iframe: None,
            summary: "TypeScript utility library for querying Cube.JS",
            bullets: vec![
                "Define cube schemas, and automatically serialize typed data",
                "Allows fully customized types",
                "Creates parsers to automatically deserialize results",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/Portfolio")],
        },
        ProjectProps {
            title: "This Portfolio!",
            deployment: None,
            summary: "A single-page static portfolio with an integrated terminal",
            iframe: None,
            bullets: vec![
                "Powered by Rust and compiled in WebAssembly.",
                "Uses cool libraries like Clap, Yew, diff-struct, and more.",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/Portfolio")],
        },
        ProjectProps {
            title: "TidyHive",
            deployment: None,
            iframe: None,
            summary: "Task management app for groups",
            bullets: vec![
                "Interact with other users and todo's",
                "NodeJS server with PostgreSQL database",
                "Small 6 member development team",
            ],
            sources: vec![
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
    ]
});

const PRC_PROJECTS: LazyCell<Vec<ProjectProps>> = LazyCell::new(|| {
    vec![
        ProjectProps {
            title: "prc-rs",
            deployment: Some("https://github.com/ultimate-research/prc-rs/releases"),
            iframe: None,
            summary: "Rewrite of paracobNET library for SSBU param files (Rust)",
            bullets: vec![
                "Read + write speeds 10x faster than the C# implementation",
                "XML format conversion, compatible with version from paracobNET",
                "Derive macro to automatically interpret param data as a given type",
            ],
            sources: vec![("Source", "https://github.com/ultimate-research/prc-rs")],
        },
        ProjectProps {
            title: "pyprc",
            deployment: None,
            iframe: None,
            summary: "Python extension module based on prc-rs (PyO3)",
            bullets: vec![
                "Write scripts to edit param files dynamically",
                "Save time when game updates are released by defining what changes to make",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/pyprc")],
        },
        ProjectProps {
            title: "prickly",
            deployment: None,
            iframe: None,
            summary: "A 'prc-cli', a TUI interface for editing PRC files",
            bullets: vec![
                "Open and edit PRC files from the terminal, no GUI libraries needed",
                "Supports diverse set of operating systems",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/prickly")],
        },
        ProjectProps {
            title: "paracobNET",
            deployment: Some("https://github.com/benhall-7/paracobNET/releases/tag/v3.0"),
            iframe: None,
            summary: "Open source game modding tools for SSBU parameters",
            bullets: vec![
                "Alter character stats, playlists, and much more",
                "Code library to interact with '.prc' filetype (C#)",
                "User interface for easy editing capability (WPF, XML)",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/paracobNET/")],
        },
    ]
});

const MISC_PROJECTS: LazyCell<Vec<ProjectProps>> = LazyCell::new(|| {
    vec![
        ProjectProps {
            title: "diff-struct",
            deployment: None,
            iframe: None,
            summary: "Diffing functionality for generic structs, written in Rust",
            bullets: vec![],
            sources: vec![("Source", "https://github.com/benhall-7/diff-struct")],
        },
        ProjectProps {
            title: "musicli",
            deployment: None,
            iframe: None,
            summary: "A terminal-based MIDI file editor (Rust, TUI)",
            bullets: vec![],
            sources: vec![("Source", "https://github.com/benhall-7/musicli")],
        },
        ProjectProps {
            title: "yamlist",
            deployment: Some("https://github.com/ultimate-research/motion_lib/releases/"),
            iframe: None,
            summary: "Open source game modding tools for SSBU motion_list.bin files",
            bullets: vec![
                "Edit animation flags, such as blending, invincibility, cancellability, etc",
                "Converts from motion_list.bin into YML and back",
                "Supports diffing and patching changed files via diff-struct!",
            ],
            sources: vec![("Source", "https://github.com/ultimate-research/motion_lib")],
        },
    ]
});

impl Deref for Project {
    type Target = ProjectProps;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Component for Project {
    type Message = ();

    type Properties = ProjectProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self(ctx.props().clone())
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {<li class="project">
            <h2>{ match self.deployment {
                None => html! {self.title},
                Some(dep) => html! {<a href={dep}>{self.title}</a>},
            }}</h2>
            <p>{self.summary}</p>
            {
                if !self.bullets.is_empty() {
                    html! {<ul>{ for self.bullets.iter().map(|b| html! {<li>{b}</li>}) }</ul>}
                } else {
                    html! {}
                }
            }
            {
                if !self.sources.is_empty() {
                    self.sources
                        .iter()
                        .enumerate()
                        .map(|(i, (t, url))| html! { <>
                            {if i > 0 { " / " } else { "" }}
                            <a href={*url}>{t}</a>
                        </>})
                        .collect::<Html>()
                } else {
                    html! {}
                }
            }
        </li>}
    }
}

#[derive(Debug)]
pub struct Projects;

impl Component for Projects {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {<div class="comp-projects">
            <h1>{"Current Projects"}</h1>
            <div class="project-iframe">
                {for CURRENT_PROJECTS.iter().filter_map(|project| project.iframe.clone())}
            </div>
            <div >
                {for CURRENT_PROJECTS.iter().map(|project| html! {
                    <Project ..project.clone()/>
                })}
            </div>
            <hr />

            <h1>{"Web Projects"}</h1>
            {for WEB_PROJECTS.iter().map(|project| html! {
                <Project ..project.clone()/>
            })}

            <hr />

            <h1>{"PRC Projects"}</h1>
            {for PRC_PROJECTS.iter().map(|project| html! {
                <Project ..project.clone()/>
            })}

            <hr />

            <h1>{"MISC PROJECTS"}</h1>
            {for MISC_PROJECTS.iter().map(|project| html! {
                <Project ..project.clone()/>
            })}
        </div>}
    }
}
