use crate::level::{transparent, Level, Tile};
use cgmath::{vec2, Vector2};

//Speed the player walks at
pub const PLAYER_SPEED: f32 = 3.0;
//Jump speed of the player
pub const PLAYER_JUMP_SPEED: f32 = 9.0;
//Speed the player climbs at
pub const PLAYER_CLIMB_SPEED: f32 = 4.0;
//Force of gravity on all sprites
pub const GRAVITY: f32 = 16.0;

//Sprite animation state
//Determines what to display when drawing the sprite onto the screen
enum AnimationState {
    Idle,
    Walking,
    Jumping,
}

pub struct Sprite {
    pub position: Vector2<f32>,
    pub dimensions: Vector2<f32>,
    pub velocity: Vector2<f32>,
    //Whether the sprite is falling (not supported by any tiles)
    falling: bool,
    climbing: bool,
    pub flipped: bool,

    //In seconds
    animation_timer: f32,
    animation_duration: f32,
    //Location of the starting and ending frame of the animation
    //Assume that the animation frames are all on the same row of the texture
    start_frame: u8,
    end_frame: u8,

    animation_state: AnimationState,
}

impl Sprite {
    //Creates a new sprite
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: vec2(x, y),
            dimensions: vec2(w, h),
            velocity: vec2(0.0, 0.0),
            falling: false,
            climbing: false,
            flipped: false,

            animation_timer: 0.0,
            animation_duration: 0.0,
            start_frame: 0,
            end_frame: 0,
            animation_state: AnimationState::Idle,
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

    //Uncollide the sprite with another sprite in the x axis
    fn uncollide_x(&mut self, sprite: &Sprite) {
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
    fn uncollide_y(&mut self, sprite: &Sprite) {
        if self.intersecting(sprite) {
            if self.position.y > sprite.position.y {
                self.position.y =
                    sprite.position.y + sprite.dimensions.y / 2.0 + self.dimensions.y / 2.0;
                //If we are supported by a tile then stop falling
                self.falling = false;
                self.velocity.y = -0.01;
                self.climbing = false;
            } else if self.position.y < sprite.position.y {
                self.position.y =
                    sprite.position.y - sprite.dimensions.y / 2.0 - self.dimensions.y / 2.0;
                //Set y velocity to 0 so we don't "stick" to the tile if the
                //player decides to hold down the jump key
                self.velocity.y = 0.0;
                //We hit the bottom of a tile, start falling again
                self.falling = true;
            }
        }
    }

    //NOTE: collision detection isn't perfect, if the sprite is moving
    //too fast or the framerate drops too low, then the sprite may end
    //up clipping through tiles
    pub fn update(&mut self, dt: f32, level: &Level) {
        //Determine if to display whether the sprite is flipped based on
        //the x velocity of the sprite and what direction the sprite is heading
        if self.velocity.x < 0.0 {
            self.flipped = true;
        } else if self.velocity.x > 0.0 {
            self.flipped = false;
        }

        //Update x
        self.position.x += self.velocity.x * dt;
        //Handle collision
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

        //Scan the level for tiles the sprite might have collided with
        //and then uncollide the sprite from the tiles
        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if !transparent(level.get_tile(x as u32, y as u32)) {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    self.uncollide_x(&hitbox);
                }
            }
        }

        //Cap speed of player when they are on a ladder
        if self.climbing {
            self.velocity.y =
                self.velocity.y.abs().min(PLAYER_CLIMB_SPEED) * self.velocity.y.signum();
        }

        //Update y
        self.position.y += self.velocity.y / 2.0 * dt;
        //Accelerate due to gravity
        if self.falling && !self.climbing {
            self.velocity.y -= GRAVITY * dt;
        }
        self.position.y += self.velocity.y / 2.0 * dt;

        //Handle collision
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

        self.falling = true;
        self.climbing = false;

        //Uncollide from any tiles and also determine if the sprite is falling
        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if !transparent(level.get_tile(x as u32, y as u32)) {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    self.uncollide_y(&hitbox);
                } else if level.get_tile(x as u32, y as u32) == Tile::Ladder {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    if self.intersecting(&hitbox) {
                        self.falling = false;
                        self.climbing = true;
                    }
                }
            }
        }
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

    //Updates the animation state of the player based on various conditions
    pub fn update_animation_state(&mut self) {
        if self.falling {
            self.animation_state = AnimationState::Jumping
        } else if self.velocity.x != 0.0 {
            self.animation_state = AnimationState::Walking
        } else {
            self.animation_state = AnimationState::Idle;
        }

        match self.animation_state {
            AnimationState::Idle => {
                self.animation_duration = 1.0;
                self.start_frame = 0;
                self.end_frame = 1;
            }
            AnimationState::Walking => {
                self.animation_duration = 1.0;
                self.start_frame = 2;
                self.end_frame = 5;
            }
            AnimationState::Jumping => {
                self.animation_duration = 1.0;
                self.start_frame = 6;
                self.end_frame = 7;
            }
        }
    }

    //Returns if the sprite is falling
    pub fn falling(&self) -> bool {
        self.falling
    }

    //Returns if the sprite is climbing
    pub fn climbing(&self) -> bool {
        self.climbing
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
}
