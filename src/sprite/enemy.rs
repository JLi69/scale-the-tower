use super::Sprite;
use crate::{
    game::DAMAGE_COOLDOWN, game::GRAVITY, gfx::VertexArrayObject, level::transparent, level::Level,
    shader::ShaderProgram,
};
use cgmath::{vec2, Matrix4, Vector2};

mod chicken;
mod eyeball;
mod slime;

const ENEMY_ATTACK_COOLDOWN: f32 = 1.0;

#[derive(PartialEq, Eq)]
enum EnemyState {
    Idle,
    Wander,
    Chase,
}

pub enum EnemyType {
    Slime,
    Eyeball,
    Chicken,
}

pub struct Enemy {
    pub sprite: Sprite,
    pub enemy_type: EnemyType,
    pub health: i32,
    falling: bool,
    damage_cooldown: f32,
    attack_cooldown: f32,
    idle_cooldown: f32,
    state: EnemyState,
}

impl Enemy {
    //Create a new enemy
    pub fn new(x: f32, y: f32, w: f32, h: f32, enemy: EnemyType, flipped: bool) -> Self {
        let mut spr = Sprite::new(x, y, w, h);

        match enemy {
            EnemyType::Slime => spr.set_animation(0.5, 0, 1),
            EnemyType::Eyeball => spr.set_animation(1.0, 0, 1),
            EnemyType::Chicken => spr.set_animation(1.0, 2, 3),
        }

        match enemy {
            EnemyType::Slime => spr.velocity.x = 0.5,
            EnemyType::Eyeball => spr.velocity.x = 1.0,
            EnemyType::Chicken => spr.velocity.x = 1.5,
        }

        let enemy_hp = match enemy {
            EnemyType::Slime => 1,
            EnemyType::Eyeball => 2,
            EnemyType::Chicken => 3,
        };

        spr.flipped = flipped;
        if spr.flipped {
            spr.velocity.x *= -1.0;
        }

        Self {
            sprite: spr,
            enemy_type: enemy,
            health: enemy_hp,
            falling: false,
            damage_cooldown: 0.0,
            attack_cooldown: 0.0,
            idle_cooldown: 0.0,
            state: EnemyState::Wander,
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
            EnemyType::Chicken => {
                shader_program.uniform_vec2f(
                    "uTexOffset",
                    1.0 / 8.0 * self.sprite.current_frame() as f32 + 2.0 / 8.0,
                    1.0 / 8.0,
                );
            }
            EnemyType::Eyeball => {
                shader_program.uniform_vec2f(
                    "uTexOffset",
                    1.0 / 8.0 * self.sprite.current_frame() as f32 + 6.0 / 8.0,
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
            } else if self.sprite.position.y < sprite.position.y {
                //Set y velocity to 0 so we don't "stick" to the tile if the
                //player decides to hold down the jump key
                self.sprite.velocity.y = 0.0;
                //We hit the bottom of a tile, start falling again
                self.falling = true;
            }
        }
    }

    //Returns the tile bounding box of the sprite, this is
    //the rectangle that represents the tile coordinates the sprite
    //is intersecting with
    //
    //of the form: (top_left_x, top_left_y, bot_right_x, bot_right_y)
    fn tile_bounding_box(&self) -> (i32, i32, i32, i32) {
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

        (top_left_x, top_left_y, bot_right_x, bot_right_y)
    }

    //Accelerates sprite due to gravity but also check if the sprite
    //collided with a tile and if the sprite did collide with a tile,
    //then check if the sprite is now being supported by that tile and
    //then check if the sprite should keep falling
    fn fall(&mut self, level: &Level, dt: f32) {
        self.sprite.position.y += self.sprite.velocity.y * dt * 0.5;
        if self.falling {
            self.sprite.velocity.y -= GRAVITY * dt;
        }
        self.sprite.position.y += self.sprite.velocity.y * dt * 0.5;

        let (top_left_x, top_left_y, bot_right_x, bot_right_y) = self.tile_bounding_box();

        self.falling = true;
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

    pub fn update(&mut self, dt: f32, level: &Level, player_pos: &Vector2<f32>) {
        self.sprite.flipped = self.sprite.velocity.x < 0.0;

        match self.enemy_type {
            EnemyType::Slime => self.update_slime(dt, level, player_pos),
            EnemyType::Eyeball => self.update_eyeball(dt, level, player_pos),
            EnemyType::Chicken => self.update_chicken(dt, level, player_pos),
        }

        self.damage_cooldown -= dt;
        self.attack_cooldown -= dt;
    }

    pub fn get_damage(&self) -> i32 {
        if self.attack_cooldown > 0.0 {
            return 0;
        }

        match self.enemy_type {
            EnemyType::Slime | EnemyType::Eyeball | EnemyType::Chicken => 1,
        }
    }

    pub fn apply_damage(&mut self, amount: i32) {
        if self.damage_cooldown <= 0.0 {
            self.health -= amount;
            self.damage_cooldown = DAMAGE_COOLDOWN;
        }
    }

    pub fn score(&self) -> u32 {
        match self.enemy_type {
            EnemyType::Slime => 10,
            EnemyType::Eyeball => 20,
            EnemyType::Chicken => 30,
        }
    }

    pub fn reset_attack_cooldown(&mut self) {
        if self.attack_cooldown < 0.0 {
            self.attack_cooldown = ENEMY_ATTACK_COOLDOWN;
        }
    }
}
