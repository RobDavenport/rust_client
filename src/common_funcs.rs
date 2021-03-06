use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn link_program(
  gl: &WebGlRenderingContext,
  vert_source: &str,
  frag_source: &str
) -> Result<WebGlProgram, String> {
  let program = gl
    .create_program()
    .ok_or_else(|| String::from("Error creating program"))
    .unwrap();

  let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();
  let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

  gl.attach_shader(&program, &vert_shader);
  gl.attach_shader(&program, &frag_shader);
  gl.link_program(&program);

  if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false) {
      Ok(program)
    } else {
      Err(gl.get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(
  gl: &WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = gl
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Error creating shader"))?;

  gl.shader_source(&shader, source);
  gl.compile_shader(&shader);
  
  if gl.get_shader_parameter(&shader,WebGlRenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false) {
      Ok(shader)
    } else {
      Err(gl.get_shader_info_log(&shader)
      .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
  let mut out = [0.; 16];

  out[0] = 1.;
  out[5] = 1.;
  out[10] = 1.;
  out[15] = 1.;

  out[12] = tx;
  out[13] = ty;
  out[14] = tz;

  out
}

pub fn scale_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
  let mut out = [0.; 16];

  out[0] = sx;
  out[5] = sy;
  out[10] = sz;
  out[15] = 1.;

  out
}

pub fn multiply_matrix4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
  let mut out = [0.; 16];

  out[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
  out[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
  out[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
  out[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

  out[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
  out[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
  out[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
  out[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

  out[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
  out[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
  out[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
  out[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

  out[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
  out[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
  out[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
  out[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

  out
}