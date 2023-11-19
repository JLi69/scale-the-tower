use super::{Enemy, EnemyState};
use crate::{level::transparent, level::Level, Sprite};
use cgmath::{InnerSpace, Vector2};

impl Enemy {
    pub fn update_eyeball(&mut self, dt: f32, level: &Level, player_pos: &Vector2<f32>) {
        if (player_pos.x - self.sprite.position.x).abs() > 0.7
            || (player_pos.y - self.sprite.position.y).abs() > 0.2
        {
            self.sprite.position.x += self.sprite.velocity.x * dt;
        }

        //Handle collision
        let (top_left_x, top_left_y, bot_right_x, bot_right_y) = self.tile_bounding_box();

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

        match self.state {
            EnemyState::Wander => {
                if collided {
                    self.sprite.velocity.x *= -1.0;
                    self.sprite.position.x += self.sprite.velocity.x * dt;
                }

                if (self.sprite.position - player_pos).magnitude() < 5.0
                    && (self.sprite.position.y - player_pos.y).abs() < 1.0
                {
                    self.state = EnemyState::Chase;
                }
            }
            EnemyState::Chase => {
                if self.sprite.position.x < player_pos.x - 0.5 {
                    self.sprite.velocity.x = self.sprite.velocity.x.abs();
                } else if self.sprite.position.x > player_pos.x + 0.5 {
                    self.sprite.velocity.x = -self.sprite.velocity.x.abs();
                }

                if (self.sprite.position - player_pos).magnitude() > 5.0 {
                    self.state = EnemyState::Wander;
                }
            }
            _ => {}
        }

        self.fall(level, dt);
    }
}
