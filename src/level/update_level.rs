use super::{InteractiveTile, Level};
use crate::game::Player;
use crate::sprite::Sprite;

impl Level {
    pub fn update_interactive_tiles(&mut self, player: &mut Player) {
        let mut delete_index = None;

        for (i, tile) in self.interactive_tiles.iter().enumerate() {
            let hitbox = Sprite::new(tile.tile_x, tile.tile_y, 1.0, 1.0);

            match tile.tile_type {
                InteractiveTile::Gold => {
                    if player.player_spr.intersecting(&hitbox) {
                        player.score += 50;
                        delete_index = Some(i);
                        break;
                    }
                }
                InteractiveTile::SmallGold => {
                    if player.player_spr.intersecting(&hitbox) {
                        player.score += 10;
                        delete_index = Some(i);
                        break;
                    }
                }
            }
        }

        if let Some(i) = delete_index {
            self.interactive_tiles.remove(i);
        }
    }

    pub fn update_enemies(&mut self, dt: f32) {
        for enemy in &mut self.enemies {
            enemy.sprite.update_animation_frame(dt);
        }
    }
}
