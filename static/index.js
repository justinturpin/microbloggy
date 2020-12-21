let imageUploadInput = document.getElementById("image-upload-input");
let imageUploadForm = document.getElementById("image-upload-form");

imageUploadForm.onsubmit = (e) => {
    e.preventDefault();

    let file = imageUploadInput.files.item(0);

    let reader = new FileReader();

    reader.onload = () => {
        let imageData = reader.result;

        let xhr = new XMLHttpRequest();

        // Reload page when the file is done uploading
        xhr.onreadystatechange = () => {
            if (xhr.readyState == 4) {
                location.reload();
            }
        }

        xhr.open("PUT", "/post/image-upload");
        xhr.setRequestHeader("Content-Type", "image/jpeg");
        xhr.send(imageData);
    }

    reader.readAsArrayBuffer(file);

    return false;
}
