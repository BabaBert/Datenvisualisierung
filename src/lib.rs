extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {

}

#[wasm_bindgen]
impl Client{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self{

        }
    }

    pub fn update(&mut self, _time: f32, _height: f32) -> Result<(), JsValue>{
        Ok(())
    }

    pub fn render(&self){

    }
}