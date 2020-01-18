extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct RustClient {

}

#[wasm_bindgen]
impl RustClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    log("initialize client");
    Self {

    }
  }

  pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
    Ok(())
  }

  pub fn draw(&self) {

  }
}