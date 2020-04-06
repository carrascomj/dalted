var target_brows = document.getElementById("fileb");
var img1 = document.getElementById("img1");
var img2 = document.getElementById("img2");
var img3 = document.getElementById("img3");
var img4 = document.getElementById("img4");
var img5 = document.getElementById("img5");
var img6 = document.getElementById("img6");

target_brows.addEventListener('change', loadDoc);

// check type
var fileTypes = [
  'image/jpeg',
  'image/pjpeg',
  'image/png',
  'image/svg'
]

function validFileType(file) {
    for(var i = 0; i < fileTypes.length; i++) {
        if(file.type === fileTypes[i]) {
            return true;
        }
    }
    return false;
}

// POST REQUEST TO SERVER: upload image
function loadDoc(event) {
  // cancel default actions
  // event.preventDefault();
  var file = target_brows.files[0];
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
    fr.onloadend = function () {
      var xhr = new XMLHttpRequest();
      xhr.open("POST", "/img_upload");
      xhr.onload = function(e) {
        //The response of the upload
        xhr.responseText;
        if(xhr.status === 200) {
          var data = JSON.parse(xhr.responseText);
          let original = fr.result;
          console.log(original);
          if (file.type === "image/jpeg") {
            original = "data:image/png;base64," + original.slice((original.indexOf("base64")+7),-1)
          }
          img1.src = original;
          img2.src = "data:image/png;base64," + data.images[0];
          img3.src = "data:image/png;base64," + data.images[1];
          img4.src = "data:image/png;base64," + data.images[2];
          img5.src = "data:image/png;base64," + data.images[3];
          img6.src = "data:image/png;base64," + data.images[4];
        } else {
          img1.alt = "Error ocurred";
          img2.alt = "Error ocurred";
          img3.alt = "Error ocurred";
          img4.alt = "Error ocurred";
          img5.alt = "Error ocurred";
          img6.alt = "Error ocurred";
          console.log(xhr.responseText);
        }
      }

      xhr.send(JSON.stringify({
        "file_type": file.type,
        "image": fr.result.slice((fr.result.indexOf("base64")+7),-1),
        // "image": fr.result,
        "message": "Apples",
      }));
    }
    fr.readAsDataURL(file);
  }
}
