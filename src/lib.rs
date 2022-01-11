extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{
    *,
    WebGlRenderingContext as GL
};
use js_sys::Promise;

mod programs;
mod shaders;
mod app_state;
mod constants;
use wasm_bindgen_futures::*;

mod gl_setup{
    use wasm_bindgen::{
        JsCast,
        JsValue,
    };
    use {
        web_sys::*,
        WebGlRenderingContext as GL
    };
    
    pub fn initialize_webgl_context() -> Result<GL, JsValue>{
        use event_listener::*;
        use web_sys::*;
    
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("rustCanvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;
    
    
        attach_mouse_scroll_handler(&canvas)?;
        attach_mouse_down_handler(&canvas)?;
        attach_mouse_up_handler(&canvas)?;
        attach_mouse_move_handler(&canvas)?;
    
    
        gl.clear_color(0., 0.0, 0.0, 1.0); //RGBA
        gl.clear_depth(1.);
        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::CULL_FACE);
        gl.depth_func(GL::LESS); 
    
        Ok(gl)
    }
    
    mod event_listener{
        use wasm_bindgen::{
            JsCast,
            JsValue,
            prelude::*,
        };
        use web_sys::*;
    
        pub fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            let listener = move |event: web_sys::WheelEvent| {
                //handler
                super::super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true);
            };
            //create listener on heap
            let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref())?;
            //create memory leak on purpose
            //listener is requiered for the duration of the program running
            listener.forget();
        
            Ok(())
        }
        
        pub fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            let listener = move |event: web_sys::WheelEvent| {
                //handler
                super::super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, false);
            };
        
            let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", listener.as_ref().unchecked_ref())?;
            listener.forget();
        
            Ok(())
        }
        
        pub fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            let listener = move |event: web_sys::WheelEvent| {
                super::super::app_state::update_mouse_position(event.client_x() as f32, event.client_y() as f32);
            };
        
            let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousemove", listener.as_ref().unchecked_ref())?;
            listener.forget();
        
            Ok(())
        }
    
        pub fn attach_mouse_scroll_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            let listener = move |event: web_sys::WheelEvent| {
                super::super::app_state::update_mouse_scroll(event.delta_y());
            };
    
            let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousewheel", listener.as_ref().unchecked_ref())?;
            listener.forget();
    
            Ok(())
        }
    
        pub fn attach_video_pause_handler(target: &EventTarget) -> Result<(), JsValue> {
            let listener = move |custom_event: web_sys::CustomEvent| {
                match custom_event.detail().as_bool().unwrap(){ 
                    true => super::super::app_state::update_video_pause(true), 
                    false => super::super::app_state::update_video_pause(false)
                }
            };
    
            let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
            target.add_event_listener_with_callback("pause", listener.as_ref().unchecked_ref())?;
            listener.forget();
            Ok(())
        }
    
        pub fn attach_video_reset_handler(target: &EventTarget) -> Result<(), JsValue> {
            let listener = move |_event: web_sys::Event| {
                super::super::app_state::reset_video();
            };
    
            let listener = Closure::wrap(Box::new(listener) as Box<dyn FnMut(_)>);
            target.add_event_listener_with_callback("pause", listener.as_ref().unchecked_ref())?;
            listener.forget();
            Ok(())
        }
        
    }
}

#[macro_use] extern crate lazy_static;

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f32;
}


#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub struct CustomEvents{
    e_video_pause: CustomEvent,
    e_video_reset: Event,
}

#[wasm_bindgen]
impl CustomEvents{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self{
        //TODO: detail for pause
        Self{
            e_video_pause: CustomEvent::new("video_pause").unwrap(),
            e_video_reset: Event::new("video_reset").unwrap(),
        }
    }
    pub fn get_pause(self) -> CustomEvent{
        self.e_video_pause
    }
    pub fn get_reset(self) -> Event{
        self.e_video_reset
    }
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
        let gl = gl_setup::initialize_webgl_context().unwrap();

                Self{
                    program_globe: programs::Globe::new(&gl),
                    gl: gl,
                }.try_into().unwrap()
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue>{
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self){
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT); 

        let curr_state = app_state::get_curr_state();

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
            curr_state.time,
            curr_state.mouse_scroll,
            //&common_funcs::matrixes::get_updated_3d_y_values(curr_state.time),
        );
    }
}



