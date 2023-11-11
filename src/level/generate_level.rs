use super::{room_template::RoomTemplate, Level, Tile, ROOM_SIZE, InteractiveTileSprite};
use rand::{rngs::ThreadRng, Rng};

impl Level {
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

        random_template.get_interactive_tile_spawns()
            .for_each(|spawn_interactive_tile| {
                self.interactive_tiles.push(InteractiveTileSprite { 
                    tile_type: spawn_interactive_tile.tile_type, 
                    tile_x: (spawn_interactive_tile.tile_x + 1 + room_x * (ROOM_SIZE + 1)) as f32, 
                    tile_y: (spawn_interactive_tile.tile_y + 1 + room_y * (ROOM_SIZE + 1)) as f32
                });
            });
    }

    pub fn generate_level(template_list: &Vec<RoomTemplate>) -> Self {
        let floors = 32;
        let mut level = Self::new(18, ROOM_SIZE * floors + floors + 1);

        let mut rng = rand::thread_rng();

        for room_y in 0..floors {
            level.generate_room_from_template(template_list, &mut rng, 0, room_y);

            ((ROOM_SIZE / 2 - 1)..(ROOM_SIZE / 2 + 3))
                .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));
        }

        level
    }
}
