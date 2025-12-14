use std::cell::LazyCell;
use yew::prelude::*;

use project_group::{ImageProps, ProjectGroup, ProjectGroupProps, ProjectInfo, Side};

mod project_group;

pub const CURRENT_PROJECTS: LazyCell<ProjectGroupProps> = LazyCell::new(|| ProjectGroupProps {
    title: "Current projects",
    projects: vec![
        (ProjectInfo {
            title: "taxonia",
            deployment: Some("https://taxonia.app"),
            summary: "A quizzy webapp to help users learn scientific taxonomy",
            bullets: vec![
                "Take and save quizzes using any location or class of organisms",
                "Features a frontend built in TypeScript and React, and a backend built in Rust",
                "Additional frontend tools include: the iNaturalist API, Material UI, TanStack Router, and GitHub Actions",
                "Additional backend tools include: Docker, Postgres, Redis, Poem, and OpenAPI",
            ],
            sources: vec![
                ("Source", "https://github.com/benhall-7/taxonia"),
                ("Frontend", "https://taxonia.app"),
                ("Backend", "https://api.taxonia.app/spec"),
            ],
        }),
        (ProjectInfo {
            title: "melon-rs",
            deployment: None,
            summary: "An experimental frontend for the DS emulator, based on melonDS",
            bullets: vec![
                "Utilizes FFI to C++ using CXX",
                "60 fps & clean audio playback",
                "Savestates, Input recording, and playback for TAS creation",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/diff-struct")],
        }),
    ],
    content: vec![
        ImageProps {
            src: "img/taxonia1.png",
            alt: "an image showing a browser window at the 'taxonia.app' URL. On the \
                page there is an autocomplete dropdown with the input 'orchid'",
        },
        ImageProps {
            src: "img/melon-rs.png",
            alt: "an image showing two windows, one with a terminal and some debug \
                info and another demonstrating the game: Kirby Super Star Ultra",
        },
    ],
    content_side: Side::Right,
});

pub const WEB_PROJECTS: LazyCell<ProjectGroupProps> = LazyCell::new(|| ProjectGroupProps {
    title: "Web projects",
    projects: vec![
        (ProjectInfo {
            title: "cube-ts",
            deployment: Some("https://www.npmjs.com/package/@benhall-7/cube-ts"),
            summary: "TypeScript utility library for querying Cube.JS",
            bullets: vec![
                "Define cube schemas, and automatically serialize typed data",
                "Allows fully customized types",
                "Creates parsers to automatically deserialize results",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/Portfolio")],
        }),
        (ProjectInfo {
            title: "This Portfolio!",
            deployment: None,
            summary: "A single-page static portfolio with an integrated terminal",
            bullets: vec![
                "Powered by Rust and compiled in WebAssembly.",
                "Uses cool libraries like Clap, Yew, diff-struct, and more.",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/Portfolio")],
        }),
        (ProjectInfo {
            title: "TidyHive",
            deployment: None,
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
        }),
    ],
    content: vec![],
    content_side: Side::Right,
});

pub const PRC_PROJECTS: LazyCell<ProjectGroupProps> = LazyCell::new(|| ProjectGroupProps {
    title: "PRC projects",
    projects: vec![
        (ProjectInfo {
            title: "prc-rs",
            deployment: Some("https://github.com/ultimate-research/prc-rs/releases"),
            summary: "Rewrite of paracobNET library for SSBU param files (Rust)",
            bullets: vec![
                "Read + write speeds 10x faster than the C# implementation",
                "XML format conversion, compatible with version from paracobNET",
                "Derive macro to automatically interpret param data as a given type",
            ],
            sources: vec![("Source", "https://github.com/ultimate-research/prc-rs")],
        }),
        (ProjectInfo {
            title: "pyprc",
            deployment: Some("https://pypi.org/project/pyprc"),
            summary: "Python extension module based on prc-rs (PyO3)",
            bullets: vec![
                "Write scripts to edit param files dynamically",
                "Save time when game updates are released by defining what changes to make",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/pyprc")],
        }),
        (ProjectInfo {
            title: "prickly",
            deployment: None,
            summary: "A 'prc-cli', a TUI interface for editing PRC files",
            bullets: vec![
                "Open and edit PRC files from the terminal, no GUI libraries needed",
                "Supports diverse set of operating systems",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/prickly")],
        }),
        (ProjectInfo {
            title: "paracobNET",
            deployment: Some("https://github.com/benhall-7/paracobNET/releases/tag/v3.0"),
            summary: "Open source game modding tools for SSBU parameters",
            bullets: vec![
                "Alter character stats, playlists, and much more",
                "Code library to interact with '.prc' filetype (C#)",
                "User interface for easy editing capability (WPF, XML)",
            ],
            sources: vec![("Source", "https://github.com/benhall-7/paracobNET/")],
        }),
    ],
    content: vec![
        ImageProps {
            src: "img/prcEditor.png",
            alt: "An image showing a desktop application. On the left hand side is \
                  a tree=like structure, and the right side is a table of data",
        },
        ImageProps {
            src: "img/prickly.webp",
            alt: "An image of a terminal program featuring nested directories, and \
                  in each one is a list of names as well as their types and values",
        },
    ],
    content_side: Side::Left,
});

pub const MISC_PROJECTS: LazyCell<ProjectGroupProps> = LazyCell::new(|| ProjectGroupProps {
    title: "Misc projects",
    projects: vec![
        (ProjectInfo {
            title: "diff-struct",
            deployment: None,
            summary: "Diffing functionality for generic structs, written in Rust",
            bullets: vec![],
            sources: vec![("Source", "https://github.com/benhall-7/diff-struct")],
        }),
        (ProjectInfo {
            title: "musicli",
            deployment: None,
            summary: "A terminal-based MIDI file editor (Rust, TUI)",
            bullets: vec![],
            sources: vec![("Source", "https://github.com/benhall-7/musicli")],
        }),
        (ProjectInfo {
            title: "yamlist",
            deployment: Some("https://github.com/ultimate-research/motion_lib/releases/"),
            summary: "Open source game modding tools for SSBU motion_list.bin files",
            bullets: vec![
                "Edit animation flags, such as blending, invincibility, cancellability, etc",
                "Converts from motion_list.bin into YML and back",
                "Supports diffing and patching changed files via diff-struct!",
            ],
            sources: vec![("Source", "https://github.com/ultimate-research/motion_lib")],
        }),
    ],
    content: vec![ImageProps {
        src: "img/musicli.jpg",
        alt: "An image of a terminal program showing a piano keyboard on the left \
                  and notes of a piece of music to the right of the keys",
    }],
    content_side: Side::Right,
});

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
            <ProjectGroup ..CURRENT_PROJECTS.clone() />
            <hr />
            <ProjectGroup ..WEB_PROJECTS.clone() />
            <hr />
            <ProjectGroup ..PRC_PROJECTS.clone() />
            <hr />
            <ProjectGroup ..MISC_PROJECTS.clone() />
        </div>}
    }
}
