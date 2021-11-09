extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{
    *,
    WebGlRenderingContext as GL
};

#[macro_use]
extern crate lazy_static;

mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;
mod app_state;

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//all the data that is stored on the user client, i.e. the browser
#[wasm_bindgen]
pub struct Client {
    gl: GL,
    program_color_2d: programs::Color2D,
}

#[wasm_bindgen]
impl Client{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        
        Self{
            program_color_2d: programs::Color2D::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue>{
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self){
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        self.program_color_2d.render(
            &self.gl,
            curr_state.control_bottom,    //bottom
            curr_state.control_top,    //top
            curr_state.control_left,    //right
            curr_state.control_right,    //left
            curr_state.canvas_height,    //canvas_height
            curr_state.canvas_width,    //canvas_width
        );
    }
}