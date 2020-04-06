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
          console.log("Some error:");
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
