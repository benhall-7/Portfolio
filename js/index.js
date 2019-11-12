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
    });
    Array.from(data.children).forEach(node => {
        node.style.display = "none";
    })
    data_node.style.display = "block";
    target.classList.add("selected");
}

classes.forEach(name => {
    let query = ".menu-" + name;
    menu.querySelector(query).addEventListener("click", ev => {
        select_from_menu(ev.target);
        ev.stopPropagation();
    });
});

select_from_menu(document.querySelector(".menu-" + classes[0]));