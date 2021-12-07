#![allow(dead_code)]
pub const SHADER: &str = r#"
    
    attribute vec4 aVertexPosition;
    attribute vec2 aTextureCoord;
    uniform mat4 uProjectionMatrix;
    varying highp vec2 vTextureCoord;

    void main(void) {
      gl_Position = uProjectionMatrix * aVertexPosition;
      vTextureCoord = aTextureCoord;
    }
"#;