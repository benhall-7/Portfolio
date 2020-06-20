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
    handleCommand(cmd.trim().split(/ +/g));
    state.cmdHistory.push(cmd);
    consoleMain.value = "";
}

consoleMain.addEventListener("keydown", e => {
    switch (e.key) {
        case "ArrowUp": {
            if (e.shiftKey) {
                let entry = state.cmdHistory.dec();
                if (entry)
                    consoleMain.value = entry; 
                e.stopPropagation();
            }
            break;
        }
        case "ArrowDown": {
            if (e.shiftKey) {
                let entry = state.cmdHistory.inc();
                if (entry)
                    consoleMain.value = entry;
                e.stopPropagation();
            }
            break;
        }
    }
});

consoleWrapper.addEventListener("submit", e => {
    submitCmd();
    e.preventDefault();
});