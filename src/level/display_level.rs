use super::{transparent, BackgroundTile, InteractiveTile, Level, Tile, CHUNK_SIZE};
use crate::gfx::VertexArrayObject;
use crate::shader::ShaderProgram;
use cgmath::{Matrix4, Vector2};
use std::mem::size_of;
use std::os::raw::c_void;

//There will be a maximum of TEXTURE_SCALE * TEXTURE_SCALE
//different tile textures that will be stored in a single texture
const TEXTURE_SCALE: f32 = 8.0;
//Number of f32 elements in a vertex
const VERTEX_LEN: usize = 5;
const STRIDE: usize = 6;
const LAVA_HEIGHT: f32 = 0.8;

const SPRITE_RENDER_DISTANCE: f32 = 64.0;

impl Level {
    //Adds the vertices of a single face to the vertex vector
    fn add_vertices(&self, x: u32, y: u32, face: &[f32; VERTEX_LEN * 6], vertices: &mut Vec<f32>) {
        //6 vertices per face
        for i in 0..6 {
            //Add the vertex position (x, y, z)
            if self.get_tile(x, y) == Tile::Spikes {
                vertices.push(face[i * VERTEX_LEN] / 2.0f32.sqrt() + 2.0 * x as f32);
            } else {
                vertices.push(face[i * VERTEX_LEN] + 2.0 * x as f32);
            }

            if self.get_tile(x, y) == Tile::Lava
                && self.get_tile(x, y + 1) != Tile::Lava
                && face[i * VERTEX_LEN + 1] > 0.0
            {
                vertices.push(face[i * VERTEX_LEN + 1] * LAVA_HEIGHT + 2.0 * y as f32);
            } else {
                vertices.push(face[i * VERTEX_LEN + 1] + 2.0 * y as f32);
            }

            if self.get_tile(x, y) == Tile::Ladder {
                vertices.push(face[i * VERTEX_LEN + 2] - 1.2);
            } else if self.get_tile(x, y) == Tile::Spikes {
                vertices.push(
                    face[i * VERTEX_LEN + 2] + (face[i * VERTEX_LEN] / 2.0).fract() / 2.0f32.sqrt()
                        - 1.0,
                );
            } else {
                vertices.push(face[i * VERTEX_LEN + 2]);
            }

            let texture_coords = [
                (face[i * VERTEX_LEN + 3] - 0.01 / TEXTURE_SCALE).max(0.0),
                (face[i * VERTEX_LEN + 4] - 0.01 / TEXTURE_SCALE).max(0.0),
            ];

            //Add the texture coordinates
            match self.get_tile(x, y) {
                Tile::Brick => {
                    vertices.push(texture_coords[0] + 1.0 / TEXTURE_SCALE);
                    vertices.push(texture_coords[1]);
                }
                Tile::Ladder => {
                    vertices.push(texture_coords[0] + 2.0 / TEXTURE_SCALE);
                    vertices.push(texture_coords[1]);
                }
                Tile::BrickTile => {
                    vertices.push(texture_coords[0] + 3.0 / TEXTURE_SCALE);
                    vertices.push(texture_coords[1]);
                }
                Tile::BrickTile2 => {
                    vertices.push(texture_coords[0] + 4.0 / TEXTURE_SCALE);
                    vertices.push(texture_coords[1]);
                }
                Tile::Lava => {
                    vertices.push(texture_coords[0] + 5.0 / TEXTURE_SCALE);
                    vertices.push(texture_coords[1]);
                }
                Tile::Spikes => {
                    vertices.push(texture_coords[0] + 6.0 / TEXTURE_SCALE);
                    vertices.push(texture_coords[1]);
                }
                _ => {
                    vertices.push(texture_coords[0]);
                    vertices.push(texture_coords[1]);
                }
            }

            if self.get_tile(x, y) == Tile::Lava {
                vertices.push(1.0 / TEXTURE_SCALE);
            } else { 
                vertices.push(0.0);
            }
        }

        if self.get_tile(x, y) == Tile::Spikes {
            for i in 0..6 {
                //Add the vertex position (x, y, z)
                vertices.push(face[i * VERTEX_LEN] / 2.0f32.sqrt() + 2.0 * x as f32);
                vertices.push(face[i * VERTEX_LEN + 1] + 2.0 * y as f32);
                vertices.push(
                    face[i * VERTEX_LEN + 2]
                        - 1.0
                        - (face[i * VERTEX_LEN] / 2.0).fract() / 2.0f32.sqrt(),
                );

                let texture_coords = [
                    (face[i * VERTEX_LEN + 3] - 0.01 / TEXTURE_SCALE).max(0.0),
                    (face[i * VERTEX_LEN + 4] - 0.01 / TEXTURE_SCALE).max(0.0),
                ];

                //Add the texture coordinates
                vertices.push(texture_coords[0] + 6.0 / TEXTURE_SCALE);
                vertices.push(texture_coords[1]);

                vertices.push(0.0);
            }
        }
    }

    fn add_tile_vertices(&self, x: u32, y: u32, vertices: &mut Vec<f32>) {
        //In the case where the tile is air, ignore it and simply return
        if self.get_tile(x, y) == Tile::Air {
            return;
        }

        //The faces of each tile,
        //each vertex of the tile consists of 5 f32 values:
        //vertex position: (x, y, z) texture coordinates: (tx, ty)
        //Each array is structured [ x, y, z, tx, ty, ... ]
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

        //Add the front face vertices, these are never covered by another
        //tile so we add then always. We never see the back face so there
        //is no point in adding that to the mesh
        self.add_vertices(x, y, &front_face, vertices);

        if self.get_tile(x, y) == Tile::Ladder || self.get_tile(x, y) == Tile::Spikes {
            return;
        }

        //Check if the other faces are covered so that we don't add more
        //vertices than we need to. The out of bounds check is to make sure
        //that x and y don't underflow when subtracting 1 from them and avoid
        //causing a crash in debug mode
        if self.out_of_bounds(x as i32, y as i32 + 1)
            || transparent(self.get_tile(x, y + 1))
            || (self.get_tile(x, y) == Tile::Lava && self.get_tile(x, y + 1) != Tile::Lava)
        {
            self.add_vertices(x, y, &top_face, vertices);
        }

        if self.out_of_bounds(x as i32, y as i32 - 1) || transparent(self.get_tile(x, y - 1)) {
            self.add_vertices(x, y, &bottom_face, vertices);
        }

        if self.out_of_bounds(x as i32 - 1, y as i32) || transparent(self.get_tile(x - 1, y)) {
            self.add_vertices(x, y, &left_face, vertices);
        }

        if self.out_of_bounds(x as i32 + 1, y as i32) || transparent(self.get_tile(x + 1, y)) {
            self.add_vertices(x, y, &right_face, vertices);
        }
    }

    pub fn add_background_vertices(&self, x: u32, y: u32, vertices: &mut Vec<f32>) {
        let background = [
            1.0f32,
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
            -1.0,
            -1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            1.0,
            -1.0,
            1.0 / TEXTURE_SCALE,
            0.0,
            -1.0,
            -1.0,
            -1.0,
            0.0,
            1.0 / TEXTURE_SCALE,
            1.0,
            -1.0,
            -1.0,
            1.0 / TEXTURE_SCALE,
            1.0 / TEXTURE_SCALE,
        ];

        if !transparent(self.get_tile(x, y))
            || self.get_background_tile(x, y) == BackgroundTile::Empty
        {
            return;
        }

        //6 vertices per face
        for i in 0..6 {
            //Add the vertex position (x, y, z)
            vertices.push(background[i * VERTEX_LEN] + 2.0 * x as f32);
            vertices.push(background[i * VERTEX_LEN + 1] + 2.0 * y as f32);
            vertices.push(background[i * VERTEX_LEN + 2]);

            //Add the texture coordinates
            match self.get_background_tile(x, y) {
                BackgroundTile::Wall => {
                    vertices.push(background[i * VERTEX_LEN + 3]);
                    vertices.push(background[i * VERTEX_LEN + 4] + 4.0 / TEXTURE_SCALE);
                }
                _ => {
                    vertices.push(background[i * VERTEX_LEN + 3]);
                    vertices.push(background[i * VERTEX_LEN + 4]);
                }
            }

            vertices.push(0.0);
        }
    }

    //Builds a vector of vertices for a single chunk
    pub fn get_chunk_vertices(&self, chunk_x: u32, chunk_y: u32) -> Vec<f32> {
        let mut vertices = vec![];

        for x in (chunk_x * CHUNK_SIZE)..(chunk_x * CHUNK_SIZE + CHUNK_SIZE) {
            for y in (chunk_y * CHUNK_SIZE)..(chunk_y * CHUNK_SIZE + CHUNK_SIZE) {
                self.add_background_vertices(x, y, &mut vertices);
            }
        }

        for x in (chunk_x * CHUNK_SIZE)..(chunk_x * CHUNK_SIZE + CHUNK_SIZE) {
            for y in (chunk_y * CHUNK_SIZE)..(chunk_y * CHUNK_SIZE + CHUNK_SIZE) {
                self.add_tile_vertices(x, y, &mut vertices);
            }
        }

        vertices
    }

    pub fn build_chunk(&mut self, chunk_x: u32, chunk_y: u32) {
        //Vertices is a vector of f32 values that represent the vertices of
        //the chunk of tiles
        let vertices = self.get_chunk_vertices(chunk_x, chunk_y);
        //Number of vertices is equal to vertices.len() / VERTEX_LEN,
        //this is stored so that we know how many vertices to draw onto the
        //screen when we need to draw the chunk
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
                (STRIDE * size_of::<f32>()) as i32,
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
                (STRIDE * size_of::<f32>()) as i32,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

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
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                (STRIDE * size_of::<f32>()) as i32,
                (5 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
        }
    }

    //Builds the chunk meshes for each chunk of tiles,
    //this is done to reduce OpenGL calls and reduce CPU usage
    pub fn build_chunks(&mut self) {
        //Generate vertex arrays and vertex buffers
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
            gl::GenBuffers(
                self.level_chunk_animation.len() as i32,
                self.level_chunk_animation.as_mut_ptr(),
            );
        }

        for chunk_x in 0..(self.width / CHUNK_SIZE + 1) {
            for chunk_y in 0..(self.height / CHUNK_SIZE + 1) {
                self.build_chunk(chunk_x, chunk_y);
            }
        }
    }

    //Display the level
    pub fn display(&self) {
        for i in 0..self.level_chunks.len() {
            unsafe {
                gl::BindVertexArray(self.level_chunks[i]);
                gl::DrawArrays(gl::TRIANGLES, 0, self.level_chunk_vertex_count[i] as i32);
            }
        }
    }

    //Display interactive tiles
    pub fn display_interactive_tiles(
        &self,
        cube_vao: &VertexArrayObject,
        shader_program: &ShaderProgram,
        player_position: &Vector2<f32>,
    ) {
        for tile in &self.interactive_tiles {
            if (tile.tile_y - player_position.y).abs() > SPRITE_RENDER_DISTANCE {
                continue;
            }

            match tile.tile_type {
                InteractiveTile::Gold => {
                    shader_program.uniform_vec2f("uTexOffset", 0.0, 2.0 / 8.0);

                    let transform_matrix = Matrix4::from_translation(cgmath::vec3(
                        tile.tile_x - 0.2,
                        tile.tile_y - 0.4,
                        0.0,
                    )) * Matrix4::from_nonuniform_scale(0.2, 0.15, 0.3);
                    shader_program.uniform_matrix4f("uTransform", &transform_matrix);
                    cube_vao.draw_arrays();

                    let transform_matrix = Matrix4::from_translation(cgmath::vec3(
                        tile.tile_x + 0.2,
                        tile.tile_y - 0.4,
                        0.0,
                    )) * Matrix4::from_nonuniform_scale(0.2, 0.15, 0.3);
                    shader_program.uniform_matrix4f("uTransform", &transform_matrix);
                    cube_vao.draw_arrays();

                    let transform_matrix = Matrix4::from_translation(cgmath::vec3(
                        tile.tile_x,
                        tile.tile_y - 0.1,
                        0.0,
                    )) * Matrix4::from_nonuniform_scale(0.2, 0.15, 0.3);
                    shader_program.uniform_matrix4f("uTransform", &transform_matrix);
                    cube_vao.draw_arrays();
                }
                InteractiveTile::SmallGold => {
                    shader_program.uniform_vec2f("uTexOffset", 0.0, 2.0 / 8.0);

                    let transform_matrix = Matrix4::from_translation(cgmath::vec3(
                        tile.tile_x,
                        tile.tile_y - 0.4,
                        0.0,
                    )) * Matrix4::from_nonuniform_scale(0.2, 0.15, 0.2);
                    shader_program.uniform_matrix4f("uTransform", &transform_matrix);
                    cube_vao.draw_arrays();
                }
            }
        }
    }
}
