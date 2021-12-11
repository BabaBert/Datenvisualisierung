#![allow(dead_code)]
pub const SHADER: &str = r#" 

  precision mediump float;

  // Data coming from the vertex shader
    varying highp vec2 vTextureCoord;
    varying highp vec2 vFlipbookCoord;

    // The texture unit to use for the color lookup
    uniform sampler2D uTexture;
    uniform sampler2D uAlpha;

    void main() {
      vec4 color0 = texture2D(uAlpha, vTextureCoord);
      vec4 color1 = texture2D(uTexture, vFlipbookCoord);
      gl_FragColor = color0 * color1;
    }
"#;