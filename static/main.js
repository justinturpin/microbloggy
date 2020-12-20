function resizeTextareas() {
    let textareas = document.getElementsByTagName("textarea");

    for (textarea of textareas) {
        // Don't know why this is necessary
        textarea.style.height = "inherit";

        let computed = window.getComputedStyle(textarea);

        let newHeight = (
            parseFloat(computed.getPropertyValue("border-top-width"))
            + parseFloat(computed.getPropertyValue("border-bottom-width"))
            + parseFloat(computed.getPropertyValue("padding-top"))
            + parseFloat(computed.getPropertyValue("padding-bottom"))
            + textarea.scrollHeight
        );

        textarea.style.height = newHeight + "px";
    }
}

window.onload = (e) => {
    // Turn all UTC dates into friendly, browser-timezone-local strings
    let dates = document.getElementsByTagName("time");

    for (date of dates) {
        let dateData = new Date();

        dateData.setTime(
            Date.parse(date.textContent)
        );

        date.textContent = dateData.toLocaleString();
    }

    // Resize all textareas
    resizeTextareas();

    // Regularly resize all textareas instead of hooking into every key input
    // one very textarea like some wonderful Javascript libraries want to do
    setInterval(resizeTextareas, 500);
}
