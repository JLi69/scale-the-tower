use std::mem::size_of;
use std::os::raw::c_void;
use rand::Rng;

//The distance of the level from the camera
pub const LEVEL_Z: f32 = -8.0;
//There will be a maximum of TEXTURE_SCALE * TEXTURE_SCALE
//different tile textures that will be stored in a single texture
const TEXTURE_SCALE: f32 = 8.0;
//Size of a "chunk" of tiles in the level
//This is the size of a square of tiles that will
//be grouped together into a single mesh so that
//we can reduce the number of OpenGL calls made
//when drawing the level
const CHUNK_SIZE: u32 = 16;
//Size of a room
const ROOM_SIZE: u32 = 16;
//Number of f32 elements in a vertex
const VERTEX_LEN: usize = 5;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Tile {
    Air,
    Brick,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum BackgroundTile {
    Empty,
    Wall,
}

pub struct Level {
    tiles: Vec<Tile>,
    background_tiles: Vec<BackgroundTile>,
    width: u32,
    height: u32,

    level_chunks: Vec<u32>,
    level_chunk_vertex_buffers: Vec<u32>,
    level_chunk_texture_coordinates: Vec<u32>,
    level_chunk_vertex_count: Vec<u32>,
}

impl Level {
    pub fn new(w: u32, h: u32) -> Self {
        //Creates a level filled with bricks that can
        //be used to generate a more complex level
        Self {
            tiles: vec![Tile::Brick; w as usize * h as usize],
            background_tiles: vec![BackgroundTile::Wall; w as usize * h as usize],
            width: w,
            height: h,

            level_chunks: vec![
                0;
                ((w / CHUNK_SIZE) as usize + 1) * ((h / CHUNK_SIZE) as usize + 1)
            ],
            level_chunk_vertex_buffers: vec![
                0;
                ((w / CHUNK_SIZE) as usize + 1)
                    * ((h / CHUNK_SIZE) as usize + 1)
            ],
            level_chunk_texture_coordinates: vec![
                0;
                ((w / CHUNK_SIZE) as usize + 1)
                    * ((h / CHUNK_SIZE) as usize + 1)
            ],
            level_chunk_vertex_count: vec![
                0;
                ((w / CHUNK_SIZE) as usize + 1)
                    * ((h / CHUNK_SIZE) as usize + 1)
            ],
        }
    }

    fn create_room(&mut self, room_x: u32, room_y: u32) {
        for x in room_x..(room_x + ROOM_SIZE) {
            for y in room_y..(room_y + ROOM_SIZE) {
                self.set_tile(x, y, Tile::Air); 
            }
        }
    }

    pub fn generate_level() -> Self {
        let mut level = Self::new(69, 69);

        for room_x in 0..4 {
            for room_y in 0..4 {
                level.create_room(room_x * (ROOM_SIZE + 1) + 1, room_y * (ROOM_SIZE + 1) + 1);
            }
        }
        
        let mut rng = rand::thread_rng();

        for room_y in 0..4 {
            for room_x in 0..3 { 
                if rng.gen::<u32>() % 4 == 0 { 
                    //Randomly combine rooms
                    (((room_y) * (ROOM_SIZE + 1) + 1)..
                    ((room_y) * (ROOM_SIZE + 1) + ROOM_SIZE + 1))
                        .for_each(|y| level.set_tile((room_x + 1) * (ROOM_SIZE + 1), y, Tile::Air));
                } else {
                    //Create doorways between rooms
                    ((room_y * (ROOM_SIZE + 1) + 1)..((room_y * (ROOM_SIZE + 1) + 4)))
                        .for_each(|y| level.set_tile((room_x + 1) * (ROOM_SIZE + 1), y, Tile::Air));
                } 
            }

            //Create an exit that will bring the player to the next floor
            let exit_x = if room_y == 0 {
                rng.gen::<u32>() % 3 + 1 
            } else {
                rng.gen::<u32>() % 4
            };

            (((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 - 1)..
            ((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 + 3))
                .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));

            //Attempt to generate a second possible exit to the next floor
            if rng.gen::<u32>() % 2 == 0 && room_y > 0 && room_y < 3 {
                let exit_x = rng.gen::<u32>() % 4;

                (((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 - 1)..
                ((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 + 3))
                    .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));
            }
        } 
        
        level
    }

    //Adds the vertices of a single face to the vertex vector
    fn add_vertices(&self, x: u32, y: u32, face: &[f32; 30], vertices: &mut Vec<f32>) {
        //6 vertices per face
        for i in 0..6 {
            //Add the vertex position (x, y, z)
            vertices.push(face[i * VERTEX_LEN] + 2.0 * x as f32);
            vertices.push(face[i * VERTEX_LEN + 1] + 2.0 * y as f32);
            vertices.push(face[i * VERTEX_LEN + 2]);

            //Add the texture coordinates
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

        //Check if the other faces are covered so that we don't add more
        //vertices than we need to. The out of bounds check is to make sure
        //that x and y don't underflow when subtracting 1 from them and avoid
        //causing a crash in debug mode
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

        if self.get_tile(x, y) != Tile::Air || self.get_background_tile(x, y) == BackgroundTile::Empty {
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
        }
    }

    //Builds a vector of vertices for a single chunk
    pub fn get_chunk_vertices(&self, chunk_x: u32, chunk_y: u32) -> Vec<f32> {
        let mut vertices = vec![];

        for x in (chunk_x * CHUNK_SIZE)..(chunk_x * CHUNK_SIZE + CHUNK_SIZE) {
            for y in (chunk_y * CHUNK_SIZE)..(chunk_y * CHUNK_SIZE + CHUNK_SIZE) {
                self.add_tile_vertices(x, y, &mut vertices);
            }
        }

        for x in (chunk_x * CHUNK_SIZE)..(chunk_x * CHUNK_SIZE + CHUNK_SIZE) {
            for y in (chunk_y * CHUNK_SIZE)..(chunk_y * CHUNK_SIZE + CHUNK_SIZE) {
                self.add_background_vertices(x, y, &mut vertices);
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
        }

        for chunk_x in 0..(self.width / CHUNK_SIZE + 1) {
            for chunk_y in 0..(self.height / CHUNK_SIZE + 1) {
                self.build_chunk(chunk_x, chunk_y);
            }
        }
    }

    //Returns true if the integer coordinates are outside of the level
    //(level coordinates range from 0 to width - 1 for x and
    //range from 0 to height - 1 for y)
    pub fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        //Out of bounds, return Air as the default
        if x >= self.width || y >= self.height {
            return Tile::Air;
        }

        self.tiles[((self.width * y) + x) as usize]
    }

    pub fn get_background_tile(&self, x: u32, y: u32) -> BackgroundTile {
        //Out of bounds, return Air as the default
        if x >= self.width || y >= self.height {
            return BackgroundTile::Empty;
        }

        self.background_tiles[((self.width * y) + x) as usize]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        //Out of bounds, ignore
        if x >= self.width || y >= self.height {
            return;
        }

        self.tiles[((self.width * y) + x) as usize] = tile;
    }

    pub fn set_background_tile(&mut self, x: u32, y: u32, tile: BackgroundTile) {
        //Out of bounds, ignore
        if x >= self.width || y >= self.height {
            return;
        }

        self.background_tiles[((self.width * y) + x) as usize] = tile;
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
}

impl Drop for Level {
    fn drop(&mut self) {
        //When the level gets dropped, make sure to delete all of the
        //vertex array objects and all of the vertex buffers
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
