pub struct Skill {
    pub category: &'static str,
    pub bullets: &'static [&'static str],
}

pub const SKILLS: &[Skill] = &[
    Skill {
        category: "Languages",
        bullets: &["Rust, Python, Javascript, C#, Lua (ordered)"],
    },
    Skill {
        category: "Frontend",
        bullets: &["HTML, CSS", ".NET Core (WPF)"],
    },
    Skill {
        category: "Web Frameworks",
        bullets: &["React, Redux, Express", "LESS", "Yew", "WebAssembly"],
    },
    Skill {
        category: "Databases",
        bullets: &["SQLite3, Postgres"],
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
