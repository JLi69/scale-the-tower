use crate::sprite::enemy::{Enemy, EnemyType};

use super::{
    room_template::{RoomTemplate, Spawn, SpawnType},
    BackgroundTile, InteractiveTile, InteractiveTileSprite, Level, Tile, ROOM_SIZE,
};
use rand::{rngs::ThreadRng, Rng};

fn generate_enemy_type(rand_value: u32, weights: &[(u32, EnemyType)]) -> EnemyType {
    let mut total = 0;
    for (probability, enemy_type) in weights {
        total += probability;
        if total > rand_value {
            return *enemy_type;
        }
    }

    //Return a slime as the default enemy
    EnemyType::Slime
}

fn spawn_enemy(
    enemies: &mut Vec<Enemy>,
    rng: &mut ThreadRng,
    spawn_location: &Spawn,
    room_x: u32,
    room_y: u32,
) {
    let rand_value = rng.gen::<u32>() % 100;
    let flipped = rng.gen::<bool>();

    let enemy_type = if room_y < 4 {
        let weights = [(40, EnemyType::Eyeball)];
        generate_enemy_type(rand_value, &weights)
    } else if room_y < 8 {
        let weights = [(30, EnemyType::Eyeball), (20, EnemyType::Chicken)];
        generate_enemy_type(rand_value, &weights)
    } else if room_y < 12 {
        let weights = [
            (20, EnemyType::Eyeball),
            (30, EnemyType::Chicken),
            (30, EnemyType::Skeleton),
            (5, EnemyType::Demon),
        ];
        generate_enemy_type(rand_value, &weights)
    } else if room_y < 20 {
        let weights = [
            (15, EnemyType::Eyeball),
            (15, EnemyType::Chicken),
            (15, EnemyType::Skeleton),
            (15, EnemyType::Demon),
        ];
        generate_enemy_type(rand_value, &weights)
    } else if room_y < 28 {
        let weights = [
            (10, EnemyType::Eyeball),
            (10, EnemyType::Chicken),
            (20, EnemyType::Skeleton),
            (20, EnemyType::Demon),
        ];
        generate_enemy_type(rand_value, &weights)
    } else if room_y < 36 {
        let weights = [
            (5, EnemyType::Eyeball),
            (5, EnemyType::Chicken),
            (25, EnemyType::Skeleton),
            (25, EnemyType::Demon),
        ];
        generate_enemy_type(rand_value, &weights)
    } else {
        let weights = [
            (5, EnemyType::Eyeball),
            (5, EnemyType::Chicken),
            (30, EnemyType::Skeleton),
            (30, EnemyType::Demon),
        ];
        generate_enemy_type(rand_value, &weights)
    };

    //Spawn enemy
    enemies.push(Enemy::new(
        (spawn_location.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32,
        (spawn_location.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32,
        enemy_type,
        flipped,
    ));
}

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
                self.set_background_tile(tile_x, tile_y, template.get_background_tile(x, y));

                if self.get_background_tile(tile_x, tile_y) == BackgroundTile::Painting1
                    && rng.gen::<bool>()
                {
                    self.set_background_tile(tile_x, tile_y, BackgroundTile::Painting2);
                }
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
                    if rand_value < 60 {
                        spawn_enemy(enemies, rng, spawn_location, room_x, room_y);
                    }
                }
                SpawnType::Enemy => {
                    //Spawn enemy
                    spawn_enemy(enemies, rng, spawn_location, room_x, room_y);
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
