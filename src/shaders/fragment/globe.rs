pub const SHADER: &str = r#" 

  // Data coming from the vertex shader
    varying highp vec2 vTextureCoord;

    // The texture unit to use for the color lookup
    uniform sampler2D uSampler;

    void main(void) {
      gl_FragColor = texture2D(uSampler, vTextureCoord);
    }
"#;