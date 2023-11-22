use super::{Enemy, EnemyState, ENEMY_ATTACK_COOLDOWN};
use crate::{game::Projectile, level::transparent, level::Level, Sprite};
use cgmath::{InnerSpace, Vector2};

impl Enemy {
    pub fn update_demon(
        &mut self,
        dt: f32,
        level: &Level,
        player_pos: &Vector2<f32>,
        projectiles: &mut Vec<(Projectile, Sprite)>,
    ) {
        if ((player_pos.x - self.sprite.position.x).abs() > 0.7
            || (player_pos.y - self.sprite.position.y).abs() > 0.2)
            && self.state != EnemyState::Idle
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
                if (self.sprite.position - player_pos).magnitude() < 5.0
                    && (self.sprite.position.y - player_pos.y).abs() < 1.0
                {
                    self.state = EnemyState::Chase;
                }

                self.sprite.set_animation(1.0, 2, 3);
                if collided {
                    self.state = EnemyState::Idle;
                    self.idle_cooldown = 0.0;
                }

                if self.idle_cooldown < -2.0 {
                    self.idle_cooldown = 2.0;
                    self.state = EnemyState::Idle;
                }
            }
            EnemyState::Chase => {
                self.sprite.set_animation(0.4, 2, 3);

                if self.sprite.position.x < player_pos.x - 0.5 {
                    self.sprite.velocity.x = 1.4;
                } else if self.sprite.position.x > player_pos.x + 0.5 {
                    self.sprite.velocity.x = -1.4;
                }

                if (self.sprite.position - player_pos).magnitude() > 5.0 {
                    self.state = EnemyState::Wander;
                    self.sprite.velocity.x = 1.1 * self.sprite.velocity.x.signum();
                }

                if self.idle_cooldown < -1.0 {
                    self.state = EnemyState::Idle;
                    self.idle_cooldown = 2.0;
                }
            }
            EnemyState::Idle => {
                self.sprite.set_animation(1.0, 0, 1);
                if self.idle_cooldown < -2.0 {
                    self.idle_cooldown = 2.0;
                    if (self.sprite.position - player_pos).magnitude() < 5.0
                        && (self.sprite.position.y - player_pos.y).abs() < 1.0
                    {
                        self.state = EnemyState::Chase;
                    } else {
                        self.state = EnemyState::Wander;
                        self.sprite.velocity.x *= -1.0;
                    }
                }

                //Shoot a fireball
                if self.attack_cooldown < 0.0 {
                    self.sprite.animation_timer = 0.75;
                    let mut sprite = Sprite::new(
                        self.sprite.position.x + self.sprite.velocity.x.signum() * 0.7,
                        self.sprite.position.y - 0.1,
                        0.3,
                        0.3,
                    );
                    sprite.velocity.x = 4.0 * self.sprite.velocity.x.signum();

                    projectiles.push((Projectile::Fireball, sprite));
                    self.attack_cooldown = ENEMY_ATTACK_COOLDOWN;
                }
            }
        }

        self.idle_cooldown -= dt;
        self.fall(level, dt);
    }
}
