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
  program_color_2d_gradient: programs::Color2DGradient,
  colors: [f32; 16],
  time: f32
}

#[wasm_bindgen]
impl RustClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> RustClient {
    log("Initializing Rust Client...");
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();

    let colors: [f32; 16] = [
      1., 0., 0., 1.,
      0., 1., 0., 1.,
      0., 0., 1., 1.,
      0., 1., 1., 1.,
    ];

    RustClient {
      program_color_2d: programs::Color2D::new(&gl),
      program_color_2d_gradient: programs::Color2DGradient::new(&gl),
      gl,
      colors,
      time: 0.
    }
  }

  fn update_colors(&mut self, time: f32) {
    self.time = self.time + (time * 0.25);
    for i in 0..4 {
      self.colors[(i * 4)] = (self.time * (i + 1) as f32).sin().abs();
      self.colors[(i * 4) + 1] = ((self.time * 1.15) * (i + 1) as f32).sin().abs();
      self.colors[(i * 4) + 2] = ((self.time * 1.3) * (i + 1) as f32).sin().abs();
    }
  }

  pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
    app_state::update_dynamic_data(time, width, height);
    self.update_colors(time);
    Ok(())
  }

  pub fn draw(&self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    let curr_state = app_state::get_current_state();

    self.program_color_2d.draw(
      &self.gl,
      0.,
      250.,
      0.,
      250.,
      curr_state.canvas_width,
      curr_state.canvas_height,
    );

    self.program_color_2d_gradient.draw(
      &self.gl,
      250.,
      750.,
      250.,
      750.,
      curr_state.canvas_width,
      curr_state.canvas_height,
      self.colors
    );
  }
}