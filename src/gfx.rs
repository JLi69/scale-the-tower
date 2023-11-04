use std::fs::File;
use std::mem::size_of;
use std::os::raw::c_void;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    //Attempt to load texture from file
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let file_res = File::open(path);

        match file_res {
            Ok(file) => {
                let decoder = png::Decoder::new(file);
                let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
                let mut buf = vec![0u8; reader.output_buffer_size()];
                let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
                let bytes = &buf[..info.buffer_size()];

                let mut texture = 0;
                unsafe {
                    gl::GenTextures(1, &mut texture);
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    gl::TextureParameteri(texture, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                    gl::TextureParameteri(texture, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA as i32,
                        info.width as i32,
                        info.height as i32,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        bytes.as_ptr() as *const c_void,
                    );
                    gl::GenerateMipmap(gl::TEXTURE_2D);
                }

                Ok(Self { id: texture })
            }
            Err(msg) => {
                eprintln!("Failed to open file: {path}");
                eprintln!("{msg}");
                Err(msg.to_string())
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

pub struct VertexArrayObject {
    id: u32,
    vert_buffers: Vec<u32>,
    vertex_count: i32,
}

impl VertexArrayObject {
    //Textured rectangle
    pub fn create_rectangle() -> Self {
        let mut vao = 0;
        let mut buffers = vec![0u32, 0];

        unsafe {
            let vertices = [
                1.0f32, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0,
                1.0, -1.0, 0.0,
            ];

            let texture_coordinates = [
                1.0f32, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            ];

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(2, &mut buffers[0]);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffers[0]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as isize,
                &vertices as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * size_of::<f32>() as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffers[1]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (texture_coordinates.len() * size_of::<f32>()) as isize,
                &texture_coordinates as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * size_of::<f32>() as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }

        Self {
            id: vao,
            vert_buffers: buffers,
            vertex_count: 6, //6 vertices in a rectangle
        }
    }

    pub fn create_cube() -> Self {
        let mut vao = 0;
        let mut buffers = vec![0, 0];

        unsafe {
            let vertices = [
                /* Front face */
                1.0f32, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
                1.0, -1.0, 1.0, /* Top face */
                1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
                -1.0, 1.0, 1.0, /* Bottom face */
                -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
                1.0, -1.0, -1.0, /* Left face */
                -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
                -1.0, -1.0, 1.0, /* Right face */
                1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0,
                1.0, 1.0, 1.0, /* Back face */
                -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0,
                -1.0, 1.0, 1.0, -1.0,
            ];

            let texture_coordinates = [
                /* Front face */
                1.0f32, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                /* Top face */
                1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                /* Bottom face */
                0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                /* Left face */
                0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                /* Right face */
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0,
                /* Back face */
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0,
            ];

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(2, &mut buffers[0]);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffers[0]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as isize,
                &vertices as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * size_of::<f32>() as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffers[1]);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (texture_coordinates.len() * size_of::<f32>()) as isize,
                &texture_coordinates as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * size_of::<f32>() as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }

        Self {
            id: vao,
            vert_buffers: buffers,
            vertex_count: 36, //36 vertices in a cube
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn draw_arrays(&self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(self.vert_buffers.len() as i32, self.vert_buffers.as_ptr());
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub fn output_gl_errors() {
    unsafe {
        let mut gl_err = gl::GetError();
        while gl_err != gl::NO_ERROR {
            eprintln!("OpenGL Error: {gl_err}");
            gl_err = gl::GetError();
        }
    }
}
