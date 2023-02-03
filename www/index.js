import * as gs from 'gunslinger';

var x = 0;
var y = 0;

window.addEventListener("mousemove", function (e) {
    x += e.movementX;
    y += e.movementY;
    //console.log(x, y);
});

window.addEventListener("click", function (e) {
    x = e.clientX;
    y = e.clientY;
    //if (!this.document.pointerLockElement) {
    //    this.document.body.requestPointerLock({
    //        unadjustedMovement: true,
    //    });
    //}
});

// gameloop frame
function renderframe() {
    gs.frame();
    requestAnimationFrame(renderframe);
}
// start loop
requestAnimationFrame(renderframe);
