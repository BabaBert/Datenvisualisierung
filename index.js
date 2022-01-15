const rust = import('./pkg/wasm_boilerplate');

const canvas            = document.getElementById('rustCanvas');
const control_bar       = document.getElementById('control_bar');
const slider            = document.getElementById('slider');
// const play_pause_reset  = document.getElementById('play_pause_reset');
// const left              = document.getElementById('left');
// const right             = document.getElementById('right');
// const zoom_in           = document.getElementById('zoom_in');
// const zoom_out          = document.getElementById('zoom_out');

const gl = canvas.getContext("webgl", { antialias: true });

// //TODO: trigger when the video is being paused or resumed playing
// //https://developer.mozilla.org/en-US/docs/Web/Events/Creating_and_triggering_events
// const e_is_playing = CustomEvent('is_playing', {playing: Boolean})
// //TODO: trigger when the video is reseted to the beginning
// const e_reset = Event('reset')

// //TODO: <source>.dipatchEvent(is_playing)
// const time_update = Event('time_update', {"bubbles":true, "cancelable":false})



rust.then(function(m){
    if (!gl) {
        alert('Failed to initialize WebGL');
        return;
    }
    
    const FPS_THROTTLE = 1000.0 / 60.0; // milliseconds / frames
    const Client = new m.Client();
    const initialTime = Date.now();
    let lastDrawTime = -1;// In milliseconds


    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height       = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width        = window.innerWidth;
                canvas.clientWidth  = window.innerWidth;
                canvas.style.width  = window.innerWidth;

                // control_bar.width       = window.innerWidth;
                // control_bar.clientWidth = window.innerWidth;
                control_bar.style.width = window.innerWidth;

                

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            if (window.innerHeight !== slider.width) {
                var slider_width = window.innerWidth - 5 * 85 + "px";

                slider.width        = slider_width;
                slider.clientWidth  = slider_width;
                slider.style.width  = slider_width;
            }

            let elapsedTime = currTime - initialTime;
            Client.update(elapsedTime, window.innerHeight, window.innerWidth);
            Client.render();
        }
    }

    render();
});