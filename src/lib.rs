extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{
    *,
    WebGlRenderingContext as GL
};
use wasm_bindgen::{
    JsCast,
    JsValue,
};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc; 
use std::sync::Mutex;

//use js_sys::Promise;

mod programs;
mod shaders;
mod constants;
//use wasm_bindgen_futures::*;

#[macro_use] extern crate lazy_static;

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

mod app_state{
    
    use std::sync::Arc;     //creates mutable *references* to the data
    use std::sync::Mutex;   //creates a lock around data so only one owner can access it at a time

    //severeal readonly references to the app_state
    lazy_static! {
        pub static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
        pub static ref INTERFACE: Mutex<Interface> = Mutex::new(Interface::new());
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
        pub last: f64,
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
                last: js_sys::Date::now(),
                timestamp: 0,
                pause: true
            }
        }
    }

    pub struct Interface{
        pub pause: bool,
        pub timestamp: usize,
        pub last: f64,
        pub zoom: f32,
    }

    impl Interface{
        fn new() -> Self {
            Self{
                pause: true,
                timestamp: 0,
                last: js_sys::Date::now(),
                zoom: 1.,
            }
        }
    }
}

mod event_listener{
    use wasm_bindgen::{
        JsCast,
        JsValue,
        prelude::*,
    };
    use web_sys::*;
    use std::sync::Arc;
    use super::app_state::*;
    use std::sync::mpsc::{Sender, Receiver};


    pub fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let listener = move |event: web_sys::WheelEvent| {
            //handler
            // super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true);
            let x = event.client_x() as f32;
            let y = event.client_y() as f32;

            let mut data = APP_STATE.lock().unwrap();
            *data = Arc::new(AppState {
                mouse_down: true,
                mouse_x: x,
                mouse_y: data.canvas_height - y,
                ..*data.clone()
            });
        };
        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref())?;
        listener.forget();
    
        Ok(())
    }
    
    pub fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let listener = move |event: web_sys::WheelEvent| {
            //handler
            // super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, false);
            let x = event.client_x() as f32;
            let y = event.client_y() as f32;

            let mut data = APP_STATE.lock().unwrap();
            *data = Arc::new(AppState {
                mouse_down: false,
                mouse_x: x,
                mouse_y: data.canvas_height - y,
                ..*data.clone()
            });
        };
        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", listener.as_ref().unchecked_ref())?;
        listener.forget();
    
        Ok(())
    }
    
    pub fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let listener = move |event: web_sys::WheelEvent| {
            // super::app_state::update_mouse_position(event.client_x() as f32, event.client_y() as f32);
            use std::f32::*;
            let x = event.client_x() as f32;
            let y = event.client_y() as f32;


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
                rotation_x_axis: f32::max(f32::min(data.rotation_x_axis + rotation_x_delta, 1.5), -1.5),  //globe can only be roated 90° upwards or downwards
                rotation_y_axis: data.rotation_y_axis - rotation_y_delta,
                ..*data.clone()
            });
        };
        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", listener.as_ref().unchecked_ref())?;
        listener.forget();
    
        Ok(())
    }

    //Todo:
    // pub fn attach_mouse_scroll_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    //     let listener = move |event: web_sys::WheelEvent| {
    //         super::app_state::update_mouse_scroll(event.delta_y());
    //     };

    //     let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
    //     canvas.add_event_listener_with_callback("mousewheel", listener.as_ref().unchecked_ref())?;
    //     listener.forget();

    //     Ok(())
    // }

    pub fn attach_zoom_in_handler(button: &HtmlButtonElement) -> Result<(), JsValue> {

        let listener = Closure::wrap(Box::new(move ||{
            INTERFACE.lock().unwrap().zoom *= 1.2;
            super::log(INTERFACE.lock().unwrap().zoom.to_string().as_str());
        }) as Box<dyn Fn()>);
        button.set_onclick(Some(listener.as_ref().unchecked_ref()));
        listener.forget();
        Ok(())
    }

    pub fn attach_zoom_out_handler(button: &HtmlButtonElement) -> Result<(), JsValue> {
        let listener = Closure::wrap(Box::new(move ||{
            INTERFACE.lock().unwrap().zoom /= 1.2;
            super::log(INTERFACE.lock().unwrap().zoom.to_string().as_str());
        }) as Box<dyn Fn()>);
        button.set_onclick(Some(listener.as_ref().unchecked_ref()));
        listener.forget();
        Ok(())
    }

    pub fn attach_video_pause_handler(button: &HtmlButtonElement) -> Result<(), JsValue> {

        let listener = move || {
            if INTERFACE.lock().unwrap().pause {
                INTERFACE.lock().unwrap().pause = false; 
            }
            else{
                INTERFACE.lock().unwrap().pause = true; 
            }
        };

        let listener = Closure::wrap(Box::new(listener) as Box<dyn Fn()>);
        button.set_onclick(Some(listener.as_ref().unchecked_ref()));
        // button.add_event_listener_with_callback("pause", listener.as_ref().unchecked_ref())?;
        listener.forget();
        Ok(())
    }

    pub fn attach_video_skip_right_handler(button: &HtmlButtonElement) -> Result<(), JsValue> {

        let listener = Closure::wrap(Box::new(move||{
            if INTERFACE.lock().unwrap().timestamp > (1703 - 12) {
                INTERFACE.lock().unwrap().timestamp = 1703;
            }
            else{
                INTERFACE.lock().unwrap().timestamp += 12;
            }
        }) as Box<dyn Fn()>);
    
        button.set_onclick(Some(listener.as_ref().unchecked_ref()));
        listener.forget();
        Ok(())
    }

    pub fn attach_video_skip_left_handler(button: &HtmlButtonElement) -> Result<(), JsValue> {

        let listener = Closure::wrap(Box::new(move||{
            if INTERFACE.lock().unwrap().timestamp < 12 {
                INTERFACE.lock().unwrap().timestamp = 0;
            }
            else{
                INTERFACE.lock().unwrap().timestamp -= 12;
            }
        }) as Box<dyn Fn()>);
    
        button.set_onclick(Some(listener.as_ref().unchecked_ref()));
        listener.forget();
        Ok(())
    }

    pub fn attach_output_handler(slider: &HtmlInputElement) -> Result<(), JsValue> {

        let s = slider.clone();
        let listener = move |event: web_sys::MouseEvent| {
            let x = event.client_x();
            let offset_left = s.offset_left();
            super::log(s.offset_left().to_string().as_str());
            super::log(s.offset_width().to_string().as_str());
            super::log(s.width().to_string().as_str());
            let client_width: i32 = 0;
            let width = s.offset_width();
            let normalized = (x - offset_left - 15)/ s.offset_width() * 1704;
            let width = client_width/width;
            INTERFACE.lock().unwrap().timestamp = (x - offset_left - 10)as usize;
        };
        let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
        slider.add_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref()).unwrap();
        listener.forget();
        Ok(())
    }

    pub fn attach_input_handler(slider: &HtmlInputElement) -> Result<(), JsValue> {
        let s = slider.clone();
        let listener = Closure::wrap(Box::new(move || {
            INTERFACE.lock().unwrap().timestamp = s.value_as_number() as usize;
        }) as Box<dyn Fn()>);
        slider.set_onclick(Some(listener.as_ref().unchecked_ref()));
        listener.forget();
        Ok(())
    }
    
}

pub fn init_webgl_context() -> Result<GL, JsValue>{
    use web_sys::*;
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0., 0.0, 0.0, 1.0); //RGBA
    gl.clear_depth(1.);
    gl.enable(GL::DEPTH_TEST);
    gl.enable(GL::CULL_FACE);
    gl.depth_func(GL::LESS); 

    Ok(gl)
}

fn init_events() -> Result<(), JsValue>{

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let play_btn = document.get_element_by_id("play_pause_reset").unwrap();
    let btn_next = document.get_element_by_id("btn_right").unwrap();
    let btn_prev = document.get_element_by_id("btn_left").unwrap();
    // let output   = document.get_element_by_id("output").unwrap();
    let input    = document.get_element_by_id("input").unwrap();
    let zoom_in  = document.get_element_by_id("zoom_in").unwrap();
    let zoom_out = document.get_element_by_id("zoom_out").unwrap();


    // Todo: attach_mouse_scroll_handler(&canvas)?;
    event_listener::attach_mouse_down_handler(&canvas)?;
    event_listener::attach_mouse_up_handler(&canvas)?;
    event_listener::attach_mouse_move_handler(&canvas)?;
    event_listener::attach_video_pause_handler(&play_btn.dyn_into().unwrap())?;
    event_listener::attach_video_skip_right_handler(&btn_next.dyn_into().unwrap())?;
    event_listener::attach_video_skip_left_handler(&btn_prev.dyn_into().unwrap())?;
    // event_listener::attach_output_handler(&slider.dyn_into().unwrap())?;
    event_listener::attach_input_handler(&input.dyn_into().unwrap())?;
    event_listener::attach_zoom_in_handler(&zoom_in.dyn_into().unwrap())?;
    event_listener::attach_zoom_out_handler(&zoom_out.dyn_into().unwrap())?;

    Ok(())
}

//all the data that is stored on the user client, i.e. the browser
#[wasm_bindgen]
pub struct Client {
    gl: GL,
    program_globe: programs::Globe::<3>,
}

#[wasm_bindgen]
impl Client{
    
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self{

        console_error_panic_hook::set_once();
        let gl = init_webgl_context().unwrap();
        init_events().unwrap();

                Self{
                    program_globe: programs::Globe::new(&gl),
                    gl: gl,
                }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue>{
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self, range: &web_sys::HtmlInputElement){
        use js_sys::Date;
        use app_state::INTERFACE;

        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT); 

        let curr_state = app_state::get_curr_state();

        if INTERFACE.lock().unwrap().pause == false{
            let now = Date::now();
            if now > INTERFACE.lock().unwrap().last + (1000. / 12.) {
                INTERFACE.lock().unwrap().timestamp += 1;
                INTERFACE.lock().unwrap().last = now;
            }
        }

        range.set_value_as_number(INTERFACE.lock().unwrap().timestamp as f64);

        let int = INTERFACE.lock().unwrap();

        self.program_globe.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
            curr_state.rotation_x_axis,
            curr_state.rotation_y_axis,
            int.timestamp,
            int.zoom,
            //INTERFACE.lock().unwrap().zoom,
            // curr_state.mouse_scroll,
            //&common_funcs::matrixes::get_updated_3d_y_values(curr_state.time),
        );
    }
}



