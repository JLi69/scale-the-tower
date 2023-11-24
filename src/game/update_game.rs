use super::{hiscore, player::PLAYER_CLIMB_SPEED, GameScreen, Projectile, State};
use crate::{
    level::{transparent, Tile},
    sprite::{
        particle::{Particle, ParticleType},
        Sprite,
    }, audio::{SfxPlayer, sfx_ids},
};
use cgmath::vec2;

const MAX_SAFE_FALL_SPEED: f32 = 14.0;
const BOUNCE_SPEED: f32 = -0.5;
const MAX_UPDATE_DISTANCE: f32 = 16.0;

impl State {
    fn add_particles(
        &mut self,
        x: f32,
        y: f32,
        sz: f32,
        speed: f32,
        particle: ParticleType,
        count: i32,
    ) {
        for _ in 0..count {
            let angle = (2.0 * std::f32::consts::PI) * rand::random::<f32>();
            let dist = rand::random::<f32>() * 0.5;
            self.particles.push(Particle::new(
                x + angle.cos() * dist,
                y + angle.sin() * dist,
                sz,
                speed,
                angle,
                particle,
            ));
        }
    }

    pub fn update_enemies(&mut self, dt: f32, sfx_player: &SfxPlayer) {
        let player_pos = self.player_position();
        for i in 0..self.enemies.len() {
            if (self.enemies[i].sprite.position.y - player_pos.y).abs() > MAX_UPDATE_DISTANCE {
                continue;
            }

            self.enemies[i].sprite.update_animation_frame(dt);
            self.enemies[i].update(dt, &self.level, &player_pos, &mut self.projectiles);

            let enemy_pos = self.enemies[i].sprite.position;

            //Melee attack
            if let Some(hitbox) = self.player.attack_hitbox() {
                if hitbox.intersecting(&self.enemies[i].sprite) {
                    if self.enemies[i].apply_damage(1) {
                        self.add_particles(
                            enemy_pos.x,
                            enemy_pos.y,
                            0.15,
                            3.0,
                            ParticleType::Blood,
                            8,
                        );
                        sfx_player.play(sfx_ids::ENEMY_HIT);
                    }

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
                if self.enemies[i].apply_damage(1) {
                    self.add_particles(enemy_pos.x, enemy_pos.y, 0.15, 3.0, ParticleType::Blood, 8); 
                    sfx_player.play(sfx_ids::ENEMY_HIT);
                }
                self.player.player_spr.velocity.y *= BOUNCE_SPEED;

                if self.enemies[i].health <= 0 {
                    self.player.score += self.enemies[i].score();
                }
            } else if self.player.player_spr.intersecting(&self.enemies[i].sprite) {
                if self.player.apply_damage(self.enemies[i].get_damage()) {
                    self.add_particles(
                        self.player_position().x,
                        self.player_position().y,
                        0.15,
                        3.0,
                        ParticleType::Blood,
                        8,
                    );
                    sfx_player.play(sfx_ids::PLAYER_HIT);
                }
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
                let enemy_pos = self.enemies[i].sprite.position;
                self.add_particles(enemy_pos.x, enemy_pos.y, 0.15, 3.0, ParticleType::Blood, 32);
                self.enemies.remove(i);
                sfx_player.play(sfx_ids::EXPLODE);
            }
        }
    }

    pub fn update_particles(&mut self, dt: f32) {
        let player_pos = self.player_position();
        for particle in &mut self.particles {
            if (particle.sprite.position.y - player_pos.y).abs() > MAX_UPDATE_DISTANCE {
                continue;
            }

            particle.update(&self.level, dt);
        }

        //Delete particles
        let mut stop = false;
        while !stop {
            stop = true;
            let mut index = None;
            for (i, particle) in self.particles.iter().enumerate() {
                if particle.timer <= 0.0 {
                    stop = false;
                    index = Some(i);
                }
            }

            if let Some(i) = index {
                self.particles.remove(i);
            }
        }
    }

    pub fn update_projectiles(&mut self, dt: f32) {
        let player_pos = self.player_position();

        //use this to spawn fire particles
        let mut destroyed_fireballs = vec![];

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
                            if *projectile == Projectile::Fireball {
                                destroyed_fireballs.push(sprite.position);
                            }
                            *projectile = Projectile::Destroyed;
                        }
                    }
                }
            }

            sprite.position += sprite.velocity * dt;
        }

        for fireball in destroyed_fireballs {
            self.add_particles(fireball.x, fireball.y, 0.2, 4.0, ParticleType::Fire, 4);
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

    pub fn update_game_screen(&mut self, dt: f32, sfx_player: &SfxPlayer) {
        //Update the player
        let falling = self.player.falling();
        let velocity_y = self.player.player_spr.velocity.y;
        self.player.update(dt, &self.level);
        //Hit the ground, apply fall damage if player is travelling fast enough
        if falling && !self.player.falling() && velocity_y < -MAX_SAFE_FALL_SPEED {
            self.player
                .apply_damage(-((velocity_y + MAX_SAFE_FALL_SPEED) / 12.0).floor() as i32);

            sfx_player.play(sfx_ids::PLAYER_HIT);
            let player_pos = self.player_position();
            self.add_particles(
                player_pos.x,
                player_pos.y,
                0.15,
                3.0,
                ParticleType::Blood,
                8,
            );
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
        self.level.update_interactive_tiles(&mut self.player, sfx_player);
        self.player.damage_cooldown -= dt;

        let player_pos = self.player_position();
        let mut hit = false;
        for (projectile, sprite) in &mut self.projectiles {
            if self.player.player_spr.intersecting(sprite) {
                hit = self.player.apply_damage(1);
                *projectile = Projectile::Destroyed;
            }
        }

        if hit {
            sfx_player.play(sfx_ids::PLAYER_HIT);
            self.add_particles(
                player_pos.x,
                player_pos.y,
                0.15,
                3.0,
                ParticleType::Blood,
                8,
            );
        }

        //Update enemies
        self.update_enemies(dt, sfx_player);
        //Update projectiles
        self.update_projectiles(dt);
        //Update particles
        self.update_particles(dt);
    }

    pub fn check_gameover(&mut self, highscores: &mut Vec<u32>, sfx_player: &SfxPlayer) {
        if self.player.player_health <= 0
            || (self.player_position().y > self.level.h() as f32 - 1.0 && !self.player.falling())
        {
            if self.player.player_health <= 0 {
                sfx_player.play(sfx_ids::EXPLODE);
                let player_pos = self.player_position();
                self.add_particles(
                    player_pos.x,
                    player_pos.y,
                    0.15,
                    3.0,
                    ParticleType::Blood,
                    32,
                );
                self.game_screen = GameScreen::GameOver;
            } else {
                sfx_player.play(sfx_ids::COIN);
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
