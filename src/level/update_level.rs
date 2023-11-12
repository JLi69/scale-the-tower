use super::{InteractiveTile, Level};
use crate::sprite::Sprite;
use crate::State;

impl Level {
    pub fn update_interactive_tiles(&mut self, state: &mut State) {
        let mut delete_index = None;

        for (i, tile) in self.interactive_tiles.iter().enumerate() {
            let hitbox = Sprite::new(tile.tile_x, tile.tile_y, 1.0, 1.0);

            match tile.tile_type {
                InteractiveTile::Gold => {
                    if state.player.intersecting(&hitbox) {
                        state.score += 50;
                        delete_index = Some(i);
                        break;
                    }
                }
                InteractiveTile::SmallGold => {
                    if state.player.intersecting(&hitbox) {
                        state.score += 10;
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
}
