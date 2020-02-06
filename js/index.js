let hidden;
let data;

const ABOUT = 'ABOUT';
const SKILLS = 'SKILLS';
const PROJECTS = 'PROJECTS';
const CURRENT_ID = "CURRENT_ID";

const states = {
  ABOUT: {},
  SKILLS: {},
  PROJECTS: {},
};

const setData = contentID => {
  while (data.lastChild) {
    data.lastChild.remove();
  }
  data.appendChild(states[contentID]);
  localStorage.setItem(CURRENT_ID, contentID);
}

const setAboutMe = () => setData(ABOUT);
const setSkills = () => setData(SKILLS);
const setProjects = () => setData(PROJECTS);

function loadData() {
  hidden = document.getElementById("hidden");
  data = document.getElementById("data");

  states[ABOUT] = hidden.querySelector("#about");
  states[SKILLS] = hidden.querySelector("#skills");
  states[PROJECTS] = hidden.querySelector("#projects");
  
  const currentId = localStorage.getItem(CURRENT_ID);
  if (currentId) {
    setData(currentId);
  } else {
    setAboutMe();
  }
}
