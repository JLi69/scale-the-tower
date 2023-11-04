use std::mem::size_of;
use std::os::raw::c_void;

pub const LEVEL_Z: f32 = -8.0;
const TEXTURE_SCALE: f32 = 8.0;
const CHUNK_SIZE: u32 = 16;
const VERTEX_LEN: usize = 5;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Tile {
    Air,
    Brick,
}

pub struct Level {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,

    level_chunks: Vec<u32>,
    level_chunk_vertex_buffers: Vec<u32>,
    level_chunk_texture_coordinates: Vec<u32>,
    level_chunk_vertex_count: Vec<u32>,
}

impl Level {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            tiles: vec![Tile::Brick; w as usize * h as usize],
            width: w,
            height: h,

            level_chunks: vec![
                0;
                (w as usize / CHUNK_SIZE as usize + 1)
                    * (h as usize / CHUNK_SIZE as usize + 1)
            ],
            level_chunk_vertex_buffers: vec![
                0;
                (w as usize / CHUNK_SIZE as usize + 1)
                    * (h as usize / CHUNK_SIZE as usize + 1)
            ],
            level_chunk_texture_coordinates: vec![
                0;
                (w as usize / CHUNK_SIZE as usize + 1)
                    * (h as usize / CHUNK_SIZE as usize + 1)
            ],
            level_chunk_vertex_count: vec![
                0;
                (w as usize / CHUNK_SIZE as usize + 1)
                    * (h as usize / CHUNK_SIZE as usize + 1)
            ],
        }
    }

    //NOTE: Delete this later
    pub fn test_level() -> Self {
        let mut test_level = Self::new(32, 32);

        for x in 8..24 {
            for y in 8..24 {
                test_level.set_tile(x, y, Tile::Air);
            }
        }

        test_level.set_tile(12, 9, Tile::Brick);
        test_level.set_tile(15, 11, Tile::Brick);
        test_level.set_tile(16, 11, Tile::Brick);
        test_level.set_tile(17, 11, Tile::Brick);
        test_level.set_tile(19, 11, Tile::Brick);

        test_level
    }

    fn add_vertices(&self, x: u32, y: u32, face: &[f32; 30], vertices: &mut Vec<f32>) {
        for i in 0..6 {
            vertices.push(face[i * VERTEX_LEN] + 2.0 * x as f32);
            vertices.push(face[i * VERTEX_LEN + 1] + 2.0 * y as f32);
            vertices.push(face[i * VERTEX_LEN + 2]);

            match self.get_tile(x, y) {
                Tile::Brick => {
                    vertices.push(face[i * VERTEX_LEN + 3] + 1.0 / TEXTURE_SCALE);
                    vertices.push(face[i * VERTEX_LEN + 4]);
                }
                _ => {
                    vertices.push(face[i * VERTEX_LEN + 3]);
                    vertices.push(face[i * VERTEX_LEN + 4]);
                }
            }
        }
    }

    fn add_tile_vertices(&self, x: u32, y: u32, vertices: &mut Vec<f32>) {
        if self.get_tile(x, y) == Tile::Air {
            return;
        }

        let front_face = [
            1.0f32,
            1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            -1.0,
            1.0,
            1.0,
            0.0,
            0.0,
            -1.0,
            -1.0,
            1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            -1.0,
            -1.0,
            1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            -1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            1.0 / TEXTURE_SCALE,
        ];
        let top_face = [
            1.0f32,
            1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            1.0 / TEXTURE_SCALE,
            1.0,
            1.0,
            -1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            -1.0,
            1.0,
            1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            1.0,
            -1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            -1.0,
            1.0,
            -1.0,
            0.0,
            0.0,
            -1.0,
            1.0,
            1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
        ];
        let bottom_face = [
            -1.0f32,
            -1.0,
            1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            -1.0,
            -1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            1.0,
            -1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            1.0 / TEXTURE_SCALE,
            -1.0,
            -1.0,
            1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            -1.0,
            -1.0,
            -1.0,
            0.0,
            0.0,
            1.0,
            -1.0,
            -1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
        ];
        let left_face = [
            -1.0f32,
            1.0,
            -1.0,
            0.0,
            0.0,
            -1.0,
            -1.0,
            -1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            -1.0,
            1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            -1.0,
            1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            -1.0,
            -1.0,
            -1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            -1.0,
            -1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            1.0 / TEXTURE_SCALE,
        ];
        let right_face = [
            1.0f32,
            -1.0,
            -1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            1.0,
            -1.0,
            0.0,
            0.0,
            1.0,
            1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            1.0,
            -1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            1.0 / TEXTURE_SCALE,
            1.0,
            -1.0,
            -1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            1.0,
            1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
        ];

        self.add_vertices(x, y, &front_face, vertices);

        if self.out_of_bounds(x as i32, y as i32 + 1) || self.get_tile(x, y + 1) == Tile::Air {
            self.add_vertices(x, y, &top_face, vertices);
        }

        if self.out_of_bounds(x as i32, y as i32 - 1) || self.get_tile(x, y - 1) == Tile::Air {
            self.add_vertices(x, y, &bottom_face, vertices);
        }

        if self.out_of_bounds(x as i32 - 1, y as i32) || self.get_tile(x - 1, y) == Tile::Air {
            self.add_vertices(x, y, &left_face, vertices);
        }

        if self.out_of_bounds(x as i32 + 1, y as i32) || self.get_tile(x + 1, y) == Tile::Air {
            self.add_vertices(x, y, &right_face, vertices);
        }
    }

    pub fn get_chunk_vertices(&self, chunk_x: u32, chunk_y: u32) -> Vec<f32> {
        let mut vertices = vec![];

        for x in (chunk_x * CHUNK_SIZE)..(chunk_x * CHUNK_SIZE + CHUNK_SIZE) {
            for y in (chunk_y * CHUNK_SIZE)..(chunk_y * CHUNK_SIZE + CHUNK_SIZE) {
                self.add_tile_vertices(x, y, &mut vertices);
            }
        }

        vertices
    }

    pub fn build_chunk(&mut self, chunk_x: u32, chunk_y: u32) {
        let vertices = self.get_chunk_vertices(chunk_x, chunk_y);
        self.level_chunk_vertex_count
            [(chunk_x + chunk_y * (self.width / CHUNK_SIZE + 1)) as usize] =
            (vertices.len() / VERTEX_LEN) as u32;
        unsafe {
            gl::BindVertexArray(
                self.level_chunks[(chunk_x + chunk_y * (self.width / CHUNK_SIZE + 1)) as usize],
            );

            gl::BindBuffer(
                gl::ARRAY_BUFFER,
                self.level_chunk_vertex_buffers
                    [(chunk_x + chunk_y * (self.width / CHUNK_SIZE + 1)) as usize],
            );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (VERTEX_LEN * size_of::<f32>()) as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(
                gl::ARRAY_BUFFER,
                self.level_chunk_texture_coordinates
                    [(chunk_x + chunk_y * (self.width / CHUNK_SIZE + 1)) as usize],
            );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (VERTEX_LEN * size_of::<f32>()) as i32,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }
    }

    pub fn build_chunks(&mut self) {
        unsafe {
            gl::GenVertexArrays(
                self.level_chunks.len() as i32,
                self.level_chunks.as_mut_ptr(),
            );
            gl::GenBuffers(
                self.level_chunk_vertex_buffers.len() as i32,
                self.level_chunk_vertex_buffers.as_mut_ptr(),
            );
            gl::GenBuffers(
                self.level_chunk_texture_coordinates.len() as i32,
                self.level_chunk_texture_coordinates.as_mut_ptr(),
            );
        }

        for chunk_x in 0..(self.width / CHUNK_SIZE) {
            for chunk_y in 0..(self.height / CHUNK_SIZE) {
                self.build_chunk(chunk_x, chunk_y);
            }
        }
    }

    pub fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        if x >= self.width || y >= self.height {
            return Tile::Air;
        }

        self.tiles[((self.width * y) + x) as usize]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.tiles[((self.width * y) + x) as usize] = tile;
    }

    pub fn w(&self) -> u32 {
        self.width
    }

    pub fn h(&self) -> u32 {
        self.height
    }

    //Display level
    pub fn display(&self) {
        for i in 0..self.level_chunks.len() {
            unsafe {
                gl::BindVertexArray(self.level_chunks[i]);
                gl::DrawArrays(gl::TRIANGLES, 0, self.level_chunk_vertex_count[i] as i32);
            }
        }
    }
}

impl Drop for Level {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(self.level_chunks.len() as i32, self.level_chunks.as_ptr());
            gl::DeleteBuffers(
                self.level_chunk_vertex_buffers.len() as i32,
                self.level_chunk_vertex_buffers.as_ptr(),
            );
            gl::DeleteBuffers(
                self.level_chunk_texture_coordinates.len() as i32,
                self.level_chunk_texture_coordinates.as_ptr(),
            );
        }
    }
}
