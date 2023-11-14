use crate::{Level, Sprite};
use cgmath::{Deg, Matrix4};

pub mod display;
pub mod update_game;

//Constants
pub const DEFAULT_PLAYER_HEALTH: i32 = 4;
pub const DAMAGE_COOLDOWN: f32 = 0.3;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum GameScreen {
    MainMenu,
    Game,
    Paused,
    GameOver,
}

pub struct Player {
    pub player_spr: Sprite,
    pub score: u32,
    pub player_health: i32,
    pub max_player_health: i32,
    pub damage_cooldown: f32,
}

impl Player {
    pub fn start_state() -> Self {
        Self {
            player_spr: Sprite::new(1.0, 1.0, 0.8, 1.0),
            score: 0,
            player_health: DEFAULT_PLAYER_HEALTH,
            max_player_health: DEFAULT_PLAYER_HEALTH,
            damage_cooldown: 0.0,
        }
    }

    pub fn apply_damage(&mut self, amount: i32) {
        if self.damage_cooldown <= 0.0 && amount > 0 {
            self.player_health -= amount;
            self.damage_cooldown = DAMAGE_COOLDOWN;
        }
    }
}

//Structure to store the current state of the application and allow us
//to pass it to different functions so that it can be modified
pub struct State {
    pub perspective: Matrix4<f32>,
    pub player: Player,
    pub game_screen: GameScreen,
    pub level: Level,
    pub left_mouse_held: bool,
}

impl State {
    pub fn starting_state() -> Self {
        Self {
            perspective: cgmath::perspective(Deg(75.0), 800.0 / 600.0, 0.1, 1000.0),
            player: Player::start_state(),
            game_screen: GameScreen::MainMenu,
            level: Level::new(1, 1),
            left_mouse_held: false,
        }
    }
}
