var target_brows = document.getElementById("fileb");
var target_drag = document.getElementById("box");
var img1 = document.getElementById("img1");
var img2 = document.getElementById("img2");
var img3 = document.getElementById("img3");
var img4 = document.getElementById("img4");
var img5 = document.getElementById("img5");
var img6 = document.getElementById("img6");

// CLICK SUBMIT
target_brows.addEventListener("change", function (event) {
  event.preventDefault();
  loadDoc(target_brows);
});

// DRAG AND DROP
document.body.addEventListener("dragover", function (event) {
  event.preventDefault();
});
document.body.addEventListener("drop", function (event) {
  event.preventDefault();
});
target_drag.addEventListener(
  "dragover",
  function (event) {
    // prevent default on box and change color
    event.preventDefault();
    target_drag.style.backgroundColor = "#3bb477";
  },
  false
);
target_drag.addEventListener(
  "drop",
  function (event) {
    // cancel default actions
    event.preventDefault();
    // restore default when drop
    target_drag.style.backgroundColor = "#3b7682";
    loadDoc(event.dataTransfer);
  },
  false
);

// check type
var fileTypes = [
  "image/jpeg",
  "image/jpg",
  "image/png",
  "image/tiff",
  "image/gif",
  "image/bmp",
  "image/x-portable-anymap",
];

function validFileType(file) {
  for (var i = 0; i < fileTypes.length; i++) {
    if (file.type === fileTypes[i]) {
      return true;
    }
  }
  return false;
}

// POST REQUEST TO SERVER: upload image
function loadDoc(target) {
  // cancel default actions
  // event.preventDefault();
  var file = target.files[0];
  if (validFileType(file)) {
    // clear previous images and let the user know that things are moving
    fill_pics("Transforming...");

    var fr = new FileReader();
    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/img_upload");
    xhr.onload = function (e) {
      //The response of the upload
      xhr.responseText;
      if (xhr.status === 200) {
        var data = JSON.parse(xhr.responseText);
        // sometimes "original" is not generated for jpg
        img1.alt = "original";
        img2.src = "data:image/png;base64," + data.images[0];
        img3.src = "data:image/png;base64," + data.images[1];
        img4.src = "data:image/png;base64," + data.images[2];
        img5.src = "data:image/png;base64," + data.images[3];
        img6.src = "data:image/png;base64," + data.images[4];
      } else {
        fill_pics("Is image too large?");
        img4.alt = "Image could not be parsed";
        img5.alt = "Error message ->";
        img6.alt = xhr.responseText;
      }
    };
    fr.onloadend = function () {
      let original = fr.result;
      if (file.type === "image/jpg") {
        original =
          "data:image/jpg;base64," +
          original.slice(original.indexOf("base64") + 7, -1);
      }
      img1.src = original;
    };
    fr.readAsDataURL(file);
    xhr.send(file);
  } else {
    fill_pics("Supported: PNG, JPG, JPEG");

    img1.alt = "format NOT supported";
    img2.alt = `input type: ${file.type}`;
  }
}

function fill_pics(filler = "") {
  img1.src = "";
  img2.src = "";
  img3.src = "";
  img4.src = "";
  img5.src = "";
  img6.src = "";
  if (filler !== "") {
    img6.src = "";
    img1.alt = filler;
    img2.alt = filler;
    img3.alt = filler;
    img4.alt = filler;
    img5.alt = filler;
    img6.alt = filler;
  }
}
