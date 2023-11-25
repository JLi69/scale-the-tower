use super::{InteractiveTile, Level};
use crate::audio::{sfx_ids, SfxPlayer};
use crate::game::Player;
use crate::sprite::Sprite;

impl Level {
    pub fn update_interactive_tiles(&mut self, player: &mut Player, sfx_player: &SfxPlayer) {
        let mut delete_index = None;

        for (i, tile) in self.interactive_tiles.iter().enumerate() {
            let hitbox = Sprite::new(tile.tile_x, tile.tile_y, 1.0, 1.0);

            match tile.tile_type {
                InteractiveTile::Gold => {
                    if player.player_spr.intersecting(&hitbox) {
                        player.score += 50;
                        delete_index = Some(i);
                        sfx_player.play(sfx_ids::COIN);
                        break;
                    }
                }
                InteractiveTile::SmallGold => {
                    if player.player_spr.intersecting(&hitbox) {
                        player.score += 10;
                        delete_index = Some(i);
                        sfx_player.play(sfx_ids::COIN);
                        break;
                    }
                }
                InteractiveTile::Heal => {
                    if player.player_spr.intersecting(&hitbox)
                        && player.player_health < player.max_player_health
                    {
                        player.player_health += 1;
                        delete_index = Some(i);
                        sfx_player.play(sfx_ids::POWERUP);
                        break;
                    }
                }
                InteractiveTile::HealthBoost => {
                    if player.player_spr.intersecting(&hitbox) {
                        player.max_player_health += 1;
                        player.player_health += 1;
                        delete_index = Some(i);
                        sfx_player.play(sfx_ids::POWERUP);
                        break;
                    }
                }
                InteractiveTile::Arrows => {
                    if player.player_spr.intersecting(&hitbox) {
                        player.arrows += 2;
                        delete_index = Some(i);
                        sfx_player.play(sfx_ids::POWERUP);
                        break;
                    }
                }
            }
        }

        if let Some(i) = delete_index {
            self.interactive_tiles.remove(i);
        }
    }
}
