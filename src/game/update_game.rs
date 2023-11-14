use super::{GameScreen, State};
use crate::{level::Tile, sprite::PLAYER_CLIMB_SPEED};

const MAX_SAFE_FALL_SPEED: f32 = 14.0;

impl State {
    pub fn update_game_screen(&mut self, dt: f32) {
        //Update the player
        let falling = self.player.player_spr.falling();
        let velocity_y = self.player.player_spr.velocity.y;
        self.player.player_spr.update(dt, &self.level);
        //Hit the ground, apply fall damage if player is travelling fast enough
        if falling && !self.player.player_spr.falling() && velocity_y < -MAX_SAFE_FALL_SPEED {
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
        self.player.player_spr.update_animation_state();
        self.level.update_interactive_tiles(&mut self.player);
        self.player.damage_cooldown -= dt;

        if self.player.player_health <= 0 {
            self.game_screen = GameScreen::GameOver;
        }
    }
}
