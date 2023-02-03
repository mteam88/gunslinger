import * as gs from 'gunslinger';

var x = 0;
var y = 0;

window.addEventListener("mousemove", function (e) {
    x += e.movementX;
    y += e.movementY;
    //console.log(x, y);
});