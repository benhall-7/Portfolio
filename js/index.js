let consoleMain = document.getElementById("console");
let consoleInput = document.getElementById("console-input");
const NBSP = "\u00A0";

class ConsoleHistory {
    constructor() {
        let storage = sessionStorage.getItem("console-history");
        this.history = storage && JSON.parse(storage) || [];
        this.historyIndex = null;
    }

    get() {
        return this.history;
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
    console: {
        focused: false,
        text: "",
        position: 0,
        history: new ConsoleHistory(),
    },
};

function setConsoleHtml() {
    let text = state.console.text.replace(/ /g, NBSP);
    let position = state.console.position;
    // check if the cursor sits underneath a letter in the input string, or under a blank space
    if (position < text.length) {
        let before = text.slice(0, position);
        let on = text[position];
        let after = text.slice(position + 1);

        let cursorElem = document.createElement("span");
        cursorElem.append(on);
        cursorElem.classList.add("console-cursor", "cursor-blink");

        consoleInput.innerHTML = null;
        consoleInput.append(before, cursorElem, after);
    } else {
        let cursorElem = document.createElement("span");
        // non-breaking space (U-00A0). Can't use a regular " " character because HTML truncates it
        cursorElem.append(NBSP);
        cursorElem.classList.add("console-cursor", "cursor-blink");

        consoleInput.innerHTML = null;
        consoleInput.append(text, cursorElem);
    }
};

function updateConsole() {
    setConsoleHtml();
}

consoleMain.addEventListener("focus", e => {
    state.console.focused = true;
    let cursor = consoleInput.querySelector(".console-cursor");
    cursor.classList.add("cursor-blink");
});

consoleMain.addEventListener("blur", e => {
    state.console.focused = false;
    let cursor = consoleInput.querySelector(".console-cursor");
    cursor.classList.remove("cursor-blink");
});

consoleMain.addEventListener("keydown", e => {
    switch (e.key) {
        case "ArrowUp": {
            if (e.shiftKey) {
                let entry = state.console.history.dec();
                if (entry) {
                    state.console.text = entry;
                    state.console.position = entry.length;
                }
            } else {
                state.console.position = 0;
            }
            updateConsole();
            e.stopPropagation();
            break;
        }
        case "ArrowDown": {
            if (e.shiftKey) {
                let entry = state.console.history.inc();
                if (entry) {
                    state.console.text = entry;
                    state.console.position = entry.length;
                }
            } else {
                state.console.position = state.console.text.length;
            }
            updateConsole();
            e.stopPropagation();
            break;
        }
        case "ArrowLeft": {
            state.console.position = Math.max(0, state.console.position - 1);
            updateConsole();
            e.stopPropagation();
            break;
        }
        case "ArrowRight": {
            state.console.position = Math.min(state.console.text.length, state.console.position + 1);
            updateConsole();
            e.stopPropagation();
            break;
        }
        case "Backspace": {
            let text = state.console.text;
            let position = state.console.position;
            if (position > 0) {
                state.console.text = text.slice(0, position - 1) + text.slice(position);
                state.console.position -= 1;
                updateConsole();
            }
            e.stopPropagation();
            break;
        }
        case "Delete": {
            let text = state.console.text;
            let position = state.console.position;
            if (position < text.length) {
                state.console.text = text.slice(0, position) + text.slice(position + 1);
                updateConsole();
            }
            e.stopPropagation();
            break;
        }
        case "Enter": {
            state.console.history.push(state.console.text);
            state.console.text = "";
            state.console.position = 0;
            updateConsole();
            e.stopPropagation();
            break;
        }
    }
});

consoleMain.addEventListener("keypress", e => {
    if (e.key.length !== 1) return;
    state.console.text += e.key;
    state.console.position += 1;
    updateConsole();
})

consoleMain.addEventListener("submit", e => {
    console.log("submit");
})