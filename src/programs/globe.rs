use super::{common_funcs::cf, common_funcs::{matrixes::*, vec_to_array::*}};
use super::common_funcs::geomertry_generator::*;
use web_sys::{
    WebGlRenderingContext as GL,
    *
};
use std::sync::Mutex;

const EARTH:  &str = "../../data/image/earth.jpg";
const TEST:   &str = "../../data/image/test.png";
const EARTH2: &str = "../../data/image/PathfinderMap_hires.jpg";
const DATA: &str = "../../data/image/data.png";
const FLIPBOOK: &str = "../../data/image/houdinisheet.jpg";
const ALPHA: &str = "../../data/image/43dfa829f98aa1da4700f0c65ce0d10e.jpg";
const SVG: &str = "../../data/image/outline.png";

const SUBDIVIONS: usize = 4;
const VERTICES: usize = size_v(SUBDIVIONS);
const INDICES: usize = size_i(SUBDIVIONS);
const VERTICES_S: usize = VERTICES * 3;
const INDICES_S: usize = INDICES * 3;

// static mut last_time: f32 = 0.;
// static mut timestamp: usize = 0;

//Modules
pub struct Globe<const T: usize> {
    pub program: WebGlProgram,                      //Program pointer
    pub indices_buffer: WebGlBuffer,
    pub position_buffer: WebGlBuffer,
    pub texture_coord_buffer: WebGlBuffer,
    //pub flipbook_coord_buffer: WebGlBuffer,
    pub texture_coord_array: [[f32; 2]; VERTICES],

    pub index_count: i32,

    pub u_projection_matrix: WebGlUniformLocation,
    pub u_samplers: [WebGlUniformLocation; T],

    pub textures: [WebGlTexture; T],
}

impl Globe<3> {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        use super::common_funcs::textures::*;
        use js_sys::*;

        

        let program = cf::link_program(
            &gl,
            &super::super::shaders::vertex::globe::SHADER,
            &super::super::shaders::fragment::globe2::SHADER,
        ).unwrap();
        
        let globe: IcoSphere<VERTICES,  INDICES> = IcoSphere::new(1., SUBDIVIONS);

        //generate arrays for sphere
        let mesh = globe.gen_mesh::<VERTICES_S, INDICES_S>();
        let raw_uv_map = globe.gen_uv_map::<VERTICES_S>();
        //let flip_map = flipbook_texture_map::<12, 142, VERTICES>(1690, &raw_uv_map);

        //create textures
        let texture  = create_texture(gl, SVG);
        let texture2 = create_alpha(gl, DATA);
        let gradient: &mut [u8; 4*256] = unsafe{std::mem::transmute(create_gradient(255).as_ptr())};
        let gradient = create_texture_from_u8(gl, gradient);

        //Vertices Buffer
        let vertices_array: Float32Array = ArrayToJS::new(&mesh.0);
        let position_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        //Indeces Buffer
        let indices_array: Uint16Array = ArrayToJS::new(&mesh.1);
        let indices_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);  
        
        //Texture Coordinates Buffer
        let uv_map: &[f32; 2*VERTICES] = unsafe{std::mem::transmute(raw_uv_map.as_ptr())};
        let tex_coord_buffer = texture_coord_buffer(gl, uv_map);

        //Flipbook Coordinates Buffer
        // let uv_map: &[f32; 2*VERTICES] = unsafe{std::mem::transmute(flip_map.as_ptr())};
        // let flip_coord_buffer = texture_coord_buffer(gl, uv_map);

    
        Self {
            u_projection_matrix: gl.get_uniform_location(&program, "uProjectionMatrix").unwrap(),
            u_samplers: 
            [gl.get_uniform_location(&program, "uTexture").unwrap(),
             gl.get_uniform_location(&program, "uAlpha").unwrap(),
             gl.get_uniform_location(&program, "uGradient").unwrap()],
            program: program,
            indices_buffer: indices_buffer,
            index_count: indices_array.length() as i32,
            position_buffer: position_buffer,
            texture_coord_buffer: tex_coord_buffer,
            //flipbook_coord_buffer: flip_coord_buffer,
            texture_coord_array: raw_uv_map,
            textures: [texture, texture2, gradient]
        }
        
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        //for control
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
        rotation_angle_x_axis: f32,
        rotation_angle_y_axis: f32,
        timestamp: usize,
        zoom: f32,
    ) {
        use super::common_funcs::textures::*;
        

        //transformation(rotation) @common_funcs
        let projection_matrix = 
        get_3d_matrices(
            bottom,
            top,
            left,
            right,
            canvas_height,
            canvas_width,
            rotation_angle_x_axis,
            rotation_angle_y_axis,
            zoom,
        );

        gl.use_program(Some(&self.program));


        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_projection_matrix),
            false,
            &projection_matrix.projection,
        );

        let a_vertex_position = gl.get_attrib_location(&self.program, "aVertexPosition");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        gl.vertex_attrib_pointer_with_i32(a_vertex_position as u32, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(a_vertex_position as u32);

        let a_texture_coord = gl.get_attrib_location(&self.program, "aTextureCoord");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.texture_coord_buffer));
        gl.vertex_attrib_pointer_with_i32(a_texture_coord as u32, 2, GL::FLOAT, true, 0, 0);
        gl.enable_vertex_attrib_array(a_texture_coord as u32);

        
        let flip_map = super::common_funcs::geomertry_generator::flipbook_texture_map::<12, 142, VERTICES>(timestamp, &self.texture_coord_array);
        let uv_map: &[f32; 2*VERTICES] = unsafe {std::mem::transmute(flip_map.as_ptr())};
        let a_texture_coord = gl.get_attrib_location(&self.program, "aFlipbookCoord");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&texture_coord_buffer(gl, uv_map)));
        gl.vertex_attrib_pointer_with_i32(a_texture_coord as u32, 2, GL::FLOAT, true, 0, 0);
        gl.enable_vertex_attrib_array(a_texture_coord as u32);
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));
        super::common_funcs::textures::active_texture(gl, &self.textures[0], 0, &self.u_samplers[0]);
        super::common_funcs::textures::active_texture(gl, &self.textures[1], 1, &self.u_samplers[1]);
        super::common_funcs::textures::active_texture(gl, &self.textures[2], 2, &self.u_samplers[2]);
        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
        // }
    }

}


