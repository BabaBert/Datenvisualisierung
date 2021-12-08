use super::{common_funcs::cf, common_funcs::{matrixes::*, normals::*}};
use js_sys::WebAssembly;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{
    WebGlRenderingContext as GL,
    *
};
use nalgebra::Vector3;

const EARTH:  &str = "../../data/image/earth.jpg";
const TEST:   &str = "../../data/image/test.png";
const EARTH2: &str = "../../data/image/PathfinderMap_hires.jpg";
const DATA: &str = "../../data/image/data.png";
const FLIPBOOK: &str = "../../data/image/houdinisheet.jpg";

//Modules
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

impl Globe {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        use geomertry_generator::*;
        use super::common_funcs::textures::*;
        use vec_to_array::ArrayToJS;
        use js_sys::*;

        

        let program = cf::link_program(
            &gl,
            &super::super::shaders::vertex::globe::SHADER,
            &super::super::shaders::fragment::globe::SHADER,
        ).unwrap();
        
        const VERTICES: usize = SIZE_V(5);
        const INDICES: usize = SIZE_I(5);
        const VERTICES_S: usize = VERTICES * 3;
        const INDICES_S: usize = INDICES * 2;
        
        let globe: IcoSphere<VERTICES,  INDICES> = IcoSphere::new::<5>(1.);

        //generate arrays for Ico sphere
        let positions_and_indices = globe.gen_mesh::<VERTICES_S, INDICES_S>();
        const Y: usize = 2022 - 1880;
        let uv_map = globe.gen_uv_map::<VERTICES_S>();
        //let uv_map = globe.flipbook_texture_map::<12, 142>(1000);
        let texture = create_texture(gl, EARTH);

        //Vertices Buffer
        let vertices_array: Float32Array = ArrayToJS::new(&positions_and_indices.0);
        let position_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        //Indeces Buffer
        let indices_array: Uint16Array = ArrayToJS::new(&positions_and_indices.1);
        let indices_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);  
        
        //Texture Coordinates Buffer
        let texture_array: Float32Array = ArrayToJS::new(&uv_map);
        let tex_coord_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coord_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &texture_array, GL::STATIC_DRAW);


        Self {
            u_projection_matrix: gl.get_uniform_location(&program, "uProjectionMatrix").unwrap(),
            u_sampler: gl.get_uniform_location(&program, "uSampler").unwrap(),

            program: program,

            indices_buffer: indices_buffer,
            index_count: indices_array.length() as i32,
            position_buffer: position_buffer,
            texture_coord_buffer: tex_coord_buffer,
            texture: texture,
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
        zoom: f32,
    ) {
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

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));

        // Tell WebGL we want to affect texture unit 0
        gl.active_texture(GL::TEXTURE0);
        // Bind the texture to texture unit 0
        gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        // Tell the shader we bound the texture to texture unit 0
        gl.uniform1i(Some(&self.u_sampler), 0);


        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }

}

mod vec_to_array{
    use js_sys::*;
    use wasm_bindgen::JsCast;

    pub trait VecToArray<T>{
        fn new(vec: &Vec<T>) -> Self;
    }

    impl VecToArray<f32> for Float32Array{
        #[inline]
        fn new(vec: &Vec<f32>) -> Self{
            let buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let mem_loc = vec.as_ptr() as u32 / 4;
            let array = Float32Array::new(&buffer).subarray(
                mem_loc,
                mem_loc + vec.len() as u32,
            );
            array
        }
    }

    impl VecToArray<u16> for Uint16Array{
        #[inline]
        fn new(vec: &Vec<u16>) -> Self{
            let buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let mem_loc = vec.as_ptr() as u32 / 2;
            let array = Uint16Array::new(&buffer).subarray(
                mem_loc,
                mem_loc + vec.len() as u32,
            );
            array
        }
    }

    pub trait ArrayToJS<T>{
        fn new<const S: usize>(array: &[T; S]) -> Self;
    }

    impl ArrayToJS<f32> for Float32Array{
        #[inline]
        fn new<const S: usize>(array: &[f32; S]) -> Self{
            let buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let mem_loc = array.as_ptr() as u32 / 4;
            let ret = Float32Array::new(&buffer).subarray(
                mem_loc,
                mem_loc + array.len() as u32,
            );
            ret
        }
    }

    impl ArrayToJS<u16> for Uint16Array{
        fn new<const S: usize>(array: &[u16; S]) -> Self{
            let buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let mem_loc = array.as_ptr() as u32 / 2;
            let ret = Uint16Array::new(&buffer).subarray(
                mem_loc,
                mem_loc + array.len() as u32,
            );
            ret
        }
    }
}

mod geomertry_generator{
    // mostly coppied from https://github.com/Gonkee/Gepe3D/blob/main/Gepe3D/src/Physics/GeometryGenerator.cs

    use std::collections::HashMap;
    use super::super::super::log;
    use super::super::common_funcs::normals::q_rsqrt;

    const CENTRE_POINT: [f32; 3] = [0., 0., 0.];

    //indices for subdivided triangle
    pub const fn SIZE_I(x: usize) -> usize{
        usize::pow(4, x as u32) * 3
    }
    //vertices for subdivided triangle
    pub const fn SIZE_V(x: usize) -> usize{
        (3 * usize::pow(2, x as u32) + usize::pow(2, (2*x) as u32) + 2)/2
    }


    pub struct IcoSphere<const V: usize, const I: usize>{
        vertices: [[f32; 3]; V],    //Vec<Vector3 <f32>>,
        indices:  [[u16; 3]; I],    //Vec<Vector3<u16>>,
        existing_mid_points:HashMap<u32, u16>,
        radius: f32,
    }

    impl Default for IcoSphere<12, 20>{
        // initialized to the starting icosahedron
        fn default() -> Self {
            const SQRT_5:   f32 = 2.23606797749978;
            const PHI:      f32 = (1. + SQRT_5) / 2.;

            Self{
                vertices:[
                    [ -1., PHI,  0.], [  1., PHI,  0.], [ -1.,-PHI,  0.], [  1.,-PHI,  0.], 
                    [  0., -1., PHI], [  0.,  1., PHI], [  0., -1.,-PHI], [  0.,  1.,-PHI],
                    [ PHI,  0., -1.], [ PHI,  0.,  1.], [-PHI,  0., -1.], [-PHI,  0.,  1.]
                ],
                indices:[
                    [ 0, 11,  5], [0,  5,  1], [0,  1,  7], [ 0,  7, 10],
                    [ 0, 10, 11], [1,  5,  9], [5, 11,  4], [11, 10,  2],
                    [10,  7,  6], [7,  1,  8], [3,  9,  4], [ 3,  4,  2],
                    [ 3,  2,  6], [3,  6,  8], [3,  8,  9], [ 4,  9,  5],
                    [ 2,  4, 11], [6,  2, 10], [8,  6,  7], [ 9,  8,  1]
                ],
                existing_mid_points: HashMap::new(),
                radius: 0.
            }
        }
    }

    impl<const V: usize, const I: usize> IcoSphere<V, I>{


        //TODO: check
        pub fn new<const S: usize>(radius: f32) -> Self{

            let mut base = IcoSphere::default();

            let mut ret = Self{
                vertices: [[0.; 3]; V],
                indices:  [[0 ; 3]; I],    
                existing_mid_points: HashMap::new(),
                radius: radius,
            };

            base.vertices.map(|x: [f32; 3]|{
                x.map(q_rsqrt)
            });

            for i in 0..S{

                // every iteration makes a new list of triangles
                for j in 0..base.indices.len(){

                    const X: usize = 0;
                    const Y: usize = 1;
                    const Z: usize = 2;
                    let a = base.gen_mid_point_id(base.indices[j][X], base.indices[j][Y]);
                    let b = base.gen_mid_point_id(base.indices[j][Y], base.indices[j][Z]);
                    let c = base.gen_mid_point_id(base.indices[j][Z], base.indices[j][X]);
                    // replace triangle with 4 new triangles
                    ret.indices[i+j+0] = [base.indices[j][X], a, c];
                    ret.indices[i+j+1] = [base.indices[j][Y], b, a];
                    ret.indices[i+j+2] = [base.indices[j][Z], c, b];
                    ret.indices[i+j+3] = [                 a, b, c];
                }
                
            }

            ret.vertices = ret.vertices.map(|x|{x.map(|x|{x * radius})});
            ret
        }

        fn gen_mid_point_id(&mut self, v1: u16, v2: u16) -> u16{
            use nalgebra::{min, max};

            // check if midpoint has already been generated by another triangle
            let unique_combination: u32 = ((min(v1, v2) as u32) << 16) + max(v1, v2) as u32;
            
            match self.existing_mid_points.contains_key(&unique_combination) {
                true => *self.existing_mid_points.get(&unique_combination).unwrap(),
                false => {
                    let mid_point: [f32; 3] = [
                        self.vertices[v1 as usize][0] + self.vertices[v2 as usize][0],
                        self.vertices[v1 as usize][1] + self.vertices[v2 as usize][1],
                        self.vertices[v1 as usize][2] + self.vertices[v2 as usize][2]
                    ].map(|x|{q_rsqrt(x/2.)});
                    let id: u16 = self.vertices.len() as u16;
                    self.existing_mid_points.insert(unique_combination, id);
                    for i in 0..self.vertices.len()-1{
                        if self.vertices[i] == [0., 0., 0.]{
                            self.vertices[i] = mid_point;
                            break;
                        }
                    }
                    id
                }
            }
    
        }

        #[inline]
        pub fn gen_uv_map<const N: usize>(&self) -> [f32; N]{
            use std::mem;
            let uv_map: [[f32; 2]; V];
            let closure = |i: [f32; 3]| -> [f32; 2]{
                let normalized: [f32; 3] = i.map(|x|{x/self.radius});
                let u: f32 = f32::atan2(normalized[0], normalized[2]) / (std::f32::consts::PI * 2.) + 0.5;
                let v: f32 = (f32::asin(-normalized[1]) / std::f32::consts::PI) + 0.5;
                [u, v]
            };
            uv_map = self.vertices.map(closure);
            let ret: &[f32; N] = unsafe{mem::transmute(uv_map.as_ptr())};
            *ret
        }

        #[inline]
        pub fn flipbook_texture_map<const X: usize, const Y: usize, const N: usize>(&self, t: usize, uv_map: &[[f32; 2]; N]) -> [[f32; 2]; N]{
            let x = X as f32;
            let y = Y as f32;

            
            let index = t % (X * Y);
            let x_offset = (index % X) as f32 / x;
            let y_offset = (index / X) as f32 / y;

            let closure = |i: [f32; 2]| -> [f32; 2]{
                [i[0] / x + x_offset, i[1] / y + y_offset]
            };
            uv_map.map(closure)
        }

        pub fn gen_mesh<const N: usize, const M: usize>(&self) -> ([f32; N], [u16; M]){
            use std::mem;
            let vertices: &[f32; N] = unsafe {mem::transmute(self.vertices.as_ptr())};
            let indices: &[u16; M] = unsafe {mem::transmute(self.indices.as_ptr())};
            (*vertices, *indices)
        }

    }

}

pub fn test(){
    use std::mem;
    let test: &[f32; 3*3] = unsafe {mem::transmute(&[[3f32, 3., 3.], [1., 1., 1.], [0., 0., 0.]])};
    super::super::log(&format!("{:?}", *test));
}