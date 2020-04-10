var target_brows = document.getElementById("fileb");
var target_drag = document.getElementById("box");
var img1 = document.getElementById("img1");
var img2 = document.getElementById("img2");
var img3 = document.getElementById("img3");
var img4 = document.getElementById("img4");
var img5 = document.getElementById("img5");
var img6 = document.getElementById("img6");

// CLICK SUBMIT
target_brows.addEventListener("change", function(event) {
  event.preventDefault();
  loadDoc(target_brows);
});

// DRAG AND DROP
document.body.addEventListener("dragover", function(event) {
  event.preventDefault();
});
document.body.addEventListener("drop", function(event) {
  event.preventDefault();
});
target_drag.addEventListener(
  "dragover",
  function(event) {
    // prevent default on box and change color
    event.preventDefault();
    target_drag.style.backgroundColor = "#3bb477";
  },
  false
);
target_drag.addEventListener(
  "drop",
  function(event) {
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
  "image/pjpeg",
  "image/png",
  "image/svg"
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
    // clear previous images...
    img1.src = "";
    img2.src = "";
    img3.src = "";
    img4.src = "";
    img5.src = "";
    img6.src = "";
    // ...and let the user know that things are moving
    img1.alt = "Transforming...";
    img2.alt = "Transforming...";
    img3.alt = "Transforming...";
    img4.alt = "Transforming...";
    img5.alt = "Transforming...";
    img6.alt = "Transforming...";
    var fr = new FileReader();
    fr.onloadend = function() {
      var xhr = new XMLHttpRequest();
      xhr.open("POST", "/img_upload");
      xhr.onload = function(e) {
        //The response of the upload
        xhr.responseText;
        if (xhr.status === 200) {
          var data = JSON.parse(xhr.responseText);
          let original = fr.result;
          console.log(original);
          if (file.type === "image/jpeg") {
            original =
              "data:image/png;base64," +
              original.slice(original.indexOf("base64") + 7, -1);
          }
          img1.src = original;
          img2.src = "data:image/png;base64," + data.images[0];
          img3.src = "data:image/png;base64," + data.images[1];
          img4.src = "data:image/png;base64," + data.images[2];
          img5.src = "data:image/png;base64," + data.images[3];
          img6.src = "data:image/png;base64," + data.images[4];
        } else {
          img1.alt = "Image could not be parsed";
          img2.alt = "Image could not be parsed";
          img3.alt = "Image could not be parsed";
          img4.alt = "Supported: PNG, JPG, JPEG";
          img5.alt = "Supported: PNG, JPG, JPEG";
          img6.alt = "Supported: PNG, JPG, JPEG";
          console.log(xhr.responseText);
        }
      };

      xhr.send(
        JSON.stringify({
          file_type: file.type,
          image: fr.result.slice(fr.result.indexOf("base64") + 7, -1),
          // "image": fr.result,
          message: "Apples"
        })
      );
    };
    fr.readAsDataURL(file);
  }
}
