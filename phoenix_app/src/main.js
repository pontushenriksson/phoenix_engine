document.addEventListener('DOMContentLoaded', () => {
  // default projects: counter‑strike 3 and fortnite 2 will be shown initially
  let projects = [
    {
      name: 'Counter-Strike 3',
      thumbnail: './assets/thumbnails/counter-strike-3.jpg',
      link: './editor/counter-strike-3.html',
    },
    {
      name: 'Fortnite 2',
      thumbnail: './assets/thumbnails/fortnite-2.jpg',
      link: './editor/fortnite-2.html',
    },
  ];

  const projectsContainer = document.querySelector('.projects');

  // render project cards including the create card
  function renderProjects() {
    projectsContainer.innerHTML = '';
    const createProjectCard = `
      <div id="createProject" class="project">
        <i class="createProjectIcon fa-solid fa-circle-plus"></i>
        <h3>Create new project</h3>
      </div>
    `;
    projectsContainer.insertAdjacentHTML('beforeend', createProjectCard);
    document
      .querySelector('#createProject')
      .addEventListener('click', openModal);

    projects.forEach((project) => {
      const projectCard = `
        <article class="project ${project.isNew ? 'new' : ''}">
          <div class="top">
            <img src="${project.thumbnail}" alt="${project.name} thumbnail" />
            <h3>${project.name}</h3>
          </div>
          <div class="bottom">
            <a href="${project.link}">Open project</a>
            <i class="projectSettingsIcon fa-solid fa-gear"></i>
          </div>
        </article>
      `;
      projectsContainer.insertAdjacentHTML('beforeend', projectCard);
      if (project.isNew) project.isNew = false;
    });
  }

  // open the modal popup
  function openModal() {
    const modalOverlay = document.querySelector('.modal-overlay');
    modalOverlay.classList.add('active');
    const modalPopup = modalOverlay.querySelector('.modal-popup');
    modalPopup.classList.add('active');
    modalOverlay.querySelector('#newProjectName').value = '';
    modalOverlay.querySelector('#newProjectName').focus();
  }

  // close the modal popup
  function closeModal() {
    const modalOverlay = document.querySelector('.modal-overlay');
    modalOverlay.querySelector('.modal-popup').classList.remove('active');
    modalOverlay.classList.remove('active');
  }

  // add a new project from the modal; only "Minecraft 2" is available for demo
  function addNewProject() {
    const inputEl = document.querySelector('#newProjectName');
    const projectName = inputEl.value.trim();
    if (projectName === '') {
      alert('Please enter a project name.');
      return;
    }
    if (projectName === 'Minecraft 2') {
      const newProject = {
        name: 'Minecraft 2',
        thumbnail: './assets/thumbnails/minecraft-2.jpg',
        link: './editor/minecraft-2.html',
        isNew: true,
      };
      if (projects.some((p) => p.name === 'Minecraft 2')) {
        alert("Project 'minecraft 2' already exists.");
      } else {
        projects.push(newProject);
      }
      closeModal();
      renderProjects();
    } else {
      alert("Project not recognized. Only 'Minecraft 2' is available.");
    }
  }

  // insert modal popup HTML into the document
  const modalHTML = `
    <div class="modal-overlay">
      <div class="modal-popup">
        <h2>create new project</h2>
        <input type="text" id="newProjectName" placeholder="Enter project name">
        <div class="modal-buttons">
          <button id="createProjectBtn">Create</button>
          <button id="cancelProjectBtn">Cancel</button>
        </div>
      </div>
    </div>
  `;
  document.body.insertAdjacentHTML('beforeend', modalHTML);
  document
    .querySelector('#createProjectBtn')
    .addEventListener('click', addNewProject);
  document
    .querySelector('#cancelProjectBtn')
    .addEventListener('click', closeModal);
  document.querySelector('.modal-overlay').addEventListener('click', (e) => {
    if (e.target.classList.contains('modal-overlay')) {
      closeModal();
    }
  });

  renderProjects();

  // theme switcher functionality
  const themeSwitcher = document.querySelector('#themeSwitcher');
  themeSwitcher.addEventListener('click', () => {
    document.body.classList.toggle('light-theme');
  });
});
