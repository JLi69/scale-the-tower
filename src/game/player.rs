use super::{Player, GRAVITY, PLAYER_HEIGHT};
use crate::level::{transparent, Level, Tile, ROOM_SIZE};
use crate::sprite::Sprite;
use cgmath::vec2;

//Speed the player walks at
pub const PLAYER_SPEED: f32 = 3.0;
//Jump speed of the player
pub const PLAYER_JUMP_SPEED: f32 = 9.0;
//Speed the player climbs at
pub const PLAYER_CLIMB_SPEED: f32 = 4.0;

impl Player {
    //Handle collision in the y axis
    fn handle_collision_y(&mut self, sprite: &Sprite) {
        if self.player_spr.intersecting(sprite) {
            if self.player_spr.position.y > sprite.position.y {
                //If we are supported by a tile then stop falling
                self.falling = false;
                self.player_spr.velocity.y = -0.01;
                self.climbing = false;
            } else if self.player_spr.position.y < sprite.position.y {
                //Set y velocity to 0 so we don't "stick" to the tile if the
                //player decides to hold down the jump key
                self.player_spr.velocity.y = 0.0;
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
        if self.player_spr.velocity.x < 0.0 {
            self.player_spr.flipped = true;
        } else if self.player_spr.velocity.x > 0.0 {
            self.player_spr.flipped = false;
        }

        //Update x
        self.player_spr.position.x += self.player_spr.velocity.x * dt;
        //Handle collision
        let top_left = vec2(self.player_spr.position.x, self.player_spr.position.y)
            - vec2(
                self.player_spr.dimensions.x.ceil() / 2.0 + 1.0,
                self.player_spr.dimensions.y.ceil() / 2.0 + 1.0,
            );
        let bot_right = vec2(self.player_spr.position.x, self.player_spr.position.y)
            + vec2(
                self.player_spr.dimensions.x.ceil() / 2.0 + 1.0,
                self.player_spr.dimensions.y.ceil() / 2.0 + 1.0,
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
                    self.player_spr.uncollide_x(&hitbox);
                }
            }
        }

        //Cap speed of player when they are on a ladder
        if self.climbing {
            self.player_spr.velocity.y = self.player_spr.velocity.y.abs().min(PLAYER_CLIMB_SPEED)
                * self.player_spr.velocity.y.signum();
        }

        //Update y
        self.player_spr.position.y += self.player_spr.velocity.y / 2.0 * dt;
        //Accelerate due to gravity
        if self.falling && !self.climbing {
            self.player_spr.velocity.y -= GRAVITY * dt;
        }
        self.player_spr.position.y += self.player_spr.velocity.y / 2.0 * dt;

        //Handle collision
        let top_left = vec2(self.player_spr.position.x, self.player_spr.position.y)
            - vec2(
                self.player_spr.dimensions.x.ceil() / 2.0 + 1.0,
                self.player_spr.dimensions.y.ceil() / 2.0 + 1.0,
            );
        let bot_right = vec2(self.player_spr.position.x, self.player_spr.position.y)
            + vec2(
                self.player_spr.dimensions.x.ceil() / 2.0 + 1.0,
                self.player_spr.dimensions.y.ceil() / 2.0 + 1.0,
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
                    self.handle_collision_y(&hitbox);
                    self.player_spr.uncollide_y(&hitbox);
                } else if level.get_tile(x as u32, y as u32) == Tile::Ladder {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    if self.player_spr.intersecting(&hitbox) {
                        self.falling = false;
                        self.climbing = true;
                    }
                }
            }
        }

        //Clamp the player's position
        self.player_spr.position.x = self
            .player_spr
            .position
            .x
            .clamp(0.0, ROOM_SIZE as f32 + 1.0);

        //Clamp the player's y position to prevent them from falling through the
        //floor of the world
        self.player_spr.position.y = self
            .player_spr
            .position
            .y
            .max(1.0 - (1.0 - PLAYER_HEIGHT) / 2.0);

        self.attack_timer -= dt;
        self.attack_cooldown -= dt;
    }

    //Updates the animation state of the player based on various conditions
    pub fn update_animation_state(&mut self) {
        if self.falling {
            self.player_spr.set_animation(1.0, 6, 7);
        } else if self.player_spr.velocity.x != 0.0 {
            self.player_spr.set_animation(1.0, 2, 5);
        } else {
            self.player_spr.set_animation(1.0, 0, 1);
        }
    }
}
