const classes = ['about', 'skills', 'projects'];

let menu = document.querySelector(".menu");
let data = document.querySelector(".data");

function select_from_menu(target) {
    let target_category = classes.reduce((real, name) => {
        if (real === undefined && target.classList.contains("menu-" + name)) {
            return name;
        } else {
            return real;
        }
    }, undefined);
    data_node = data.querySelector(".data-" + target_category);

    Array.from(menu.children).forEach(node => {
        node.classList.remove("selected");
        let arrow = node.querySelector(".arrow");
        if (arrow) {
            arrow.remove();
        }
    });
    Array.from(data.children).forEach(node => {
        node.style.display = "none";
    })

    let arrow = document.createElement("span");
    arrow.textContent = ">";
    arrow.classList.add("arrow");

    data_node.style.display = "block";
    target.classList.add("selected");
    target.prepend(arrow);
}

classes.forEach(name => {
    let query = ".menu-" + name;
    menu.querySelector(query).addEventListener("click", ev => {
        select_from_menu(ev.target);
        ev.stopPropagation();
    });
});

select_from_menu(document.querySelector(".menu-" + classes[0]));