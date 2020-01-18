pub const SHADER: &str = r#"
  precision mediump float;

  uniform vec4 uColor;

  varying lowp vec4 vColor;

  void main() {
    gl_FragColor = vec4(vColor.r, vColor.g, vColor.b, vColor.a);
  }
"#;