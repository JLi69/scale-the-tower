pub mod display_level;
pub mod room_template;
pub mod generate_level;

//The distance of the level from the camera
pub const LEVEL_Z: f32 = -8.0;
//Size of a "chunk" of tiles in the level
//This is the size of a square of tiles that will
//be grouped together into a single mesh so that
//we can reduce the number of OpenGL calls made
//when drawing the level
const CHUNK_SIZE: u32 = 16;
//Size of a room
const ROOM_SIZE: u32 = 16;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Tile {
    Air,
    Brick,
    Ladder,
    BrickTile,
    BrickTile2
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum BackgroundTile {
    Empty,
    Wall,
}

pub fn transparent(tile: Tile) -> bool {
    matches!(tile, Tile::Air | Tile::Ladder)
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
