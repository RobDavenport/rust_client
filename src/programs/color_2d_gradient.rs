use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_funcs as cf;

pub struct Color2DGradient {
  program: WebGlProgram,
  color_buffer: WebGlBuffer,
  index_count: i32,
  gl_vertex_buffer: WebGlBuffer,
  u_transform: WebGlUniformLocation
}

impl Color2DGradient {
  pub fn new(gl: &WebGlRenderingContext) -> Color2DGradient {
    let program = cf::link_program(
      &gl, 
      super::super::shaders::vertex::color_2d_gradient::SHADER, 
      super::super::shaders::fragment::color_2d_gradient::SHADER
    ).unwrap();

    let vertices: [f32; 8] = [
      0., 1.,
      0., 0.,
      1., 1.,
      1., 0.
    ];
    let vertices_ptr_index = vertices.as_ptr() as u32 / 4;
    let vertex_wasm_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let js_vertex_array = js_sys::Float32Array::new(&vertex_wasm_buffer).subarray(
      vertices_ptr_index,
      vertices_ptr_index + vertices.len() as u32
    );
    let gl_vertex_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&gl_vertex_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_vertex_array, GL::STATIC_DRAW);

    let index_array: [u16; 6] = [0, 1, 2, 2, 1, 3];
    let index_memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let index_ptr_index = index_array.as_ptr() as u32 / 2;
    let js_index_array = js_sys::Uint16Array::new(&index_memory_buffer).subarray(
      index_ptr_index,
      index_ptr_index + index_array.len() as u32
    );
    let gl_index_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&gl_index_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &js_index_array, GL::STATIC_DRAW);

    Color2DGradient {
      color_buffer: gl.create_buffer().ok_or("failed to create buffer").unwrap(),
      index_count: index_array.len() as i32,
      u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
      gl_vertex_buffer,
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
    canvas_height: f32,
    colors: [f32; 16]
  ) {
    gl.use_program(Some(&self.program));

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.gl_vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
    gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(1);

    // let colors: [f32; 16] = [
    //   1., 0., 0., 1.,
    //   0., 1., 0., 1.,
    //   0., 0., 1., 1.,
    //   0., 1., 1., 1.,
    // ];

    let colors_wasm_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let colors_ptr_index = colors.as_ptr() as u32 / 4;
    let js_colors_array = js_sys::Float32Array::new(&colors_wasm_buffer)
      .subarray(colors_ptr_index, colors_ptr_index + colors.len() as u32);
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_colors_array, GL::DYNAMIC_DRAW);

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

    gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
  }
}