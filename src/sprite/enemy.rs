use super::Sprite;
use crate::{
    game::GRAVITY, gfx::VertexArrayObject, level::transparent, level::Level, shader::ShaderProgram,
};
use cgmath::{vec2, Matrix4};

pub enum EnemyType {
    Slime,
}

pub struct Enemy {
    pub sprite: Sprite,
    pub enemy_type: EnemyType,
    falling: bool,
}

impl Enemy {
    //Create a new enemy
    pub fn new(x: f32, y: f32, w: f32, h: f32, enemy: EnemyType, flipped: bool) -> Self {
        let mut spr = Sprite::new(x, y, w, h);

        match enemy {
            EnemyType::Slime => spr.set_animation(0.5, 0, 1),
        }

        match enemy {
            EnemyType::Slime => spr.velocity.x = 0.5,
        }

        spr.flipped = flipped;
        if spr.flipped {
            spr.velocity.x *= -1.0;
        }

        Self {
            sprite: spr,
            enemy_type: enemy,
            falling: false,
        }
    }

    pub fn display(&self, rect_vao: &VertexArrayObject, shader_program: &ShaderProgram) {
        shader_program.uniform_bool("uFlipped", self.sprite.flipped);

        let transform_matrix = Matrix4::from_translation(cgmath::vec3(
            self.sprite.position.x,
            self.sprite.position.y,
            0.0,
        )) * Matrix4::from_scale(0.5);
        shader_program.uniform_matrix4f("uTransform", &transform_matrix);

        //Apply texture
        match self.enemy_type {
            EnemyType::Slime => {
                shader_program.uniform_vec2f(
                    "uTexOffset",
                    1.0 / 8.0 * self.sprite.current_frame() as f32,
                    1.0 / 8.0,
                );
            }
        }

        rect_vao.draw_arrays();
    }

    //Handle collision in the y axis
    fn handle_collision_y(&mut self, sprite: &Sprite) {
        if self.sprite.intersecting(sprite) {
            if self.sprite.position.y > sprite.position.y {
                //If we are supported by a tile then stop falling
                self.falling = false;
                self.sprite.velocity.y = -0.01;
                self.falling = false;
            } else if self.sprite.position.y < sprite.position.y {
                //Set y velocity to 0 so we don't "stick" to the tile if the
                //player decides to hold down the jump key
                self.sprite.velocity.y = 0.0;
                //We hit the bottom of a tile, start falling again
                self.falling = true;
            }
        }
    }

    fn update_slime(&mut self, dt: f32, level: &Level) {
        self.sprite.position.x += self.sprite.velocity.x * dt;
        //Handle collision
        let top_left = vec2(self.sprite.position.x, self.sprite.position.y)
            - vec2(
                self.sprite.dimensions.x.ceil() / 2.0 + 1.0,
                self.sprite.dimensions.y.ceil() / 2.0 + 1.0,
            );
        let bot_right = vec2(self.sprite.position.x, self.sprite.position.y)
            + vec2(
                self.sprite.dimensions.x.ceil() / 2.0 + 1.0,
                self.sprite.dimensions.y.ceil() / 2.0 + 1.0,
            );
        let (top_left_x, top_left_y) = (top_left.x.floor() as i32, top_left.y.floor() as i32);
        let (bot_right_x, bot_right_y) = (bot_right.x.ceil() as i32, bot_right.y.ceil() as i32);

        //Scan the level for tiles the sprite might have collided with
        //and then uncollide the sprite from the tiles
        let mut collided = false;
        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if !transparent(level.get_tile(x as u32, y as u32))
                    || (!level.out_of_bounds(x, y - 1)
                        && transparent(level.get_tile(x as u32, y as u32 - 1)))
                {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    if self.sprite.intersecting(&hitbox) {
                        collided = true;
                    }
                    self.sprite.uncollide_x(&hitbox);
                }
            }
        }

        if collided {
            self.sprite.velocity.x *= -1.0;
            self.sprite.position.x += self.sprite.velocity.x * dt;
        }

        self.sprite.position.y += self.sprite.velocity.y * dt * 0.5;
        if self.falling {
            self.sprite.velocity.y += GRAVITY * 0.5;
        }
        self.sprite.position.y += self.sprite.velocity.y * dt * 0.5;

        //Uncollide from any tiles and also determine if the sprite is falling
        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if !transparent(level.get_tile(x as u32, y as u32)) {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    self.handle_collision_y(&hitbox);
                    self.sprite.uncollide_y(&hitbox);
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32, level: &Level) {
        match self.enemy_type {
            EnemyType::Slime => self.update_slime(dt, level),
        }
    }

    pub fn get_damage(&self) -> i32 {
        match self.enemy_type {
            EnemyType::Slime => 1, 
        }
    }
}
