use super::{Enemy, EnemyState};
use crate::{
    game::player::PLAYER_CLIMB_SPEED,
    level::transparent,
    level::{Level, Tile},
    Sprite,
};
use cgmath::{InnerSpace, Vector2};

impl Enemy {
    pub fn update_chicken(&mut self, dt: f32, level: &Level, player_pos: &Vector2<f32>) {
        if ((player_pos.x - self.sprite.position.x).abs() > 0.7
            || (player_pos.y - self.sprite.position.y).abs() > 0.2)
            && (self.state == EnemyState::Wander || self.state == EnemyState::Chase)
        {
            self.sprite.position.x += self.sprite.velocity.x * dt;
        }

        //Handle collision
        let (top_left_x, top_left_y, bot_right_x, bot_right_y) = self.tile_bounding_box();

        //Scan the level for tiles the sprite might have collided with
        //and then uncollide the sprite from the tiles
        let mut collided = false;
        let mut at_edge = false;
        for x in top_left_x..bot_right_x {
            for y in top_left_y..bot_right_y {
                if level.out_of_bounds(x, y) {
                    continue;
                }

                if !transparent(level.get_tile(x as u32, y as u32)) {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    if self.sprite.intersecting(&hitbox) {
                        collided = true;
                    }
                    self.sprite.uncollide_x(&hitbox);
                } else if !level.out_of_bounds(x, y - 1)
                    && transparent(level.get_tile(x as u32, y as u32 - 1))
                    && self.state == EnemyState::Wander
                {
                    let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                    if self.sprite.intersecting(&hitbox) {
                        at_edge = true;
                    }
                }
            }
        }

        if self.sprite.touching_tile(Tile::Lava, level)
            || (self.sprite.touching_tile(Tile::Spikes, level)
                && self.sprite.velocity.y <= -PLAYER_CLIMB_SPEED)
        {
            self.health = 0;
        }

        if (self.sprite.position - player_pos).magnitude() < 5.0 {
            self.state = EnemyState::Chase;
            self.sprite.velocity.x = 2.0 * self.sprite.velocity.x.signum();
        }

        match self.state {
            EnemyState::Wander => {
                self.sprite.set_animation(1.0, 2, 3);
                if collided || at_edge {
                    self.state = EnemyState::Idle;
                    self.idle_cooldown = -4.5;
                }

                if self.idle_cooldown < -5.0 {
                    self.state = EnemyState::Idle;
                    self.idle_cooldown = 5.0;
                }
            }
            EnemyState::Chase => {
                self.idle_cooldown = 0.0;
                self.sprite.set_animation(0.4, 2, 3);

                if self.sprite.position.x < player_pos.x - 0.5 {
                    self.sprite.velocity.x = 2.0;
                } else if self.sprite.position.x > player_pos.x + 0.5 {
                    self.sprite.velocity.x = -2.0;
                }

                if (self.sprite.position - player_pos).magnitude() > 5.0
                    && self.sprite.velocity.y <= 0.0
                {
                    self.state = EnemyState::Wander;
                    self.sprite.velocity.x = 1.5 * self.sprite.velocity.x.signum();
                }

                if collided && !self.falling && self.state == EnemyState::Chase {
                    //Attempt to jump over the obstacle
                    self.sprite.velocity.y = 6.0;
                }
            }
            EnemyState::Idle => {
                self.sprite.set_animation(1.0, 0, 1);
                if self.idle_cooldown < -5.0 {
                    self.idle_cooldown = 5.0;
                    self.state = EnemyState::Wander;
                    self.sprite.velocity.x *= -1.0;
                }
            }
        }

        self.idle_cooldown -= dt;
        self.fall(level, dt);
    }
}
