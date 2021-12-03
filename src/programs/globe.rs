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
        use vec_to_array::VecToArray;
        use js_sys::*;

        

        let program = cf::link_program(
            &gl,
            &super::super::shaders::vertex::globe::SHADER,
            &super::super::shaders::fragment::globe::SHADER,
        ).unwrap();


        let globe = IcoSphere::new(1., 5);

        //generate arrays for Ico sphere
        let positions_and_indices = globe.gen_mesh();
        let uv_map = globe.gen_uv_map();
        let texture = create_texture(gl, EARTH);

        //Vertices Buffer
        let vertices_array: Float32Array = VecToArray::new(&positions_and_indices.0);
        let position_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        //Indeces Buffer
        let indices_array: Uint16Array = VecToArray::new(&positions_and_indices.1);
        let indices_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);  
        
        //Texture Coordinates Buffer
        let texture_array: Float32Array = VecToArray::new(&uv_map);
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
}

mod geomertry_generator{
    // mostly coppied from https://github.com/Gonkee/Gepe3D/blob/main/Gepe3D/src/Physics/GeometryGenerator.cs

    use std::collections::HashMap;
    use nalgebra::{Vector2, Vector3};
    use super::super::super::log;


    const CENTRE_POINT: Vector3<f32> = Vector3::new(0., 0., 0.);

    pub struct IcoSphere{
        vertices: Vec<Vector3<f32>>,
        indices: Vec<Vector3<u16>>,
        existing_mid_points:HashMap<u32, u16>,
        radius: f32,
    }

    impl Default for IcoSphere{
        // initialized to the starting icosahedron
        fn default() -> Self {
            const SQRT_5:   f32 = 2.23606797749978;
            const PHI:      f32 = (1. + SQRT_5) / 2.;

            Self{
                vertices: vec![
                    Vector3::new( -1., PHI,  0.), Vector3::new(  1., PHI,  0.), Vector3::new( -1.,-PHI,  0.), Vector3::new(  1.,-PHI,  0.), 
                    Vector3::new(  0., -1., PHI), Vector3::new(  0.,  1., PHI), Vector3::new(  0., -1.,-PHI), Vector3::new(  0.,  1.,-PHI),
                    Vector3::new( PHI,  0., -1.), Vector3::new( PHI,  0.,  1.), Vector3::new(-PHI,  0., -1.), Vector3::new(-PHI,  0.,  1.)
                ],
                indices: vec![
                    Vector3::new( 0, 11,  5), Vector3::new(0,  5,  1),Vector3::new(0,  1,  7),Vector3::new( 0,  7, 10),
                    Vector3::new( 0, 10, 11), Vector3::new(1,  5,  9),Vector3::new(5, 11,  4),Vector3::new(11, 10,  2),
                    Vector3::new(10,  7,  6), Vector3::new(7,  1,  8),Vector3::new(3,  9,  4),Vector3::new( 3,  4,  2),
                    Vector3::new( 3,  2,  6), Vector3::new(3,  6,  8),Vector3::new(3,  8,  9),Vector3::new( 4,  9,  5),
                    Vector3::new( 2,  4, 11), Vector3::new(6,  2, 10),Vector3::new(8,  6,  7),Vector3::new( 9,  8,  1)
                ],
                existing_mid_points: HashMap::new(),
                radius: 0.
            }
        }
    }

    impl IcoSphere{
        pub fn new(radius: f32, subdivision: usize) -> Self{
            let mut base = Self::default();
            base.radius = radius;

            for i in &mut base.vertices {
                *i = i.normalize();
            }

            for _ in 0..subdivision {

                // every iteration makes a new list of triangles
                let mut new_indices: Vec<Vector3<u16>> =Vec::new();

                for i in 0..base.indices.len(){
                    let a = base.gen_mid_point_id(base.indices[i].x, base.indices[i].y);
                    let b = base.gen_mid_point_id(base.indices[i].y, base.indices[i].z);
                    let c = base.gen_mid_point_id(base.indices[i].z, base.indices[i].x);

                    // replace triangle with 4 new triangles
                    new_indices.push(Vector3::new(base.indices[i].x, a, c));
                    new_indices.push(Vector3::new(base.indices[i].y, b, a));
                    new_indices.push(Vector3::new(base.indices[i].z, c, b));
                    new_indices.push(Vector3::new(                a, b, c));
                }
                base.indices = new_indices;
            }
            for i in &mut base.vertices{
                *i *= radius;
            }
            base
        }

        fn gen_mid_point_id(&mut self, v1: u16, v2: u16) -> u16{
            use nalgebra::{min, max};
            // check if midpoint has already been generated by another triangle
            let unique_combination: u32 = ((min(v1, v2) as u32) << 16) + max(v1, v2) as u32;
            
            match self.existing_mid_points.contains_key(&unique_combination) {
                true => *self.existing_mid_points.get(&unique_combination).unwrap(),
                false => {
                    let mid_point: Vector3<f32> = ((self.vertices[v1 as usize] + self.vertices[v2 as usize])/2.).normalize();
                    let id: u16 = self.vertices.len() as u16;
                    self.existing_mid_points.insert(unique_combination, id);
                    self.vertices.insert(self.vertices.len(), mid_point);
                    id
                }
            }
    
        }

        pub fn gen_uv_map(&self) -> Vec<f32>{
            let mut uv_vertices: Vec<f32> = Vec::new();

            for i in self.vertices.iter(){
                let normalized = i - CENTRE_POINT / self.radius;
                let u: f32 = f32::atan2(normalized.x, normalized.z) / (std::f32::consts::PI * 2.) + 0.5;
                let v: f32 = (f32::asin(-normalized.y) / std::f32::consts::PI) + 0.5;
                uv_vertices.append(&mut vec![u, v]);
            }
            uv_vertices
        }

        pub fn gen_mesh(&self) -> (Vec<f32>, Vec<u16>){
            let mut vertices: Vec<f32> = Vec::new();
            let mut indices:  Vec<u16> = Vec::new();
            for i in self.vertices.iter(){
                vertices.append(&mut vec![i.x, i.y, i.z]);
            }
            for i in self.indices.iter(){
                indices.append(&mut vec![i.x, i.y, i.z]);

            }
            (vertices, indices)
        }
    }
}