use crate::{
    game::GRAVITY,
    gfx::VertexArrayObject,
    level::{transparent, Level},
    shader::ShaderProgram,
};
use cgmath::{vec2, vec3, Matrix4};

use super::Sprite;

#[derive(Copy, Clone)]
pub enum ParticleType {
    Blood,
    Fire,
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub sprite: Sprite,
    pub particle_type: ParticleType,
    pub timer: f32,
    falling: bool,
}

impl Particle {
    pub fn new(x: f32, y: f32, sz: f32, speed: f32, angle: f32, particle: ParticleType) -> Self {
        let mut spr = Sprite::new(x, y, sz, sz);
        spr.velocity = vec2(speed * angle.cos(), speed * angle.sin());

        let time = match particle {
            ParticleType::Blood => 3.0,
            ParticleType::Fire => 1.0,
        };

        Self {
            sprite: spr,
            particle_type: particle,
            timer: time,
            falling: true,
        }
    }

    //Handle collision in the y axis
    fn handle_collision_y(&mut self, sprite: &Sprite) {
        if self.sprite.intersecting(sprite) {
            if self.sprite.position.y > sprite.position.y {
                //If we are supported by a tile then stop falling
                self.falling = false;
                self.sprite.velocity.y = -0.01;
                self.sprite.velocity.x = 0.0;
            } else if self.sprite.position.y < sprite.position.y {
                //Set y velocity to 0 so we don't "stick" to the tile if the
                //player decides to hold down the jump key
                self.sprite.velocity.y = 0.0;
                //We hit the bottom of a tile, start falling again
                self.falling = true;
            }
        }
    }

    pub fn update(&mut self, level: &Level, dt: f32) {
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
        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if !transparent(level.get_tile(x as u32, y as u32)) {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    if self.sprite.intersecting(&hitbox) {
                        self.sprite.velocity.x = 0.0;
                    }
                    self.sprite.uncollide_x(&hitbox);
                }
            }
        }

        self.sprite.position.y += self.sprite.velocity.y * dt * 0.5;
        if self.falling {
            self.sprite.velocity.y -= GRAVITY * dt;
        }
        self.sprite.position.y += self.sprite.velocity.y * dt * 0.5;

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

        //Uncollide from any tiles and also determine if the sprite is falling
        self.falling = true;
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

        self.timer -= dt;
    }

    pub fn display(&self, rect_vao: &VertexArrayObject, shader_program: &ShaderProgram) {
        let transform_matrix =
            Matrix4::from_translation(vec3(self.sprite.position.x, self.sprite.position.y, 0.0))
                * Matrix4::from_nonuniform_scale(
                    0.5 * self.sprite.dimensions.x,
                    0.5 * self.sprite.dimensions.y,
                    0.5,
                );
        shader_program.uniform_matrix4f("uTransform", &transform_matrix);

        match self.particle_type {
            ParticleType::Blood => {
                shader_program.uniform_vec2f("uTexOffset", 0.0, 4.0 / 8.0);
            }
            ParticleType::Fire => {
                shader_program.uniform_vec2f("uTexOffset", 1.0 / 8.0, 4.0 / 8.0);
            }
        }

        rect_vao.draw_arrays();
    }
}
