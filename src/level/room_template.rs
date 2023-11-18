use super::Tile;
use super::ROOM_SIZE;
use core::slice::Iter;
use std::fs::File;
use std::io::Read;

/*
 * There are 4 types of room templates:
 * - Starting room -> this is the room the player starts in
 * - Hallway -> contains enemies and treasure
 * - Vertical -> this is the room that will lead the player to the next floor
 * */

pub enum SpawnType {
    MaybeTreasure,
    Treasure,
    MaybeEnemy,
    Enemy,
}

pub struct Spawn {
    pub spawn_type: SpawnType,
    pub tile_x: u32,
    pub tile_y: u32,
}

pub struct RoomTemplate {
    tiles: [Tile; (ROOM_SIZE * ROOM_SIZE) as usize],
    spawns: Vec<Spawn>,
}

//Attempts to convert an ascii character
//to a tile, if the character is not
//recognized then this function returns None
fn ascii_to_tile(ch: u8) -> Option<Tile> {
    match ch {
        b'#' => Some(Tile::Brick),
        b'.' => Some(Tile::Air),
        b'|' => Some(Tile::Ladder),
        b'@' => Some(Tile::BrickTile),
        b'=' => Some(Tile::BrickTile2),
        b'!' => Some(Tile::Lava),
        b'^' => Some(Tile::Spikes),
        _ => None,
    }
}

fn ascii_to_spawn(ch: u8) -> Option<SpawnType> {
    match ch {
        b'g' => Some(SpawnType::MaybeTreasure),
        b'G' => Some(SpawnType::Treasure),
        b'e' => Some(SpawnType::MaybeEnemy),
        b'E' => Some(SpawnType::Enemy),
        _ => None,
    }
}

impl RoomTemplate {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        match File::open(path) {
            Ok(mut file) => {
                let mut template = RoomTemplate {
                    tiles: [Tile::Air; (ROOM_SIZE * ROOM_SIZE) as usize],
                    spawns: Vec::new(),
                };

                let mut buf = [0u8; ((ROOM_SIZE + 1) * ROOM_SIZE) as usize];
                file.read(&mut buf).map_err(|e| e.to_string())?;

                let mut x = 0;
                let mut y = ROOM_SIZE - 1;

                buf.iter()
                    .filter(|ch| ch.is_ascii_graphic())
                    .for_each(|ch| {
                        template.set_tile(x, y, ascii_to_tile(*ch).unwrap_or(Tile::Air));
                        x += 1;
                        if x >= ROOM_SIZE && y > 0 {
                            x = 0;
                            y -= 1;
                        }
                    });

                x = 0;
                y = ROOM_SIZE - 1;

                buf.iter()
                    .filter(|ch| ch.is_ascii_graphic())
                    .for_each(|ch| {
                        if let Some(t) = ascii_to_spawn(*ch) {
                            template.spawns.push(Spawn {
                                spawn_type: t,
                                tile_x: x,
                                tile_y: y,
                            })
                        }

                        x += 1;
                        if x >= ROOM_SIZE && y > 0 {
                            x = 0;
                            y -= 1;
                        }
                    });

                Ok(template)
            }
            Err(msg) => {
                eprintln!("Failed to open: {path}");
                eprintln!("{msg}");
                Err(msg.to_string())
            }
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        if x >= ROOM_SIZE || y >= ROOM_SIZE {
            return Tile::Air;
        }

        self.tiles[(y * ROOM_SIZE + x) as usize]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        if x >= ROOM_SIZE || y >= ROOM_SIZE {
            return;
        }

        self.tiles[(y * ROOM_SIZE + x) as usize] = tile;
    }

    pub fn get_spawns(&self) -> Iter<Spawn> {
        self.spawns.iter()
    }
}

pub fn load_room_templates(path: &str) -> Vec<RoomTemplate> {
    let mut template_list_path = String::from(path);
    template_list_path.push_str("/template_list.txt");

    let paths: Vec<String> = match File::open(&template_list_path) {
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(sz) => eprintln!("read {sz} bytes from {template_list_path}"),
                Err(msg) => eprintln!("{msg}"),
            }

            buf.lines()
                .map(|line| {
                    let mut path = String::from(path);
                    path.push('/');
                    path.push_str(line);
                    path
                })
                .collect()
        }
        Err(msg) => {
            eprintln!("failed to open: {template_list_path}");
            eprintln!("{msg}");
            return Vec::new(); //return empty room template list
        }
    };

    paths
        .iter()
        .map(|path| RoomTemplate::load_from_file(path))
        .filter(|template_res| template_res.is_ok())
        .map(|template_res| {
            template_res.unwrap_or(RoomTemplate {
                tiles: [Tile::Air; (ROOM_SIZE * ROOM_SIZE) as usize],
                spawns: Vec::new(),
            })
        })
        .collect()
}
