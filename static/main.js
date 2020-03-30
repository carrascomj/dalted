var target_brows = document.getElementById("fileb");
var img1 = document.getElementById("img1");

target_brows.addEventListener('change', loadDoc);
img1.addEventListener('change', () => {console.log("hall1");});

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
      console.log(fr.result);
      xhr.onload = function(e) {
        //The response of the upload
        xhr.responseText;
        if(xhr.status === 200) {
          console.log("Received stuff");
          var data = JSON.parse(xhr.responseText);
          img1.src = "data:image/png;base64," + data.images[0];
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
