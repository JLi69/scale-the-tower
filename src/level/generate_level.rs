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
        enemies: &mut Vec<Enemy>,
        template: &RoomTemplate,
        rng: &mut ThreadRng,
        room_x: u32,
        room_y: u32,
    ) {
        for x in 0..ROOM_SIZE {
            for y in 0..ROOM_SIZE {
                let tile_x = x + room_x * (ROOM_SIZE + 1) + 1;
                let tile_y = y + room_y * (ROOM_SIZE + 1) + 1;
                self.set_tile(tile_x, tile_y, template.get_tile(x, y));
            }
        }

        for spawn_location in template.get_spawns() {
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
                    let rand_value = rng.gen::<u32>() % 100;
                    let flipped = rng.gen::<bool>();

                    if rand_value < 30 {
                        enemies.push(Enemy::new(
                            (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                            0.9,
                            1.0,
                            EnemyType::Slime,
                            flipped,
                        ));
                    } else if rand_value < 40 {
                        enemies.push(Enemy::new(
                            (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                            0.9,
                            1.0,
                            EnemyType::Eyeball,
                            flipped,
                        ));
                    } else if rand_value < 50 {
                        enemies.push(Enemy::new(
                            (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                            0.9,
                            1.0,
                            EnemyType::Chicken,
                            flipped,
                        ));
                    }
                }
                SpawnType::Enemy => {
                    let rand_value = rng.gen::<u32>() % 100;
                    let flipped = rng.gen::<bool>();
                    //Spawn enemy
                    if rand_value < 20 {
                        enemies.push(Enemy::new(
                            (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                            0.9,
                            1.0,
                            EnemyType::Chicken,
                            flipped,
                        ));
                    } else if rand_value < 40 {
                        enemies.push(Enemy::new(
                            (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                            0.9,
                            1.0,
                            EnemyType::Chicken,
                            flipped,
                        ));
                    } else {
                        enemies.push(Enemy::new(
                            (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
                            (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
                            0.9,
                            1.0,
                            EnemyType::Chicken,
                            flipped,
                        ));
                    }
                }
            }
        }
    }

    pub fn generate_level(template_list: &Vec<RoomTemplate>) -> (Self, Vec<Enemy>) {
        let floors = 48;
        let mut level = Self::new(18, ROOM_SIZE * floors + floors + 1);
        let mut enemies = Vec::<Enemy>::new();

        let mut rng = rand::thread_rng();

        for room_y in 0..floors {
            if template_list.is_empty() {
                level.empty_room(0, room_y);
                continue;
            }

            let random_template = &template_list[rng.gen::<usize>() % template_list.len()];

            level.generate_room_from_template(&mut enemies, random_template, &mut rng, 0, room_y);

            ((ROOM_SIZE / 2 - 1)..(ROOM_SIZE / 2 + 3))
                .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));
        }

        (level, enemies)
    }
}
