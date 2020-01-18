use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_funcs as cf;

pub struct Color2D {
  program: WebGlProgram,
  vertex_buffer_length: usize,
  gl_buffer: WebGlBuffer,
  u_color: WebGlUniformLocation,
  u_transform: WebGlUniformLocation
}

impl Color2D {
  pub fn new(gl: &WebGlRenderingContext) -> Color2D {
    let program = cf::link_program(
      &gl, 
      super::super::shaders::vertex::color_2d::SHADER, 
      super::super::shaders::fragment::color_2d::SHADER
    ).unwrap();

    let vertices: [f32; 12] = [
      0., 1.,
      0., 0.,
      1., 1.,
      1., 1.,
      0., 0.,
      1., 0.
    ];

    let vertices_ptr_index = vertices.as_ptr() as u32 / 4;

    let wasm_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();

    let js_vertex_array = js_sys::Float32Array::new(&wasm_buffer).subarray(
      vertices_ptr_index,
      vertices_ptr_index + vertices.len() as u32
    );

    let gl_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&gl_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_vertex_array, GL::STATIC_DRAW);

    Color2D {
      u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
      u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
      gl_buffer,
      vertex_buffer_length: vertices.len(),
      program,
    }
  }

  pub fn draw(
    &self,
    gl: &WebGlRenderingContext,
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_width: f32,
    canvas_height: f32
  ) {
    gl.use_program(Some(&self.program));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.gl_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.uniform4f(
      Some(&self.u_color),
      0.,
      0.5,
      0.25,
      1.
    );

    let translation_matrix = cf::translation_matrix(
      2. * left / canvas_width - 1.,
      2. * bottom / canvas_height - 1.,
      0.
    );

    let scale_matrix = cf::scale_matrix(
      2. * (right - left) / canvas_width,
      2. * (top - bottom) / canvas_height,
      0.,
    );

    let transform_matrix = cf::multiply_matrix4(scale_matrix, translation_matrix);
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_matrix);

    gl.draw_arrays(GL::TRIANGLES, 0, (self.vertex_buffer_length / 2) as i32);
  }
}