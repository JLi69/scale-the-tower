use super::BackgroundTile;
use super::Tile;
use super::ROOM_SIZE;
use core::slice::Iter;
use std::fs::File;
use std::io::Read;
use crate::gfx::load_image_pixels;

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
    Pickup,
}

pub struct Spawn {
    pub spawn_type: SpawnType,
    pub tile_x: u32,
    pub tile_y: u32,
}

pub struct RoomTemplate {
    tiles: [Tile; (ROOM_SIZE * ROOM_SIZE) as usize],
    background_tiles: [BackgroundTile; (ROOM_SIZE * ROOM_SIZE) as usize],
    spawns: Vec<Spawn>,
}

//Attempts to convert a pixel
//to a tile, if the pixel is not
//recognized then this function returns None
//pixels are of the format alpha, b, g, r
fn pixel_to_tile(pixel: u32) -> Option<Tile> {
    match pixel {
        0x00000000 => Some(Tile::Air),
        0xffaaaaaa => Some(Tile::Brick),
        0xff002848 => Some(Tile::Ladder),
        0xff666666 => Some(Tile::BrickTile),
        0xff444444 => Some(Tile::BrickTile2),
        0xff0000ff => Some(Tile::Lava),
        0xffffffff => Some(Tile::Spikes),
        _ => None,
    }
}

fn pixel_to_spawn(pixel: u32) -> Option<SpawnType> {
    match pixel {
        0xff008888 => Some(SpawnType::MaybeTreasure),
        0xff00ffff => Some(SpawnType::Treasure),
        0xff008800 => Some(SpawnType::MaybeEnemy),
        0xff00ff00 => Some(SpawnType::Enemy),
        0xffff00ff => Some(SpawnType::Pickup),
        _ => None,
    }
}

fn pixel_to_background_tile(pixel: u32) -> Option<BackgroundTile> {
    match pixel {
        0xffff4848 => Some(BackgroundTile::BannerTop),
        0xff222222 => Some(BackgroundTile::SkullDecoration),
        0xffffff00 => Some(BackgroundTile::Window),
        0xff888800 => Some(BackgroundTile::BarredWindow),
        0xffff8800 => Some(BackgroundTile::Painting1),
        0xffffaa66 => Some(BackgroundTile::BigWindowTop),
        _ => None,
    }
}

impl RoomTemplate {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let mut template = RoomTemplate {
            tiles: [Tile::Air; (ROOM_SIZE * ROOM_SIZE) as usize],
            background_tiles: [BackgroundTile::Wall; (ROOM_SIZE * ROOM_SIZE) as usize],
            spawns: Vec::new(),
        };

        let (buf, _) = load_image_pixels(path)?;

        let mut x = 0;
        let mut y = ROOM_SIZE - 1;

        buf.iter()
            .for_each(|pixel| {
                template.set_tile(x, y, pixel_to_tile(*pixel).unwrap_or(Tile::Air));
                template.set_background_tile(
                    x,
                    y,
                    pixel_to_background_tile(*pixel).unwrap_or(BackgroundTile::Wall),
                );

                if template.get_background_tile(x, y + 1) == BackgroundTile::BannerTop {
                    template.set_background_tile(x, y, BackgroundTile::BannerBottom);
                } else if template.get_background_tile(x, y + 1)
                    == BackgroundTile::BigWindowTop
                {
                    template.set_background_tile(x, y, BackgroundTile::BigWindowBottom);
                }

                if let Some(t) = pixel_to_spawn(*pixel) {
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

    pub fn get_background_tile(&self, x: u32, y: u32) -> BackgroundTile {
        if x >= ROOM_SIZE || y >= ROOM_SIZE {
            return BackgroundTile::Empty;
        }

        self.background_tiles[(y * ROOM_SIZE + x) as usize]
    }

    pub fn set_background_tile(&mut self, x: u32, y: u32, background_tile: BackgroundTile) {
        if x >= ROOM_SIZE || y >= ROOM_SIZE {
            return;
        }

        self.background_tiles[(y * ROOM_SIZE + x) as usize] = background_tile;
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
            if let Err(msg) = file.read_to_string(&mut buf) {
                eprintln!("{msg}");
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
                background_tiles: [BackgroundTile::Empty; (ROOM_SIZE * ROOM_SIZE) as usize],
                spawns: Vec::new(),
            })
        })
        .collect()
}
