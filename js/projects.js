projects = [
    {
        "title": "Potluck Planner",
        "description": "Potluck Planner provides users a way to manage potlucks; see guests attending, what food they want to bring, and more.",
        "tech": [
            "The 'home' and 'about' page of this project is designed simply through HTML, CSS, and some Javascript",
            "This side of the project was my own work; early on, our team planned a set of styles to work with so we would stay consistent",
            "The rest of the application is written with ReactJS."
        ],
        "deployment": "https://potluck-planner-lambda.github.io/ui-ben/",
        "repo": "https://github.com/Potluck-Planner-Lambda"
    },
    {
        "title": "paracobNET",
        "description": "A 4-tool suite of programs allowing loading and editing of the proprietary '.prc' filetype in Super Smash Bros. Ultimate",
        "tech": [
            "Currently I am the sole author of the project",
            "Written in C#, and utilizing the .NET Core framework",
            "Designed for portability as it can be used within other applications and on any computer architecture supported by .NET Core, like Windows, Linux, and MacOS."
        ],
        "deployment": "https://github.com/BenHall-7/paracobNET/releases/latest",
        "repo": "https://github.com/BenHall-7/paracobNET"
    }
];

let parent = document.querySelector(".data-projects");

function createProject(data) {
    let p = document.createElement("div");

    let deployment = document.createElement("a");
    let header = document.createElement("h1");
    let desc = document.createElement("p");
    let tech = document.createElement("ul");
    let github = document.createElement("a");

    p.classList.add("project");

    deployment.textContent = data.title;
    desc.textContent = data.description;
    github.textContent = "Source";

    deployment.setAttribute("href", data.deployment);
    github.setAttribute("href", data.repo);

    tech.append(...data.tech.map(point => {
        let t = document.createElement("li");
        t.textContent = point;
        return t;
    }));
    
    header.append(deployment);
    p.append(header, desc, tech, github);

    return p;
}

parent.append(...projects.map(proj => createProject(proj)));