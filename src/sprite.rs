use crate::level::{Level, Tile};
use cgmath::{vec2, Vector2};

pub const PLAYER_SPEED: f32 = 3.0;
pub const PLAYER_JUMP_SPEED: f32 = 9.0;
pub const GRAVITY: f32 = 16.0;

pub struct Sprite {
    pub position: Vector2<f32>,
    pub dimensions: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub falling: bool,
}

impl Sprite {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: vec2(x, y),
            dimensions: vec2(w, h),
            velocity: vec2(0.0, 0.0),
            falling: false,
        }
    }

    pub fn intersecting(&self, sprite: &Sprite) -> bool {
        if self.position.x - self.dimensions.x / 2.0 < sprite.position.x + sprite.dimensions.x / 2.0
            && self.position.y - self.dimensions.y / 2.0
                < sprite.position.y + sprite.dimensions.y / 2.0
            && self.position.x + self.dimensions.x / 2.0
                > sprite.position.x - sprite.dimensions.x / 2.0
            && self.position.y + self.dimensions.y / 2.0
                > sprite.position.y - sprite.dimensions.y / 2.0
        {
            true
        } else {
            false
        }
    }

    fn uncollide_x(&mut self, sprite: &Sprite) {
        if self.intersecting(&sprite) {
            if self.position.x > sprite.position.x {
                self.position.x =
                    sprite.position.x + sprite.dimensions.x / 2.0 + self.dimensions.x / 2.0;
            } else if self.position.x < sprite.position.x {
                self.position.x =
                    sprite.position.x - sprite.dimensions.x / 2.0 - self.dimensions.x / 2.0;
            }
        }
    }

    fn uncollide_y(&mut self, sprite: &Sprite) {
        if self.intersecting(&sprite) {
            if self.position.y > sprite.position.y {
                self.position.y =
                    sprite.position.y + sprite.dimensions.y / 2.0 + self.dimensions.y / 2.0;
                self.falling = false;
            } else if self.position.y < sprite.position.y {
                self.position.y =
                    sprite.position.y - sprite.dimensions.y / 2.0 - self.dimensions.y / 2.0;
                self.falling = true;
                self.velocity.y = 0.0;
            }
        }
    }

    pub fn update(&mut self, dt: f32, level: &Level) {
        self.position.x += self.velocity.x * dt;
        //Handle collision
        let top_left = vec2(self.position.x, self.position.y)
            - vec2(self.dimensions.x / 2.0 + 1.0, self.dimensions.y / 2.0 + 1.0);
        let bot_right = vec2(self.position.x, self.position.y)
            + vec2(self.dimensions.x / 2.0 + 1.0, self.dimensions.y / 2.0 + 1.0);
        let (top_left_x, top_left_y) = (top_left.x.floor() as i32, top_left.y.floor() as i32);
        let (bot_right_x, bot_right_y) = (bot_right.x.ceil() as i32, bot_right.y.ceil() as i32);

        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if level.get_tile(x as u32, y as u32) != Tile::Air {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    self.uncollide_x(&hitbox);
                }
            }
        }

        self.position.y += self.velocity.y / 2.0 * dt;
        if self.falling {
            self.velocity.y -= GRAVITY * dt;
        }
        self.position.y += self.velocity.y / 2.0 * dt;

        //Handle collision
        let top_left = vec2(self.position.x, self.position.y)
            - vec2(self.dimensions.x / 2.0 + 1.0, self.dimensions.y / 2.0 + 1.0);
        let bot_right = vec2(self.position.x, self.position.y)
            + vec2(self.dimensions.x / 2.0 + 1.0, self.dimensions.y / 2.0 + 1.0);
        let (top_left_x, top_left_y) = (top_left.x.floor() as i32, top_left.y.floor() as i32);
        let (bot_right_x, bot_right_y) = (bot_right.x.ceil() as i32, bot_right.y.ceil() as i32);

        self.falling = true;
        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if level.get_tile(x as u32, y as u32) != Tile::Air {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    self.uncollide_y(&hitbox);
                }
            }
        }
    }
}