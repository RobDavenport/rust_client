extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn alert(s: &str);
}

#[wasm_bindgen]
pub struct RustClient {
  gl: WebGlRenderingContext,
  program_color_2d: programs::Color2D,
}

#[wasm_bindgen]
impl RustClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> RustClient {
    log("initialize client");
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();

    RustClient {
      program_color_2d: programs::Color2D::new(&gl),
      gl,
    }
  }

  pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
    app_state::update_dynamic_data(time, width, height);
    Ok(())
  }

  pub fn draw(&self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    let curr_state = app_state::get_current_state();

    self.program_color_2d.render(
      &self.gl,
      0.,
      250.,
      0.,
      250.,
      curr_state.canvas_width,
      curr_state.canvas_height,
    )
  }
}