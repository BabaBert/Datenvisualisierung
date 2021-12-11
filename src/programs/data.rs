use web_sys::{
    WebGlRenderingContext as GL,
    *
};

pub struct Globe {
    pub program: WebGlProgram,                      //Program pointer
    pub indices_buffer: WebGlBuffer,
    pub position_buffer: WebGlBuffer,
    pub texture_coord_buffer: WebGlBuffer,

    pub index_count: i32,

    pub u_projection_matrix: WebGlUniformLocation,
    pub u_sampler: WebGlUniformLocation,

    pub texture: WebGlTexture,
}