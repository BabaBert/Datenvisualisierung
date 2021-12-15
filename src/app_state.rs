use std::sync::Arc;     //creates mutable *references* to the data
use std::sync::Mutex;   //creates a lock around data so only one owner can access it at a time

//severeal readonly references to the app_state
lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {  //canvas size is stored every time -> can be optimized
    let min_height_width = canvas_height.min(canvas_width);
    let display_size = 1. * min_height_width;
    let half_display_size = display_size / 2.;
    let half_canvas_height = canvas_height / 2.;
    let half_canvas_width = canvas_width / 2.;

    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        canvas_height: canvas_height,
        canvas_width: canvas_width,

        control_bottom: half_canvas_height - half_display_size,
        control_top: half_canvas_height + half_display_size,
        control_left: half_canvas_width - half_display_size,
        control_right: half_canvas_width + half_display_size,

        time: time,
        ..*data.clone()
    });
}

pub fn get_curr_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

//AppState is constantly updated with the client's info
pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub control_bottom: f32,
    pub control_top: f32,
    pub control_left: f32,
    pub control_right: f32,
    pub mouse_down: bool,
    pub mouse_scroll: f32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub rotation_x_axis: f32,
    pub rotation_y_axis: f32,
    pub time: f32,
    pub timestamp: usize,
    pub pause: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0., 
            canvas_width: 0.,
            control_bottom: 0.,
            control_top: 0.,
            control_left: 0.,
            control_right: 0.,
            mouse_down: false,
            mouse_scroll: 0.,
            mouse_x: -1.,
            mouse_y: -1.,
            rotation_x_axis: 0.,        //angle
            rotation_y_axis: 0.,
            time: 0.,
            timestamp: 0,
            pause: true
        }
    }
}

//grabs only the requiered information form AppState through the Arc-Mutex pattern
pub fn update_mouse_down(x: f32, y: f32, is_down: bool) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: is_down,
        mouse_x: x,
        mouse_y: data.canvas_height - y,
        ..*data.clone()
    });
}

pub fn update_mouse_position(x: f32, y: f32) {
    use std::f32::*;
    let mut data = APP_STATE.lock().unwrap();
    let inverted_y = data.canvas_height - y;
    let x_delta = x - data.mouse_x;
    let y_delta = inverted_y - data.mouse_y;
    let rotation_x_delta = if data.mouse_down {
        consts::PI * y_delta / data.canvas_height
    } else {
        0.
    };
    let rotation_y_delta = if data.mouse_down {
        consts::PI * x_delta / data.canvas_width
    } else {
        0.
    };

    *data = Arc::new(AppState {
        mouse_x: x,
        mouse_y: inverted_y,
        rotation_x_axis: f32::max(f32::min(data.rotation_x_axis + rotation_x_delta, 1.), -1.),  //globe can only be roated 90° upwards or downwards
        rotation_y_axis: data.rotation_y_axis - rotation_y_delta,
        ..*data.clone()
    });
}

pub fn update_mouse_scroll(mouse_scroll: f64){
    let mut data = APP_STATE.lock().unwrap();
    match mouse_scroll {
        x if x > 0. => {
            *data = Arc::new(AppState {
                mouse_scroll: data.mouse_scroll + 10.,
                ..*data.clone()
            });
        }
        y if y < 0. => {
            *data = Arc::new(AppState {
                mouse_scroll: data.mouse_scroll - 10.,
                ..*data.clone()
            });
        }
        _ => {*data = Arc::new(AppState{..*data.clone()})}
    }
}

pub fn update_video_pause(pause: bool){
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState{
        pause: pause,
        ..*data.clone()
    })
}

pub fn reset_video(){
    
}

pub fn update_time_stamp(){
    let mut data = APP_STATE.lock().unwrap();
    let t = data.timestamp;
    *data = Arc::new(AppState {
        timestamp: t + 1,
        ..*data.clone()
    });
}
