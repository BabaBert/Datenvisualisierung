//Rot zu Blau

var temp = document.getElementById("temperaturCanvas");
var tempx = temp.getContext("2d");

var grd = tempx.createLinearGradient(0,0,0,256);
grd.addColorStop(0,"red");
grd.addColorStop(.5,"white");
grd.addColorStop(1, "blue");
tempx.fillStyle = grd;
tempx.fillRect(0,0,100,256);


var slider = document.getElementById("output");
var output = document.getElementById("year");

output.innerHTML = convertYears(slider.value);

// slider.oninput = function(){
//     output.innerHTML = convertYears(this.value);
// }

function convertYears(input){
    var months = ['Januar', 'Februar', 'März', 'April', 'Mai', 'Juni', 'Juli', 'August', 'September', 'Oktober', 'November', 'Dezember'];

    var temp = parseInt(input/12);
    var year = temp+1880;

    var month = months[input%12];


    return month + " " + year;
}

//Schwarz zu Weiß
/*
var temp = document.getElementById("temperaturCanvas");
var tempx = temp.getContext("2d");

var grd = tempx.createLinearGradient(0,0,0,256);
grd.addColorStop(0,"black");
grd.addColorStop(1, "white");
tempx.fillStyle = grd;
tempx.fillRect(0,0,100,256);
*/