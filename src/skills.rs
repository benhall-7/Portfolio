pub struct Skill {
    pub category: &'static str,
    pub bullets: &'static [&'static str],
}

pub const SKILLS: &[Skill] = &[
    Skill {
        category: "Languages",
        bullets: &[
            "TypeScript",
            "Javascript",
            "Rust, Python, Ruby, C#, Lua (roughly ordered)",
        ],
    },
    Skill {
        category: "Frontend",
        bullets: &["HTML, CSS", ".NET Core (WPF)"],
    },
    Skill {
        category: "Backend",
        bullets: &["NodeJS", "Ruby on Rails"],
    },
    Skill {
        category: "Web Frameworks",
        bullets: &["React, Redux, Express", "LESS", "Yew", "WebAssembly"],
    },
    Skill {
        category: "Databases",
        bullets: &["MySQL, Postgres"],
    },
    Skill {
        category: "Build systems",
        bullets: &["NodeJS, Cargo"],
    },
    Skill {
        category: "CI / CD",
        bullets: &["GitHub Actions, Pages", "Appveyor", "Heroku"],
    },
    Skill {
        category: "Version Control",
        bullets: &["Git"],
    },
    Skill {
        category: "Testing",
        bullets: &["Jest, Cargo"],
    },
];
