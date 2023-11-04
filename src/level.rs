use crate::gfx::VertexArrayObject;
use crate::shader::ShaderProgram;
use cgmath::Matrix4;

pub const LEVEL_Z: f32 = -8.0;

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
}

impl Level {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            tiles: vec![Tile::Brick; w as usize * h as usize],
            width: w,
            height: h,
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

        test_level
    }

    pub fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            true
        } else {
            false
        }
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
    pub fn display(&self, program: &ShaderProgram, cube_vao: &VertexArrayObject) {
        program.uniform_vec4f("uColor", 0.5, 0.5, 0.5, 1.0);
        cube_vao.bind();
        for x in 0..self.w() {
            for y in 0..self.h() {
                let transform_matrix =
                    Matrix4::from_translation(cgmath::vec3(x as f32, y as f32, 0.0))
                        * Matrix4::from_scale(0.5);
                match self.get_tile(x, y) {
                    Tile::Brick => {
                        program.uniform_vec2f("uTexOffset", 1.0 / 8.0, 0.0);
                        program.uniform_matrix4f("uTransform", &transform_matrix);
                        cube_vao.draw_arrays();
                    }
                    _ => {}
                }
            }
        }
    }
}
