function previewFile() {
    const preview = document.querySelector("img");
    const file = document.querySelector("input[type=file]").files[0];
    const reader = new FileReader();

    reader.addEventListener(
        "load",
        () => {
            // convert image file to base64 string
            preview.src = reader.result;
        },
        false,
    );

    if (file) {
        reader.readAsDataURL(file);
    }

    // canvas = document.createElement('canvas');
    // canvas.width = 1000;
    // canvas.height = 1000;
    // canvas.getContext('2d').drawImage(preview, 0, 0);
}
