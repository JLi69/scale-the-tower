use crate::sprite::enemy::{Enemy, EnemyType};

use super::{
    room_template::RoomTemplate, room_template::SpawnType, InteractiveTile, InteractiveTileSprite,
    Level, Tile, ROOM_SIZE,
};
use rand::{rngs::ThreadRng, Rng};

impl Level {
    fn empty_room(&mut self, room_x: u32, room_y: u32) {
        for x in 0..ROOM_SIZE {
            for y in 0..ROOM_SIZE {
                let tile_x = x + room_x * (ROOM_SIZE + 1) + 1;
                let tile_y = y + room_y * (ROOM_SIZE + 1) + 1;
                self.set_tile(tile_x, tile_y, Tile::Air);
            }
        }
    }

    fn generate_room_from_template(
        &mut self,
        templates: &Vec<RoomTemplate>,
        rng: &mut ThreadRng,
        room_x: u32,
        room_y: u32,
    ) {
        if templates.is_empty() {
            return;
        }

        let random_template = &templates[rng.gen::<usize>() % templates.len()];

        for x in 0..ROOM_SIZE {
            for y in 0..ROOM_SIZE {
                let tile_x = x + room_x * (ROOM_SIZE + 1) + 1;
                let tile_y = y + room_y * (ROOM_SIZE + 1) + 1;
                self.set_tile(tile_x, tile_y, random_template.get_tile(x, y));
            }
        }

        for spawn_location in random_template.get_spawns() {
            match spawn_location.spawn_type {
                SpawnType::MaybeTreasure => {
                    let rand_number = rng.gen::<u32>() % 100;

                    if rand_number < 10 {
                        self.interactive_tiles.push(InteractiveTileSprite {
                            tile_type: InteractiveTile::Gold,
                            tile_x: (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            tile_y: (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                        });
                    } else if rand_number < 50 {
                        self.interactive_tiles.push(InteractiveTileSprite {
                            tile_type: InteractiveTile::SmallGold,
                            tile_x: (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            tile_y: (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                        });
                    }
                }
                SpawnType::Treasure => {
                    self.interactive_tiles.push(InteractiveTileSprite {
                        tile_type: InteractiveTile::Gold,
                        tile_x: (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                        tile_y: (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                    });
                }
                SpawnType::MaybeEnemy => {
                    //Spawn enemy
                }
                SpawnType::Enemy => {
                    //Spawn enemy
                    self.enemies.push(Enemy::new(
                        (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                        (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                        1.0,
                        1.0,
                        EnemyType::Slime,
                    ));
                }
            }
        }
    }

    pub fn generate_level(template_list: &Vec<RoomTemplate>) -> Self {
        let floors = 48;
        let mut level = Self::new(18, ROOM_SIZE * floors + floors + 1);

        let mut rng = rand::thread_rng();

        for room_y in 0..floors {
            if room_y < floors - 1 {
                level.generate_room_from_template(template_list, &mut rng, 0, room_y);
            } else {
                level.empty_room(0, room_y);
            }

            ((ROOM_SIZE / 2 - 1)..(ROOM_SIZE / 2 + 3))
                .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));
        }

        level
    }
}
