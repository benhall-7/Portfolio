let menu_classes = ['menu-about', 'menu-skills', 'menu-projects'];
let data_classes = ['data-about', 'data-skills', 'data-projects'];

let menu = document.querySelector(".menu");
let data = document.querySelector(".data");

function select_from_menu(target) {
    menu.childNodes.forEach(node => {
        node.classList.remove("selected");
    });
    target.classList.add("selected");
}

menu_classes.forEach(name => {
    let query = "." + name;
    menu.querySelector(query).addEventListener("click", ev => {
        select_from_menu(ev.target);
        ev.stopPropagation();
    });
});

document.addEventListener("load", ev => {

});