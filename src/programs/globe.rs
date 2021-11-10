use super::super::{common_funcs as cf, common_funcs::{matrixes::*, normals::*}};
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlRenderingContext as GL,
    *
};

pub struct Globe {
    pub program: WebGlProgram,                      //Program pointer
    pub indices_buffer: WebGlBuffer,
    pub index_count: i32,
    //pub normals_buffer: WebGlBuffer,
    pub position_buffer: WebGlBuffer,
    //pub u_normals_rotation: WebGlUniformLocation,
    pub u_opacity: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,
    //pub y_buffer: WebGlBuffer,
}

impl Globe {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        use Geomertry_Generator::*;

        let program = cf::link_program(
            &gl,
            &super::super::shaders::vertex::graph_3d::SHADER,
            &super::super::shaders::fragment::varying_color_from_vertex::SHADER,
        ).unwrap();

        let positions_and_indices = gen_mesh(1., 5);// cf::get_position_grid_n_by_n(super::super::constants::GRID_SIZE);
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = positions_and_indices.0.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + positions_and_indices.0.len() as u32,
        );
        let buffer_position = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_position));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let indices_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let indices_location = positions_and_indices.1.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
            indices_location,
            indices_location + positions_and_indices.1.len() as u32,
        );
        let buffer_indices = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);

        Self {
            //u_normals_rotation: gl.get_uniform_location(&program, "uNormalsRotation").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_projection: gl.get_uniform_location(&program, "uProjection").unwrap(),
            program: program,

            indices_buffer: buffer_indices,
            index_count: indices_array.length() as i32,
            //normals_buffer: gl.create_buffer().ok_or("failed normals create buffer").unwrap(),
            position_buffer: buffer_position,
            //y_buffer: gl.create_buffer().ok_or("failed to create buffer").unwrap(),
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
    ) {
        gl.use_program(Some(&self.program));

        let projection_matrix = get_3d_matrices(
            bottom,
            top,
            left,
            right,
            canvas_height,
            canvas_width,
            rotation_angle_x_axis,
            rotation_angle_y_axis,
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_projection),
            false,
            &projection_matrix.projection,
        );
        gl.uniform1f(Some(&self.u_opacity), 1.0);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        //gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.y_buffer));
        // gl.vertex_attrib_pointer_with_i32(1, 1, GL::FLOAT, false, 0, 0);
        // gl.enable_vertex_attrib_array(1);

        // let y_memory_buffer = wasm_bindgen::memory()
        //     .dyn_into::<WebAssembly::Memory>()
        //     .unwrap()
        //     .buffer();
        // //let y_location = y_vals.as_ptr() as u32 / 4;
        // // let y_array = js_sys::Float32Array::new(&y_memory_buffer).subarray(
        // //     y_location,
        // //     y_location + y_vals.len() as u32,
        // // );
        // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &y_array, GL::DYNAMIC_DRAW);

        // //gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_buffer));
        // gl.vertex_attrib_pointer_with_i32(2, 3, GL::FLOAT, false, 0, 0);
        // gl.enable_vertex_attrib_array(2);

        // //let normals_vals = get_grid_normals(super::super::constants::GRID_SIZE, &y_vals);
        // // let normals_vals_memory_buffer = wasm_bindgen::memory()
        // //     .dyn_into::<WebAssembly::Memory>()
        // //     .unwrap()
        // //     .buffer();
        // // let normals_vals_location = normals_vals.as_ptr() as u32 / 4;
        // // let normals_vals_array = js_sys::Float32Array::new(&normals_vals_memory_buffer).subarray(
        // //     normals_vals_location,
        // //     normals_vals_location + normals_vals.len() as u32
        // // );
        // //gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_buffer));
        // gl.buffer_data_with_array_buffer_view(
        //     GL::ARRAY_BUFFER,
        //     &,
        //     GL::DYNAMIC_DRAW,
        // );

        // gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));

        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}

pub mod Geomertry_Generator{
    // mostly coppied from https://github.com/Gonkee/Gepe3D/blob/main/Gepe3D/src/Physics/GeometryGenerator.cs
    use web_sys::{
        WebGlRenderingContext as GL,
        *
    };
    use std::collections::HashMap;
    use nalgebra::Vector3;
    

    pub struct IcoSphere{
        vertices: Vec<Vector3<f32>>,
        indices: Vec<Vector3<u16>>,
        existing_mid_points:HashMap<u32, u16>,
    }

    impl Default for IcoSphere{
        // initialized to the starting icosahedron
        fn default() -> Self {
            let PHI:f32 = (1. + f32::sqrt(5.)) / 2.;

            Self{
                vertices: vec![
                    Vector3::new(-1., PHI, 0.), Vector3::new(  1., PHI, 0.), Vector3::new( -1.,-PHI, 0.),
                    Vector3::new( 1.,-PHI, 0.), Vector3::new(  0.,-1., PHI), Vector3::new(  0., 1., PHI),
                    Vector3::new( 0.,-1.,-PHI), Vector3::new(  0., 1.,-PHI), Vector3::new(  0., 0., -1.),
                    Vector3::new(PHI, 0.,  1.), Vector3::new(-PHI, 0., -1.), Vector3::new(-PHI, 0.,  1.)
                ],
                indices: vec![
                    Vector3::new( 0, 11,  5), Vector3::new(0,  5,  1),Vector3::new(0,  1,  7),Vector3::new( 0,  7, 10),
                    Vector3::new( 0, 10, 11), Vector3::new(1,  5,  9),Vector3::new(5, 11,  4),Vector3::new(11, 10,  2),
                    Vector3::new(10,  7,  6), Vector3::new(7,  1,  8),Vector3::new(3,  9,  4),Vector3::new( 3,  4,  2),
                    Vector3::new( 3,  2,  6), Vector3::new(3,  6,  8),Vector3::new(3,  8,  9),Vector3::new( 4,  9,  5),
                    Vector3::new( 2,  4, 11), Vector3::new(6,  2, 10),Vector3::new(8,  6,  7),Vector3::new( 9,  8,  1)
                ],
                existing_mid_points: HashMap::new()
            }
        }
    }

    impl IcoSphere{
        pub fn new(radius: f32, subdivision: u8) -> Self{
            let mut base = Self::default();

            for i in 0..base.vertices.len() {
                base.vertices[i] = base.vertices[i].normalize();
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
            for i in 0..base.vertices.len(){
                base.vertices[i] *= radius;
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
    }

    pub fn gen_mesh(radius: f32, subdivision: u8) -> (Vec<f32>, Vec<u16>){
        let mesh = IcoSphere::new(radius, subdivision);
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        for i in mesh.vertices{
            vertices.append(&mut vec![i.x, i.y, i.z]);
        }
        for i in mesh.indices{
            indices.append(&mut vec![i.x, i.y, i.z]);
        }
        (vertices, indices)
    }

    

}