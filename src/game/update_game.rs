use super::{hiscore, player::PLAYER_CLIMB_SPEED, GameScreen, State};
use crate::level::Tile;

const MAX_SAFE_FALL_SPEED: f32 = 14.0;
const BOUNCE_SPEED: f32 = -0.5;

impl State {
    pub fn update_enemies(&mut self, dt: f32) {
        for i in 0..self.enemies.len() {
            self.enemies[i].sprite.update_animation_frame(dt);
            self.enemies[i].update(dt, &self.level);

            //Melee attack
            if let Some(hitbox) = self.player.attack_hitbox() {
                if hitbox.intersecting(&self.enemies[i].sprite) {
                    self.enemies[i].apply_damage(1);
                }
            }

            //Goomba stomp the enemy
            if self.player.player_spr.intersecting(&self.enemies[i].sprite) &&
               self.player_position().y > self.enemies[i].sprite.position.y &&
               self.player_velocity().y < -PLAYER_CLIMB_SPEED {
                self.enemies[i].apply_damage(2);
                self.player.player_spr.velocity.y *= BOUNCE_SPEED;
            } 
        }

        let mut stop = false;
        while !stop {
            stop = true;
            let mut index = None;
            for (i, enemy) in self.enemies.iter().enumerate() {
                if enemy.health <= 0 {
                    self.player.score += enemy.score();
                    stop = false;
                    index = Some(i); 
                }
            }

            if let Some(i) = index {
                self.enemies.remove(i); 
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

        //Update enemies
        self.update_enemies(dt);
    }

    pub fn check_gameover(&mut self, highscores: &mut Vec<u32>) {
        if self.player.player_health <= 0 {
            self.game_screen = GameScreen::GameOver;

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
