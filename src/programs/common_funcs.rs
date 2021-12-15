pub mod cf{
    use web_sys::{
        *,
        WebGlRenderingContext as GL,
    };

    pub fn link_program(
        gl: &WebGlRenderingContext,
        vert_source: &str,
        frag_source: &str,
    ) -> Result<WebGlProgram, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Error creating program"))?;
    
        let vert_shader = compile_shader(
            &gl,
            GL::VERTEX_SHADER,
            vert_source,
        ).unwrap();
    
        let frag_shader = compile_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            frag_source,
        ).unwrap();
    
        gl.attach_shader(&program, &vert_shader);
        gl.attach_shader(&program, &frag_shader);
        gl.link_program(&program);
    
        if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(gl.get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    fn compile_shader(
        gl: &WebGlRenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Error creating shader"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);
        
        if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false) 
        {
            Ok(shader)
        } else {
            Err(gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unable to get shader info log")))
        }
    }
}

pub mod matrixes{
    use nalgebra::{Matrix4, Perspective3};
    use super::super::super::constants::*;
    pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
        let mut return_var = [0.; 16];
    
        return_var[0] = 1.;
        return_var[5] = 1.;
        return_var[10] = 1.;
        return_var[15] = 1.;
    
        return_var[12] = tx;
        return_var[13] = ty;
        return_var[14] = tz;
    
        return_var
    }
    
    pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
        let mut return_var = [0.; 16];
    
        return_var[0] = sx;
        return_var[5] = sy;
        return_var[10] = sz;
        return_var[15] = 1.;
    
        return_var
    }
    
    pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
        let mut return_var = [0.; 16];
    
        return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
        return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
        return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
        return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];
    
        return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
        return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
        return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
        return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];
    
        return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
        return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
        return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
        return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];
    
        return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
        return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
        return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
        return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];
    
        return_var
    }

    pub fn get_updated_3d_y_values(curr_time: f32) -> Vec<f32> {
        let point_count_per_row = GRID_SIZE + 1;
        let mut y_vals: Vec<f32> = vec![0.; point_count_per_row * point_count_per_row];
        let half_grid: f32 = point_count_per_row as f32 / 2.;
        let frequency_scale: f32 = 3. * std::f32::consts::PI;
        let y_scale = 0.15;
        let sin_offset = curr_time / 1000.; //speed

        for z in 0..point_count_per_row {
            for x in 0..point_count_per_row {
                let use_y_index = z * point_count_per_row + x;
                let scaled_x = frequency_scale * (x as f32 - half_grid) / half_grid;
                let scaled_z = frequency_scale * (z as f32 - half_grid) / half_grid;
                y_vals[use_y_index] = y_scale * ((scaled_x * scaled_x + scaled_z * scaled_z).sqrt() + sin_offset).sin();
            }
        }

        y_vals
    }

    pub fn get_3d_matrices(
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
        rotation_angle_x_axis: f32,
        rotation_angle_y_axis: f32,
        _zoom: f32,
    ) -> Matrices3D {
        let mut return_var = Matrices3D {
            normals_rotation: [0.; 16],
            projection: [0.; 16],
        };
    
        let rotate_x_axis: [f32; 16] = [
            1., 0.,                          0.,                            0.,
            0., rotation_angle_x_axis.cos(), -rotation_angle_x_axis.sin(),  0.,
            0., rotation_angle_x_axis.sin(), rotation_angle_x_axis.cos(),   0.,
            0., 0.,                          0.,                            1.,
        ];
    
        let rotate_y_axis: [f32; 16] = [
            rotation_angle_y_axis.cos(),  0., rotation_angle_y_axis.sin(), 0.,
            0.,                           1., 0.,                          0.,
            -rotation_angle_y_axis.sin(), 0., rotation_angle_y_axis.cos(), 0.,
            0.,                           0., 0.,                          1.,
        ];
        
        //et rotation_matrix = Rotation::new();
        let rotation_matrix = mult_matrix_4(rotate_y_axis, rotate_x_axis);
    
        let aspect: f32 = canvas_width / canvas_height;
        let scale_x = (right - left ) / canvas_width ;
        let scale_y = (top - bottom ) / canvas_height;
        let scale = scale_y * 2.;// * 0.5;//+ zoom as f32;
    
        let translation_matrix: [f32; 16] = translation_matrix(
            -1. + scale_x + 2. * left / canvas_width,
            -1. + scale_y + 2. * bottom / canvas_height,
            Z_PLANE,
        );
    
        let scale_matrix: [f32; 16] = scaling_matrix(scale, scale, 0.);
        let rotation_scale = mult_matrix_4(rotation_matrix, scale_matrix);
        let combined_transform = mult_matrix_4(rotation_scale, translation_matrix);
        let perspective_matrix_tmp: Perspective3<f32> = Perspective3::new(aspect, FIELD_OF_VIEW, Z_NEAR, Z_FAR);
        let mut perspective: [f32; 16] = [0.; 16];
        perspective.copy_from_slice(perspective_matrix_tmp.as_matrix().as_slice());
    
        return_var.projection = mult_matrix_4(combined_transform, perspective);
    
        let normal_matrix = Matrix4::new(
            rotation_matrix[0],
            rotation_matrix[1],
            rotation_matrix[2],
            rotation_matrix[3],
            rotation_matrix[4],
            rotation_matrix[5],
            rotation_matrix[6],
            rotation_matrix[7],
            rotation_matrix[8],
            rotation_matrix[9],
            rotation_matrix[10],
            rotation_matrix[11],
            rotation_matrix[12],
            rotation_matrix[13],
            rotation_matrix[14],
            rotation_matrix[15],
        );
    
        match normal_matrix.try_inverse() {
            Some(inv) => {
                return_var.normals_rotation.copy_from_slice(inv.as_slice());
            }
            None => {}
        }
    
        return_var
    }

    pub struct Matrices3D {
        pub normals_rotation: [f32; 16],
        pub projection: [f32; 16],
    }
}

pub mod normals{

    pub fn normalize(vector: [f32; 3]) -> [f32; 3]{
        let closure = |i: f32|{
            i/f32::sqrt(f32::powi(vector[0], 2) + f32::powi(vector[1], 2) + f32::powi(vector[2], 2))
        };
        vector.map(closure)
    }

    pub fn get_grid_normals(n: usize, y_vals: &Vec<f32>) -> Vec<f32> {
        let points_per_row = n + 1;
        let graph_layout_width: f32 = 2.;
        let square_size: f32 = graph_layout_width / n as f32;
        let mut return_var: Vec<f32> = vec![0.; 3 * points_per_row * points_per_row];
    
        for z in 0..points_per_row {
            for x in 0..points_per_row {
                let y_val_index_a = z * points_per_row + x;
                let return_var_start_index = 3 * y_val_index_a;
    
                if z == n || x == n {
                    return_var[return_var_start_index + 1] = 1.; //default
                } else {
                    let y_val_index_b = y_val_index_a + points_per_row;
                    let y_val_index_c = y_val_index_a + 1;
                    
                    let x_val_1 = square_size * x as f32;
                    let x_val_2 = x_val_1 + square_size;
    
                    let z_val_1 = square_size * z as f32;
                    let z_val_2 = z_val_1 + square_size;
    
                    let normals = get_normal_vec(
                        x_val_1,
                        y_vals[y_val_index_a],
                        z_val_1,
                        x_val_1,
                        y_vals[y_val_index_b],
                        z_val_2,
                        x_val_2,
                        y_vals[y_val_index_c],
                        z_val_2,
                    );
    
                    return_var[return_var_start_index + 0] = normals.0;
                    return_var[return_var_start_index + 1] = normals.1;
                    return_var[return_var_start_index + 2] = normals.2;
                }
            }
        }
    
        return_var
    }
    
    pub fn get_normal_vec(
        point_a_x: f32,
        point_a_y: f32,
        point_a_z: f32,
        point_b_x: f32,
        point_b_y: f32,
        point_b_z: f32,
        point_c_x: f32,
        point_c_y: f32,
        point_c_z: f32,
    ) -> (f32, f32, f32) {
        let u_x = point_b_x - point_a_x;
        let u_y = point_b_y - point_a_y;
        let u_z = point_b_z - point_a_z;
    
        let v_x = point_c_x - point_a_x;
        let v_y = point_c_y - point_a_y;
        let v_z = point_c_z - point_a_z;
    
        let normal_x = u_y * v_z - v_y * u_z;
        let normal_y = -1. * (u_x * v_z - v_x * u_z);
        let normal_z = u_x * v_y - v_x * u_y;
    
        let normal_size = (normal_x * normal_x + normal_y * normal_y + normal_z * normal_z).sqrt();
    
        (
            normal_x / normal_size,
            normal_y / normal_size,
            normal_z / normal_size,
        )
    }
}

pub mod textures{
    use web_sys::{
        WebGlRenderingContext as GL,
        *
    };
    use js_sys::*;

    const TEXTURES: [u32; 32] = [
        GL::TEXTURE0,
        GL::TEXTURE1,
        GL::TEXTURE2,
        GL::TEXTURE3,
        GL::TEXTURE4,
        GL::TEXTURE5,
        GL::TEXTURE6,
        GL::TEXTURE7,
        GL::TEXTURE8,
        GL::TEXTURE9,
        GL::TEXTURE10,
        GL::TEXTURE11,
        GL::TEXTURE12,
        GL::TEXTURE13,
        GL::TEXTURE14,
        GL::TEXTURE15,
        GL::TEXTURE16,
        GL::TEXTURE17,
        GL::TEXTURE18,
        GL::TEXTURE19,
        GL::TEXTURE20,
        GL::TEXTURE21,
        GL::TEXTURE22,
        GL::TEXTURE23,
        GL::TEXTURE24,
        GL::TEXTURE25,
        GL::TEXTURE26,
        GL::TEXTURE27,
        GL::TEXTURE28,
        GL::TEXTURE29,
        GL::TEXTURE30,
        GL::TEXTURE31,
    ];

    #[inline]
    pub fn texture_coord_buffer< const UV: usize>(gl: &GL, uv_map: &[f32; UV]) -> WebGlBuffer{
        use super::vec_to_array::*;
        let uv_map: &[f32; UV] = unsafe{std::mem::transmute(uv_map.as_ptr())};
        let texture_array: Float32Array = ArrayToJS::new(uv_map);
        let tex_coord_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coord_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &texture_array, GL::STATIC_DRAW);
        tex_coord_buffer
    }
    
    #[inline]
    pub fn active_texture(gl: &GL, texture: &WebGlTexture, index: usize, location: &WebGlUniformLocation){
        gl.active_texture(TEXTURES[index]);
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        gl.uniform1i(Some(location), index as i32);
    }

    #[inline]
    pub fn create_texture(gl: &GL, src: &str) -> WebGlTexture{
        use wasm_bindgen::{closure::Closure, JsCast}; 
        
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        let image = HtmlImageElement::new().unwrap();
        image.set_src(src);
        super::super::super::log(&image.current_src());

        //Event handler for when the image is loaded
        let gl_c = gl.clone();
        let texture_c = texture.clone();
        let image_c = image.clone();
        let listener: Closure<dyn Fn()> = Closure::wrap(Box::new(move ||{
                //image_on_load(&gl_c, &texture_c, &image_c);
                gl_c.bind_texture(GL::TEXTURE_2D, Some(&texture_c));
                gl_c.tex_image_2d_with_u32_and_u32_and_image(
                    GL::TEXTURE_2D,         //target
                    0,                      //level
                    GL::RGBA as i32,          //internal format
                    GL::RGBA,                 //source format
                    GL::UNSIGNED_BYTE,      //source type
                    &image_c                //image
                ).unwrap();    

                const IS_POWER_OF_TWO: fn(i32) -> bool = |x: i32| -> bool {
                    x & x-1 == 0
                };
                if IS_POWER_OF_TWO(image_c.width() as i32) && IS_POWER_OF_TWO(image_c.height() as i32){
                    gl_c.generate_mipmap(GL::TEXTURE_2D);
                }
                else{
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::MIRRORED_REPEAT as i32);
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
                }
        }));
        image.set_onload(Some(listener.as_ref().unchecked_ref()));
        listener.forget();

        //Pixel to be used while loading
        const WIDTH: i32 = 1;
        const HEIGHT: i32 = 1;
        const SRC_FORMAT: u32 =  GL::RGBA;
        const SRC_TYPE: u32 = GL::UNSIGNED_BYTE;
        let mut pixel: [u8; 4] = [0, 0, 255, 255];
        gl.read_pixels_with_opt_u8_array(0, 0, WIDTH, HEIGHT, SRC_FORMAT, SRC_TYPE, Some(&mut pixel)).unwrap();

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::MIRRORED_REPEAT as i32);

        texture
    }

    pub fn create_alpha(gl: &GL, src: &str) -> WebGlTexture{
        use wasm_bindgen::{closure::Closure, JsCast}; 
        
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        let image = HtmlImageElement::new().unwrap();
        image.set_src(src);
        super::super::super::log(&image.current_src());

        //Event handler for when the image is loaded
        let gl_c = gl.clone();
        let texture_c = texture.clone();
        let image_c = image.clone();
        let listener: Closure<dyn Fn()> = Closure::wrap(Box::new(move ||{
                //image_on_load(&gl_c, &texture_c, &image_c);
                gl_c.bind_texture(GL::TEXTURE_2D, Some(&texture_c));
                gl_c.tex_image_2d_with_u32_and_u32_and_image(
                    GL::TEXTURE_2D,         //target
                    0,                      //level
                    GL::RGBA as i32,          //internal format
                    GL::RGBA,                 //source format
                    GL::UNSIGNED_BYTE,      //source type
                    &image_c                //image
                ).unwrap();    

                const IS_POWER_OF_TWO: fn(i32) -> bool = |x: i32| -> bool {
                    x & x-1 == 0
                };
                if IS_POWER_OF_TWO(image_c.width() as i32) && IS_POWER_OF_TWO(image_c.height() as i32){
                    gl_c.generate_mipmap(GL::TEXTURE_2D);
                }
                else{
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
                    gl_c.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
                }
        }));
        image.set_onload(Some(listener.as_ref().unchecked_ref()));
        listener.forget();

        //Pixel to be used while loading
        const WIDTH: i32 = 1;
        const HEIGHT: i32 = 1;
        const SRC_FORMAT: u32 =  GL::RGBA;
        const SRC_TYPE: u32 = GL::UNSIGNED_BYTE;
        let mut pixel: [u8; 4] = [0, 0, 255, 255];
        gl.read_pixels_with_opt_u8_array(0, 0, WIDTH, HEIGHT, SRC_FORMAT, SRC_TYPE, Some(&mut pixel)).unwrap();

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::MIRRORED_REPEAT as i32);

        texture
    }

    pub fn create_texture_from_u8<const S: usize>(gl: &GL, gradient: &mut [u8; S]) -> WebGlTexture{
        
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(GL::TEXTURE_2D, 0, GL::RGBA as i32, 256, 1, 0, GL::RGBA, GL::UNSIGNED_BYTE, Some(gradient)).unwrap();
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::MIRRORED_REPEAT as i32);

        texture
    }

    pub fn create_gradient(alpha: u8) -> [[u8; 4]; 256]{
        let mut gradient = [[0; 4]; 256];
        for i in 0..256{
            match i {
                0 => gradient[0] = [200, 200, 200, 255],
                1..=127 => gradient[i] = [i as u8*2, i as u8*2, 255, alpha],
                128 => gradient[128] = [255; 4],
                _ => gradient[i] = [255, 255-(i as u8-128u8)*2+1, 255-(i as u8-128u8)*2+1, alpha]
            }
        }
        gradient
    }

    
}

pub mod vec_to_array{
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