let consoleMain = document.getElementById("console");
let consoleWrapper = document.getElementById("console-wrapper");
let content = document.getElementById("content");
const NBSP = "\u00A0";

// create a map of all the templates in the document by id
let templates = Array.from(document.getElementsByTagName("template")).reduce((prev, cur) => {
    prev[cur.id] = cur;
    return prev;
}, {});

class ConsoleHistory {
    constructor() {
        let storage = sessionStorage.getItem("console-history");
        this.history = storage && JSON.parse(storage) || [];
        this.historyIndex = null;
    }

    push(h) {
        if (this.history.length >= 50)
            this.history.shift();
        this.history.push(h);
        sessionStorage.setItem("console-history", JSON.stringify(this.history));
        this.historyIndex = null;
    }

    inc() {
        if (this.historyIndex !== null) {
            this.clampIndex(this.historyIndex + 1);
            return this.history[this.historyIndex];
        } else {
            return false;
        }
    }

    dec() {
        let before = this.historyIndex;
        if (this.historyIndex !== null) {
            this.clampIndex(this.historyIndex - 1);
        } else {
            // automatically clamped to max value
            this.clampIndex(Infinity);
        }

        if (this.historyIndex !== before) {
            return this.history[this.historyIndex];
        } else {
            return false;
        }
    }

    clampIndex(index) {
        if (this.history.length == 0) {
            this.historyIndex = null;
        } else {
            this.historyIndex = Math.min(Math.max(0, index), this.history.length - 1);
        }
    }

    length() {
        return this.history.length;
    }

    get(index) {
        return this.history[index];
    }

    clear() {
        this.history = [];
        this.historyIndex = null;
        localStorage.removeItem('console-history');
    }
}

class Iterator {
    constructor(array) {
        this.inner = array;
        this.index = 0;
    }

    current() {
        if (this.index < this.inner.length) {
            return this.inner[this.index];
        } else {
            return null;
        }
    }

    next() {
        this.index += 1;
        return this.current();
    }

    upto() {
        return this.inner.slice(0, this.index + 1).join(" ");
    }
}

let state = {
    cmdHistory: new ConsoleHistory(),
};

// object representation of commands for automatic input parsing
// TODO: help text should be automatically generated for each command
const COMMANDS = {
    // if "commands" is an object, do a foreach loop
    about: { template: "input-about" },
    contact: { template: "input-contact" },
    skills: { template: "input-skills" },
    projects: { template: "input-projects" },
    help: { template: "input-help" },
    history: {
        default: {
            do: () => {
                // TODO: find a way to turn this into a representative object
                let header = document.createElement("h2");
                header.textContent = "Command history"
                let count = document.createElement("p");
                count.textContent = "count: " + state.cmdHistory.length();
                let list = document.createElement("ul");
                for (let i = state.cmdHistory.length() - 1; i >= 0; i--) {
                    let entry = document.createElement("li");
                    let value = state.cmdHistory.get(i);
                    entry.append(`${i} - `);
                    let cmd = document.createElement("span");
                    cmd.tabIndex = 0;
                    cmd.classList.add("cmd");
                    cmd.onclick = () => submitCmd(value);
                    cmd.textContent = value;
                    entry.append(cmd);
                    list.appendChild(entry);
                }
                content.append(header, count, list);
            }
        },
        // here the commands need to be an array because we have one that conditionally runs
        // (based on if the input is a number)
        commands: [
            {
                name: "clear",
                template: "input-history-clear",
                ret: () => state.cmdHistory.clear(),
            }, {
                conditional: arg => Number.isInteger(Number(arg)),
                do: arg => {
                    let value = Number(arg);
                    if (value < 0)
                        value += state.cmdHistory.length();
                    if (value >= 0 && value < state.cmdHistory.length()) {
                        let output = document.createElement("p");
                        let text = state.cmdHistory.get(value);
                        output.textContent = text;
                        output.tabIndex = 0;
                        output.classList.add("cmd");
                        output.onclick = () => submitCmd(text);
                        content.appendChild(output);
                    } else {
                        let output = document.createElement("p");
                        output.textContent = "The specified index was not in the bounds of the history array";
                        content.appendChild(output);
                    }
                }
            },
        ]
    },
}

function findCommand(commands, arg) {
    let find = null;
    if (Array.isArray(commands)) {
        for (const cmd of commands) {
            if (cmd.name === arg) {
                find = cmd;
                break;
            } else if (cmd.conditional && cmd.conditional(arg)) {
                find = cmd;
                break;
            }
        }
    } else {
        for (const [name, cmd] of Object.entries(commands)) {
            if (name === arg) {
                find = cmd;
                break;
            }
        }
    }
    return find;
}

function doCommand(cmd, args) {
    // to do: clarify if 'do' should come before or after 'template'
    if (cmd.do)
        cmd.do(args.current());

    if (cmd.template) {
        content.appendChild(templates[cmd.template].content.cloneNode(true));
    }
    
    if (cmd.ret) {
        return () => cmd.ret(args.current());
    } else {
        return null;
    }
}

function recurseCommands(cmds, args) {
    const cmd = findCommand(cmds, args.current());
    if (cmd) {
        if (cmd.commands) {
            // if no further arguments are passed, there needs to be a default
            if (!args.next()) {
                if (cmd.default) {
                    return doCommand(cmd.default, args);
                } else {
                    // error message for when no default is available
                    let response = templates["input-default-invalid"].content.cloneNode(true);
                    response.querySelector(".arg0").append(args.upto());
                    content.appendChild(response);
                }
            } else {
                return recurseCommands(cmd.commands, args);
            }
        } else {
            return doCommand(cmd, args);
        }
    } else {
        let response = templates["input-invalid"].content.cloneNode(true);
        response.querySelector(".arg0").append(args.upto());
        content.appendChild(response);
    }
}

function handleCommand(args) {
    content.innerHTML = null;
    let iterator = new Iterator(args);
    if (iterator.current()) {
        return recurseCommands(COMMANDS, iterator);
    } else {
        content.appendChild(templates["input-none"].content.cloneNode(true));
    }
}

function submitCmd(commandStr) {
    let cmd = commandStr || consoleMain.value;
    // trim front and back, and split words up by spaces
    let cb = handleCommand(cmd.trim().split(/ +/g));
    state.cmdHistory.push(cmd);
    consoleMain.value = "";
    consoleMain.blur();
    if (typeof cb == "function") cb();
}

consoleMain.addEventListener("keydown", e => {
    switch (e.key) {
        case "ArrowUp": {
            if (e.shiftKey) {
                let entry = state.cmdHistory.dec();
                if (entry)
                    consoleMain.value = entry; 
                e.preventDefault();
            }
            break;
        }
        case "ArrowDown": {
            if (e.shiftKey) {
                let entry = state.cmdHistory.inc();
                if (entry)
                    consoleMain.value = entry;
                e.preventDefault();
            }
            break;
        }
    }
});

consoleWrapper.addEventListener("submit", e => {
    submitCmd();
    e.preventDefault();
});