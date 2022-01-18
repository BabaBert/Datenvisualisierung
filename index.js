const rust = import('./pkg/wasm_boilerplate');

const canvas            = document.getElementById('rustCanvas');
const info              = document.getElementById('infoText')

const control_bar       = document.getElementById('control_bar');
// const center            = document.getElementById('center');
const output            = document.getElementById('output');
const input             = document.getElementById('input');
const play_pause_reset  = document.getElementById('play_pause_reset');


const gl = canvas.getContext("webgl", { antialias: true });

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

                // info.translate(window.innerWidth/2, window.innerHeight / 2);

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            if (window.innerHeight !== input.width) {
                var input_width = output_width = window.innerWidth - 5 * 85 + "px";

                input.width        = input_width;
                input.clientWidth  = input_width;
                input.style.width  = input_width;

                output.width        = output_width;
                output.clientWidth  = output_width;
                output.style.width  = output_width;


            }

            let elapsedTime = currTime - initialTime;
            Client.update(elapsedTime, window.innerHeight, window.innerWidth);
            Client.render(output);
        }
    }

    render();
});