use crate::level::{Level, Tile};
use cgmath::{vec2, Vector2};

pub mod enemy;

pub struct Sprite {
    pub position: Vector2<f32>,
    pub dimensions: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub flipped: bool,

    //In seconds
    animation_timer: f32,
    animation_duration: f32,
    //Location of the starting and ending frame of the animation
    //Assume that the animation frames are all on the same row of the texture
    start_frame: u8,
    end_frame: u8,
}

impl Sprite {
    //Creates a new sprite
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: vec2(x, y),
            dimensions: vec2(w, h),
            velocity: vec2(0.0, 0.0),
            flipped: false,

            animation_timer: 0.0,
            animation_duration: 0.0,
            start_frame: 0,
            end_frame: 0,
        }
    }

    //Collision detection
    pub fn intersecting(&self, sprite: &Sprite) -> bool {
        self.position.x - self.dimensions.x / 2.0 < sprite.position.x + sprite.dimensions.x / 2.0
            && self.position.y - self.dimensions.y / 2.0
                < sprite.position.y + sprite.dimensions.y / 2.0
            && self.position.x + self.dimensions.x / 2.0
                > sprite.position.x - sprite.dimensions.x / 2.0
            && self.position.y + self.dimensions.y / 2.0
                > sprite.position.y - sprite.dimensions.y / 2.0
    }

    //Updates the animation timer
    pub fn update_animation_frame(&mut self, dt: f32) {
        if self.animation_duration <= 0.0 {
            return;
        }

        self.animation_timer += dt;
        self.animation_timer -=
            (self.animation_timer / self.animation_duration).floor() * self.animation_duration;
    }

    //Returns the current frame of the animation (returns a u8, we can therefore
    //have up to 256 different frames of animation)
    pub fn current_frame(&self) -> u8 {
        self.start_frame
            + ((self.end_frame - self.start_frame + 1) as f32 * self.animation_timer
                / self.animation_duration) as u8
    }

    //Checks if the sprite is in contact with a certain type of tile
    pub fn touching_tile(&self, tile: Tile, level: &Level) -> bool {
        let top_left = vec2(self.position.x, self.position.y)
            - vec2(
                self.dimensions.x.ceil() / 2.0 + 1.0,
                self.dimensions.y.ceil() / 2.0 + 1.0,
            );
        let bot_right = vec2(self.position.x, self.position.y)
            + vec2(
                self.dimensions.x.ceil() / 2.0 + 1.0,
                self.dimensions.y.ceil() / 2.0 + 1.0,
            );
        let (top_left_x, top_left_y) = (top_left.x.floor() as i32, top_left.y.floor() as i32);
        let (bot_right_x, bot_right_y) = (bot_right.x.ceil() as i32, bot_right.y.ceil() as i32);

        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                let hitbox = if tile == Tile::Spikes {
                    Sprite::new(x as f32, y as f32 - 0.2, 0.8 / 2.0f32.sqrt(), 0.6)
                } else {
                    Sprite::new(x as f32, y as f32, 1.0, 1.0)
                };
                if level.get_tile(x as u32, y as u32) == tile && self.intersecting(&hitbox) {
                    return true;
                }
            }
        }

        false
    }

    pub fn set_animation(&mut self, duration: f32, start_frame: u8, end_frame: u8) {
        self.animation_duration = duration;
        self.start_frame = start_frame;
        self.end_frame = end_frame;
    }

    //Uncollide the sprite with another sprite in the x axis
    pub fn uncollide_x(&mut self, sprite: &Sprite) {
        if self.intersecting(sprite) {
            if self.position.x > sprite.position.x {
                self.position.x =
                    sprite.position.x + sprite.dimensions.x / 2.0 + self.dimensions.x / 2.0 + 0.01;
            } else if self.position.x < sprite.position.x {
                self.position.x =
                    sprite.position.x - sprite.dimensions.x / 2.0 - self.dimensions.x / 2.0 - 0.01;
            }
        }
    }

    //Uncollide the sprite with another sprite in the y axis
    pub fn uncollide_y(&mut self, sprite: &Sprite) {
        if self.intersecting(sprite) {
            if self.position.y > sprite.position.y {
                self.position.y =
                    sprite.position.y + sprite.dimensions.y / 2.0 + self.dimensions.y / 2.0;
            } else if self.position.y < sprite.position.y {
                self.position.y =
                    sprite.position.y - sprite.dimensions.y / 2.0 - self.dimensions.y / 2.0;
            }
        }
    }
}
