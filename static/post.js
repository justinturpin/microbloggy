let postEditToggleButton = document.getElementById("edit-post-toggle-button");
let deletePostToggleButton = document.getElementById("delete-post-toggle-button");
let postEditContainer = document.getElementById("post-edit-container");
let postStaticContainer = document.getElementById("post-static-container");
let modalContainer = document.getElementById("modal-container");

postEditToggleButton.onclick = (e) => {
    postEditContainer.style.display = "block";
    postStaticContainer.style.display = "none";

    resizeTextareas();
}

deletePostToggleButton.onclick = (e) => {
    /* Start modal stuff */
    modalContainer.style.display = "block";
}

modalContainer.onclick = (e) => {
    if (e.target == modalContainer) {
        modalContainer.style.display = "none";
    }
}
