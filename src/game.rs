use crate::{sprite::enemy::Enemy, Level, Sprite};
use cgmath::{Deg, Matrix4, Vector2};

pub mod display;
pub mod hiscore;
pub mod player;
pub mod update_game;

//Constants
//Force of gravity on all sprites
pub const GRAVITY: f32 = 16.0;
pub const DEFAULT_PLAYER_HEALTH: i32 = 4;
pub const DAMAGE_COOLDOWN: f32 = 0.3;
pub const ATTACK_COOLDOWN: f32 = 0.5;
pub const ATTACK_TIMER: f32 = 0.2;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum GameScreen {
    MainMenu,
    Game,
    Paused,
    GameOver,
    HighScores,
}

pub struct Player {
    pub player_spr: Sprite,
    pub score: u32,
    pub player_health: i32,
    pub max_player_health: i32,
    pub damage_cooldown: f32,
    falling: bool,
    climbing: bool,
    attack_cooldown: f32,
    attack_timer: f32,
}

impl Player {
    pub fn start_state() -> Self {
        Self {
            player_spr: Sprite::new(1.0, 1.0, 0.8, 1.0),
            score: 0,
            player_health: DEFAULT_PLAYER_HEALTH,
            max_player_health: DEFAULT_PLAYER_HEALTH,
            damage_cooldown: 0.0,
            falling: false,
            climbing: false,
            attack_cooldown: 0.0,
            attack_timer: 0.0,
        }
    }

    pub fn apply_damage(&mut self, amount: i32) {
        if self.damage_cooldown <= 0.0 && amount > 0 {
            self.player_health -= amount;
            self.damage_cooldown = DAMAGE_COOLDOWN;
        }
    }

    //Returns if the player is falling
    pub fn falling(&self) -> bool {
        self.falling
    }

    //Returns if the player is climbing
    pub fn climbing(&self) -> bool {
        self.climbing
    }

    pub fn attack(&mut self) {
        if self.attack_cooldown <= 0.0 {
            self.attack_cooldown = ATTACK_COOLDOWN;
            self.attack_timer = ATTACK_TIMER;
        }
    }

    //Returns None if the attack cooldown isn't at 0 yet
    pub fn attack_hitbox(&self) -> Option<Sprite> {
        if self.attack_timer < 0.0 {
            return None;
        }

        if self.player_spr.flipped {
            Some(Sprite::new(
                self.player_spr.position.x - 0.8,
                self.player_spr.position.y + 0.3,
                1.0,
                1.0,
            ))
        } else {
            Some(Sprite::new(
                self.player_spr.position.x + 0.8,
                self.player_spr.position.y + 0.3,
                1.0,
                1.0,
            ))
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
    pub new_highscore: bool,
    pub enemies: Vec<Enemy>,
}

impl State {
    pub fn starting_state() -> Self {
        Self {
            perspective: cgmath::perspective(Deg(75.0), 800.0 / 600.0, 0.1, 1000.0),
            player: Player::start_state(),
            game_screen: GameScreen::MainMenu,
            level: Level::new(1, 1),
            left_mouse_held: false,
            new_highscore: false,
            enemies: vec![],
        }
    }

    pub fn player_position(&self) -> Vector2<f32> {
        self.player.player_spr.position
    }

    pub fn set_player_velocity_x(&mut self, vel_x: f32) {
        self.player.player_spr.velocity.x = vel_x;
    }

    pub fn set_player_velocity_y(&mut self, vel_y: f32) {
        self.player.player_spr.velocity.y = vel_y;
    }

    pub fn player_velocity(&self) -> Vector2<f32> {
        self.player.player_spr.velocity
    }
}
