use super::Tile;
use super::ROOM_SIZE;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Read;

/*
 * There are 4 types of room templates:
 * - Starting room -> this is the room the player starts in
 * - Hallway -> contains enemies and treasure
 * - Vertical -> this is the room that will lead the player to the next floor
 * */

pub struct RoomTemplate {
    tiles: [Tile; (ROOM_SIZE * ROOM_SIZE) as usize],
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
        _ => None,
    }
}

impl RoomTemplate {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        match File::open(path) {
            Ok(mut file) => {
                let mut template = RoomTemplate {
                    tiles: [Tile::Air; (ROOM_SIZE * ROOM_SIZE) as usize],
                };

                let mut buf = Vec::new();
                file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

                let mut x = 0;
                let mut y = ROOM_SIZE - 1;
                for ch in &buf {
                    if !ch.is_ascii_graphic() {
                        continue;
                    }

                    template.set_tile(x, y, ascii_to_tile(*ch).unwrap_or(Tile::Air));

                    x += 1;
                    if x >= ROOM_SIZE && y > 0 {
                        x = 0;
                        y -= 1;
                    }
                }

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
}

fn load_room_template_from_direntry(
    template_path: DirEntry,
    room_template_list: &mut Vec<RoomTemplate>,
) {
    if let Some(path) = template_path.path().to_str() {
        match RoomTemplate::load_from_file(path) {
            Ok(template) => {
                room_template_list.push(template);
            }
            Err(msg) => {
                eprintln!("{msg}");
            }
        }
    }
}

pub fn load_room_templates(path: &str) -> Vec<RoomTemplate> {
    let mut template_list = Vec::new(); 

    match std::fs::read_dir(path) {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(template_path) => {
                        load_room_template_from_direntry(template_path, &mut template_list);
                    }
                    Err(msg) => {
                        eprintln!("{msg}");
                    }
                }
            }
        }
        Err(msg) => {
            eprintln!("failed to open: {path}");
            eprintln!("{msg}");
        }
    }

    template_list
}
