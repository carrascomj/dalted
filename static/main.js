var target_brows = document.getElementById("fileb");

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
function loadDoc() {
  console.log("Yes!");
  var input = target_brows,
  file = input.files[0];

  var fd = new FormData();
  fd.append("data", file);
  fd.append("complete", true);
  fd.append("boundary", "huhuhpuche");

  var xhr = new XMLHttpRequest();
  xhr.open("POST", "/img_upload");
  xhr.send(fd);
  xhr.onloadend = function(e) {
      //The response of de upload
      xhr.responseText;
      if(callback) {
        callback();
  }
  }
}
