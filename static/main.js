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
    let dates = document.getElementsByTagName("time");

    for (date of dates) {
        let dateData = new Date();

        dateData.setTime(
            Date.parse(date.textContent)
        );

        date.textContent = dateData.toLocaleString();
    }

    resizeTextareas();

    setInterval(resizeTextareas, 500);
}
