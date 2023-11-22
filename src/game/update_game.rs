use super::{hiscore, player::PLAYER_CLIMB_SPEED, GameScreen, Projectile, State};
use crate::{
    level::{transparent, Tile},
    sprite::Sprite,
};
use cgmath::vec2;

const MAX_SAFE_FALL_SPEED: f32 = 14.0;
const BOUNCE_SPEED: f32 = -0.5;
const MAX_UPDATE_DISTANCE: f32 = 16.0;

impl State {
    pub fn update_enemies(&mut self, dt: f32) {
        let player_pos = self.player_position();
        for i in 0..self.enemies.len() {
            if (self.enemies[i].sprite.position.y - player_pos.y).abs() > MAX_UPDATE_DISTANCE {
                continue;
            }

            self.enemies[i].sprite.update_animation_frame(dt);
            self.enemies[i].update(dt, &self.level, &player_pos, &mut self.projectiles);

            //Melee attack
            if let Some(hitbox) = self.player.attack_hitbox() {
                if hitbox.intersecting(&self.enemies[i].sprite) {
                    self.enemies[i].apply_damage(1);

                    if self.enemies[i].health <= 0 {
                        self.player.score += self.enemies[i].score();
                    }
                }
            }

            //Goomba stomp the enemy
            if self.player.player_spr.intersecting(&self.enemies[i].sprite)
                && self.player_position().y
                    > self.enemies[i].sprite.position.y + self.enemies[i].sprite.dimensions.y / 2.0
                && self.player_velocity().y < -PLAYER_CLIMB_SPEED
                && !self.player.climbing()
                && self.player.player_health > 0
            {
                self.enemies[i].apply_damage(1);
                self.player.player_spr.velocity.y *= BOUNCE_SPEED;

                if self.enemies[i].health <= 0 {
                    self.player.score += self.enemies[i].score();
                }
            } else if self.player.player_spr.intersecting(&self.enemies[i].sprite) {
                self.player.apply_damage(self.enemies[i].get_damage());
                self.enemies[i].reset_attack_cooldown();
            }

            for (projectile, sprite) in &mut self.projectiles {
                if self.enemies[i].sprite.intersecting(sprite) {
                    self.enemies[i].health = 0;
                    *projectile = Projectile::Destroyed;
                }
            }
        }

        let mut stop = false;
        while !stop {
            stop = true;
            let mut index = None;
            for (i, enemy) in self.enemies.iter().enumerate() {
                if enemy.health <= 0 {
                    stop = false;
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                self.enemies.remove(i);
            }
        }
    }

    pub fn update_projectiles(&mut self, dt: f32) {
        let player_pos = self.player_position();
        for (projectile, sprite) in &mut self.projectiles {
            if (sprite.position.y - player_pos.y).abs() > MAX_UPDATE_DISTANCE {
                continue;
            }

            if sprite.position.x.abs() >= 64.0 {
                *projectile = Projectile::Destroyed;
            }

            let top_left = vec2(sprite.position.x, sprite.position.y)
                - vec2(
                    sprite.dimensions.x.ceil() / 2.0 + 1.0,
                    sprite.dimensions.y.ceil() / 2.0 + 1.0,
                );
            let bot_right = vec2(sprite.position.x, sprite.position.y)
                + vec2(
                    sprite.dimensions.x.ceil() / 2.0 + 1.0,
                    sprite.dimensions.y.ceil() / 2.0 + 1.0,
                );
            let (top_left_x, top_left_y) = (top_left.x.floor() as i32, top_left.y.floor() as i32);
            let (bot_right_x, bot_right_y) = (bot_right.x.ceil() as i32, bot_right.y.ceil() as i32);
            for x in top_left_x..bot_right_x {
                for y in top_left_y..bot_right_y {
                    if !self.level.out_of_bounds(x, y)
                        && !transparent(self.level.get_tile(x as u32, y as u32))
                    {
                        let hitbox = Sprite::new(x as f32, y as f32, 1.0, 1.0);
                        if sprite.intersecting(&hitbox) {
                            *projectile = Projectile::Destroyed;
                        }
                    }
                }
            }

            sprite.position += sprite.velocity * dt;
        }

        let mut stop = false;
        while !stop {
            stop = true;
            let mut index = None;
            for (i, (projectile, _)) in self.projectiles.iter().enumerate() {
                if *projectile == Projectile::Destroyed {
                    stop = false;
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                self.projectiles.remove(i);
            }
        }
    }

    pub fn update_game_screen(&mut self, dt: f32) {
        //Update the player
        let falling = self.player.falling();
        let velocity_y = self.player.player_spr.velocity.y;
        self.player.update(dt, &self.level);
        //Hit the ground, apply fall damage if player is travelling fast enough
        if falling && !self.player.falling() && velocity_y < -MAX_SAFE_FALL_SPEED {
            self.player
                .apply_damage(-((velocity_y + MAX_SAFE_FALL_SPEED) / 12.0).floor() as i32);
        }
        //Kill player instantly upon contact with lava
        if self
            .player
            .player_spr
            .touching_tile(Tile::Lava, &self.level)
        {
            self.player.player_health = 0;
        }
        //Kill player instantly when jumping onto spikes
        if self
            .player
            .player_spr
            .touching_tile(Tile::Spikes, &self.level)
            && self.player.player_spr.velocity.y < -PLAYER_CLIMB_SPEED
        {
            self.player.player_health = 0;
        }
        self.player.player_spr.update_animation_frame(dt);
        self.player.update_animation_state();
        self.level.update_interactive_tiles(&mut self.player);
        self.player.damage_cooldown -= dt;

        for (projectile, sprite) in &mut self.projectiles {
            if self.player.player_spr.intersecting(sprite) {
                self.player.apply_damage(1);
                *projectile = Projectile::Destroyed;
            }
        }

        //Update enemies
        self.update_enemies(dt);
        //Update projectiles
        self.update_projectiles(dt);
    }

    pub fn check_gameover(&mut self, highscores: &mut Vec<u32>) {
        if self.player.player_health <= 0
            || (self.player_position().y > self.level.h() as f32 - 1.0 && !self.player.falling())
        {
            if self.player.player_health <= 0 {
                self.game_screen = GameScreen::GameOver;
            } else {
                self.player.player_spr.set_animation(1.0, 0, 0);
                self.player.player_spr.update_animation_frame(0.0);
                self.player.score += 500;
                self.game_screen = GameScreen::WinScreen;
            }

            //Check if the player got a new high score and if they
            //did, write that highscore to a file
            if hiscore::is_new_highscore(self.player.score, highscores) {
                hiscore::add_highscore(self.player.score, highscores);
                hiscore::write_highscores("hiscores", highscores);
                self.new_highscore = true;
            } else {
                self.new_highscore = false;
            }
        }
    }
}
