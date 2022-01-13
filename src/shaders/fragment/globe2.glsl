
precision mediump float;

// Data coming from the vertex shader
varying highp vec2 vTextureCoord;
varying highp vec2 vFlipbookCoord;
// The texture unit to use for the color lookup
uniform sampler2D uTexture;
uniform sampler2D uAlpha;

// color lookup table: [[u8; 4] 256]
uniform sampler2D uGradient;

void main() {
    float gamma = 2.2;
    vec4 globe = texture2D(uTexture, vTextureCoord);
    float index = texture2D(uAlpha, vFlipbookCoord).z * 255.0;
    vec4 data = texture2D(uGradient, vec2((index + 0.5) / 256.0, 0.5));
    vec3 diffuseColor = pow(data.rgb, vec3(gamma));
    gl_FragColor = globe * vec4(diffuseColor.rgb, 1);//(diffuseColor.r, diffuseColor.g, diffuseColor.b, data.a);
}