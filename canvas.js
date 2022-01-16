//Rot zu Blau

var temp = document.getElementById("temperaturCanvas");
var tempx = temp.getContext("2d");

var grd = tempx.createLinearGradient(0,0,0,256);
grd.addColorStop(0,"red");
grd.addColorStop(.5,"white");
grd.addColorStop(1, "blue");
tempx.fillStyle = grd;
tempx.fillRect(0,0,100,256);

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