let consoleMain = document.getElementById("console");
let consoleWrapper = document.getElementById("console-wrapper");
let content = document.getElementById("content");
let gameParams = {
    size: 20,
    x: 10,
    y: 10,
    alive: "#ffffff",
    dead: "#808080",
};
let gameIntervalHandle = null;

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
    diff: { template: "input-diff" },
    conway: { template: "input-conway" }
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

function clean() {
    if (gameIntervalHandle) {
        clearInterval(gameIntervalHandle);
        gameIntervalHandle = null;
    }
    if (window.GAME) {
        GAME = null;
    }
}

function submitCmd(commandStr) {
    let cmd = commandStr || consoleMain.value;
    // trim front and back, and split words up by spaces
    let cb = handleCommand(cmd.toLowerCase().trim().split(/ +/g));
    state.cmdHistory.push(cmd);
    consoleMain.value = "";
    consoleMain.blur();
    if (typeof cb == "function") cb();
    clean();
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

function handleDiffChange() {
    let a = document.getElementById("diff-a").value;
    let b = document.getElementById("diff-b").value;
    let diff = Diff(a, b);
    let output = document.getElementById("diff-output");
    output.textContent = "";
    let pos = 0;
    for (const change of diff) {
        const next = change.get("index");
        if (next > 0) {
            output.append(a.slice(pos, next));
            pos = next;
        }
        switch (change.get("type")) {
            case "removed": {
                let deleted = document.createElement("span");
                deleted.classList.add("diff-remove");
                deleted.append(a.slice(pos, pos + change.get("len")));
                output.append(deleted);
                pos += change.get("len");
                break;
            }
            case "inserted": {
                let inserted = document.createElement("span");
                inserted.classList.add("diff-insert");
                inserted.append(change.get("changes"));
                output.append(inserted);
                break;
            }
            case "altered": {
                let deleted = document.createElement("span");
                deleted.classList.add("diff-alter-a");
                deleted.append(a.slice(pos, pos + change.get("changes").length));
                let inserted = document.createElement("span");
                inserted.classList.add("diff-alter-b");
                inserted.append(change.get("changes"));
                output.append(deleted, inserted);
                pos += change.get("changes").length;
                break;
            }
        }
    }
    if (pos < a.length) {
        output.append(a.slice(pos));
    }
}

async function conwayStart() {
    let canvas = document.getElementById("game-canvas");
    window.GAME = new GameOfLife(gameParams.x, gameParams.y, gameParams.size);
    GAME.set_on([[0, 1], [1, 2], [2, 0], [2, 1], [2, 2]]);
    GAME.draw(canvas, gameParams.alive, gameParams.dead);
    
    canvas.addEventListener('click', e => {
        const rect = canvas.getBoundingClientRect();
        const x = Math.floor((e.clientX - rect.left) / gameParams.size);
        const y = Math.floor((e.clientY - rect.top) / gameParams.size);
        GAME.invert([[y, x]]);
        GAME.draw(canvas, gameParams.alive, gameParams.dead);
    });

    let size_form = document.getElementById("conway-size-form");
    size_form.addEventListener("submit", e => {
        e.preventDefault();
        let height = document.getElementById("conway-width").value;
        let width = document.getElementById("conway-height").value;
        height = height.length > 0 ? Number(height) : gameParams.y;
        width = width.length > 0 ? Number(width) : gameParams.x;
        GAME = new GameOfLife(width, height, gameParams.size);
        GAME.draw(canvas, gameParams.alive, gameParams.dead);
    })
}

function __conwayStep() {
    let canvas = document.getElementById("game-canvas");
    GAME.step();
    GAME.draw(canvas, gameParams.alive, gameParams.dead);
}

async function conwayStep() {
    if (!gameIntervalHandle) {
        __conwayStep()
    }
}

async function conwayPlay() {
    if (!gameIntervalHandle) {
        gameIntervalHandle = setInterval(__conwayStep, 1000 / 10);

        let play = document.getElementById("conway-controls-play");
        let pause = document.getElementById("conway-controls-pause");
        let step = document.getElementById("conway-controls-step");
        play.disabled = true;
        pause.disabled = false;
        step.disabled = true;
    }
}

async function conwayStop() {
    if (gameIntervalHandle) {
        clearInterval(gameIntervalHandle)
        gameIntervalHandle = null;
        
        let play = document.getElementById("conway-controls-play");
        let pause = document.getElementById("conway-controls-pause");
        let step = document.getElementById("conway-controls-step");
        play.disabled = false;
        pause.disabled = true;
        step.disabled = false;
    }
}

async function conwayReset() {
    await conwayStop();
    let canvas = document.getElementById("game-canvas");
    GAME.clear();
    GAME.draw(canvas, gameParams.alive, gameParams.dead);
}

async function setPreset(type) {
    let canvas = document.getElementById("game-canvas");
    switch (type) {
        case "blinker": {
            GAME = new GameOfLife(5, 5, gameParams.size);
            GAME.set_on([[1, 2], [2, 2], [3, 2]]);
            break;
        }
        case "pentadec": {
            GAME = new GameOfLife(11, 18, gameParams.size);
            GAME.set_on([
                [4, 5], [5, 5],
                [6, 4], [6, 6],
                [7, 5], [8, 5], [9, 5], [10, 5],
                [11, 4], [11, 6],
                [12, 5], [13, 5]
            ]);
            break;
        }
        case "lwss": {
            GAME = new GameOfLife(25, 7, gameParams.size);
            GAME.set_on([[1, 1], [3, 1], [4, 2], [4, 3], [4, 4], [4, 5], [3, 5], [2, 5], [1, 4]]);
            break;
        }
        default: return;
    }
    GAME.draw(canvas, gameParams.alive, gameParams.dead);
}
