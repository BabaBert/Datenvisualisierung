const rust = import('./pkg/wasm_boilerplate');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext("webgl", { antialias: true });

// //TODO: trigger when the video is being paused or resumed playing
// //https://developer.mozilla.org/en-US/docs/Web/Events/Creating_and_triggering_events
// const e_is_playing = CustomEvent('is_playing', {playing: Boolean})
// //TODO: trigger when the video is reseted to the beginning
// const e_reset = Event('reset')

// //TODO: <source>.dipatchEvent(is_playing)
// const time_update = Event('time_update', {"bubbles":true, "cancelable":false})



rust.then(m => {
    if (!gl) {
        alert('Failed to initialize WebGL');
        return;
    }
    
    const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
    const VIDEO_FRAME_RATE = 1000. / 24.;
    const Client = new m.Client();
    const initialTime = Date.now();
    let lastDrawTime = -1;// In milliseconds

    const CUSTOM_EVENTS = new m.CustomEvents();
    //const PAUSE_EVENT = CUSTOM_EVENTS.get_pause();
    //const RESET_EVENT = CUSTOM_EVENTS.get_reset();

    //TODO: bind PAUSE_EVENT, RESET_EVENT


    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currTime - initialTime;
            Client.update(elapsedTime, window.innerHeight, window.innerWidth);
            Client.render();
        }

        if(currTime >= lastDrawTime + VIDEO_FRAME_RATE){
            
        }
    }

    render();
});