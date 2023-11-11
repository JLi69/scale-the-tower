use super::{room_template::RoomTemplate, Level, Tile, ROOM_SIZE};
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
    }

    pub fn generate_level(template_list: &Vec<RoomTemplate>) -> Self {
        let floors = 64;
        let mut level = Self::new(18, ROOM_SIZE * floors + floors + 1);

        let mut rng = rand::thread_rng();

        level.generate_room_from_template(template_list, &mut rng, 0, 0);

        for room_y in 0..floors {
            level.generate_room_from_template(template_list, &mut rng, 0, room_y);

            ((ROOM_SIZE / 2 - 1)..(ROOM_SIZE / 2 + 3))
                .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));
        }

        level
    }
}
