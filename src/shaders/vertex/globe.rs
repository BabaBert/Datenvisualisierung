pub const SHADER: &str = r#"
    
    attribute vec4 aVertexPosition;
    attribute vec2 aTextureCoord;
    attribute vec2 aFlipbookCoord;
    uniform mat4 uProjectionMatrix;
    varying highp vec2 vTextureCoord;
    varying highp vec2 vFlipbookCoord;

    void main(void) {
      gl_Position = uProjectionMatrix * aVertexPosition;
      vTextureCoord = aTextureCoord;
      vFlipbookCoord = aFlipbookCoord;
    }
"#;