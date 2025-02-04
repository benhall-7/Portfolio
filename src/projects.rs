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
        title: "prc-rs",
        deployment: Some("https://github.com/ultimate-research/prc-rs/releases"),
        summary: "Rewrite of paracobNET library for SSBU param files (Rust)",
        bullets: &[
            "Read + write speeds up to 10x faster than the C# implementation",
            "param-xml reimplementation compatible with version on paracobNET",
        ],
        sources: &[("Source", "https://github.com/ultimate-research/prc-rs")],
    },
    Project {
        title: "pyprc",
        deployment: None,
        summary: "Python extension module based on prc-rs (PyO3)",
        bullets: &[
            "Write scripts to edit param files dynamically",
            "Save time when game updates are released by defining what changes to make",
        ],
        sources: &[("Source", "https://github.com/benhall-7/pyprc")],
    },
    Project {
        title: "paracobNET",
        deployment: Some("https://github.com/benhall-7/paracobNET/releases/tag/v3.0"),
        summary: "Open source game modding tools for SSBU parameters",
        bullets: &[
            "Alter character stats, playlists, and much more",
            "Code library to interact with '.prc' filetype (C#)",
            "User interface for easy editing capability (WPF, XML)",
        ],
        sources: &[("Source", "https://github.com/benhall-7/paracobNET/")],
    },
    Project {
        title: "This Portfolio!",
        deployment: None,
        summary: "A super cool, dynamic, terminal-powered single page app of awesomeness",
        bullets: &[
            "Powered by Rust, and cool libraries like Clap and Yew, (compiled in WebAssembly)",
        ],
        sources: &[("Source", "https://github.com/benhall-7/Portfolio")],
    },
    Project {
        title: "diff-struct",
        deployment: None,
        summary: "Diffing functionality for generic structs (Rust)",
        bullets: &[],
        sources: &[("Source", "https://github.com/benhall-7/diff-struct")],
    },
];

pub const CURRENT_PROJECTS: &'static [Project] = &[Project {
    title: "melon-rs",
    deployment: None,
    summary: "An experimental frontend for the DS emulator, based on melonDS",
    bullets: &[
        "Utilizes FFI to C++ using CXX",
        "60 fps playback, clean audio playback",
        "Savestates, Input recording, and playback for TAS creation",
    ],
    sources: &[("Source", "https://github.com/benhall-7/diff-struct")],
}];

pub const OTHER_PROJECTS: &'static [Project] = &[
    Project {
        title: "musicli",
        deployment: None,
        summary: "A terminal-based MIDI file editor (Rust, TUI)",
        bullets: &[],
        sources: &[("Source", "https://github.com/benhall-7/musicli")],
    },
    Project {
        title: "yamlist",
        deployment: Some("https://github.com/ultimate-research/motion_lib/releases/"),
        summary: "Open source game modding tools for SSBU motion_list.bin files",
        bullets: &[
            "Edit animation flags, such as blending, invincibility, cancellability, etc",
            "Converts from motion_list.bin into YML and back",
            "Supports diffing and patching changed files via diff-struct!",
        ],
        sources: &[("Source", "https://github.com/ultimate-research/motion_lib")],
    },
    Project {
        title: "TidyHive",
        deployment: None,
        summary: "Task management app for groups",
        bullets: &[
            "Interact with other users and todo's",
            "NodeJS server with PostgreSQL database",
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
];

pub fn render_projects(projects: &[Project]) -> Html {
    html! {
        for projects.iter().map(|project| {
            html! {<li class="project">
                <h2>{ match project.deployment {
                    None => html! {project.title},
                    Some(dep) => html! {<a href=dep>{project.title}</a>},
                }}</h2>
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
