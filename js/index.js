let consoleMain = document.getElementById("console");
let consoleWrapper = document.getElementById("console-wrapper");
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

let state = {
    cmdHistory: new ConsoleHistory(),
};

async function handleCommand(commands) {
    let content = document.getElementById("content");
    content.innerHTML = null;

    switch (commands[0]) {
        case "": {
            content.appendChild(templates["input-none"].content.cloneNode(true));
            break;
        }
        case "help": {
            content.appendChild(templates["input-help"].content.cloneNode(true));
            break;
        }
        case "about": {
            content.appendChild(templates["input-about"].content.cloneNode(true));
            break;
        }
        case "contact": {
            content.appendChild(templates["input-contact"].content.cloneNode(true));
            break;
        }
        case "skills": {
            content.appendChild(templates["input-skills"].content.cloneNode(true));
            break;
        }
        case "projects": {
            content.appendChild(templates["input-projects"].content.cloneNode(true));
            break;
        }
        case "history": {
            if (commands.length > 1) {
                const second = commands[1];
                if (second == "help") {
                    content.appendChild(templates["input-history-help"].content.cloneNode(true));
                } else if (second === "clear") {
                    // since default behavior is to push new commands after this function exits
                    // we send this callback to remove it later instead
                    return () => {
                        state.cmdHistory.clear();
                        content.append("Command history cleared.");
                    };
                } else if (Number.isInteger(Number(second))) {
                    let value = Number(second);
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
                } else {
                    let response = templates["input-history-invalid"].content.cloneNode(true);
                    response.querySelector(".arg0").append(commands[1]);
                    content.appendChild(response);
                }
            } else {
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
            break;
        }
        default: {
            let response = templates["input-invalid"].content.cloneNode(true);
            response.querySelector(".arg0").append(commands[0]);
            content.appendChild(response);
            break;
        }
    }
}

function submitCmd(commandStr) {
    let cmd = commandStr || consoleMain.value;
    // trim front and back, and split words up by spaces
    handleCommand(cmd.trim().split(/ +/g)).then(cb => {
        state.cmdHistory.push(cmd);
        consoleMain.value = "";    
        if (typeof cb == "function") cb();
    });
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