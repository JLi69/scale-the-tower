use super::{Level, ROOM_SIZE, Tile, room_template::{TemplateList, RoomTemplate}};
use rand::{Rng, rngs::ThreadRng};

impl Level {
    fn create_room(&mut self, room_x: u32, room_y: u32) {
        for x in (room_x * (ROOM_SIZE + 1) + 1)..(room_x * (ROOM_SIZE + 1) + 1 + ROOM_SIZE) {
            for y in (room_y * (ROOM_SIZE + 1) + 1)..(room_y * (ROOM_SIZE + 1) + 1 + ROOM_SIZE) {
                self.set_tile(x, y, Tile::Air);
            }
        }
    }

    fn generate_room_from_template(
        &mut self, 
        templates: &Vec<RoomTemplate>, 
        rng: &mut ThreadRng, 
        room_x: u32, 
        room_y: u32
    ) {
        if templates.len() == 0 {
            return; 
        }

        let random_template = 
            &templates[rng.gen::<usize>() % templates.len()];

        for x in 0..ROOM_SIZE {
            for y in 0..ROOM_SIZE {
                let tile_x = x + room_x * (ROOM_SIZE + 1) + 1;
                let tile_y = y + room_y * (ROOM_SIZE + 1) + 1;
                self.set_tile(tile_x, tile_y, random_template.get_tile(x, y)); 
            }
        } 
    }

    pub fn generate_level(template_list: &TemplateList) -> Self {
        let mut level = Self::new(69, 69);

        for room_x in 0..4 {
            for room_y in 0..4 {
                level.create_room(room_x, room_y);
            }
        }

        let mut rng = rand::thread_rng();

        level.generate_room_from_template(&template_list.starting_room_templates, &mut rng, 0, 0);

        for room_y in 0..4 {
            //Create an exit that will bring the player to the next floor
            let exit_x = if room_y == 0 {
                rng.gen::<u32>() % 3 + 1
            } else {
                rng.gen::<u32>() % 4
            };

            for room_x in 0..4 { 
                if room_x == exit_x { 
                    level.generate_room_from_template(&template_list.vertical_room_templates, &mut rng, room_x, room_y);         
                } else if room_x != 0 || room_y != 0 { 
                    level.generate_room_from_template(&template_list.hallway_templates, &mut rng, room_x, room_y);         
                }

                if room_x == 3 {
                    continue; 
                }

                if rng.gen::<u32>() % 4 == 0 {
                    //Randomly combine rooms
                    (((room_y) * (ROOM_SIZE + 1) + 1)
                        ..((room_y) * (ROOM_SIZE + 1) + ROOM_SIZE + 1))
                        .for_each(|y| level.set_tile((room_x + 1) * (ROOM_SIZE + 1), y, Tile::Air));
                } else {
                    //Create doorways between rooms
                    ((room_y * (ROOM_SIZE + 1) + 1)..(room_y * (ROOM_SIZE + 1) + 4))
                        .for_each(|y| level.set_tile((room_x + 1) * (ROOM_SIZE + 1), y, Tile::Air));
                }
            } 

            (((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 - 1)
                ..((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 + 3))
                .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));

            /*
            //Attempt to generate a second possible exit to the next floor
            if rng.gen::<u32>() % 2 == 0 && room_y > 0 && room_y < 3 {
                let exit_x = rng.gen::<u32>() % 4;

                (((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 - 1)
                    ..((exit_x) * (ROOM_SIZE + 1) + ROOM_SIZE / 2 + 3))
                    .for_each(|x| level.set_tile(x, (room_y + 1) * (ROOM_SIZE + 1), Tile::Air));
             
                level.generate_room_from_template(&template_list.vertical_room_templates, &mut rng, exit_x, room_y);         
            }*/
        }

        level
    }
}
