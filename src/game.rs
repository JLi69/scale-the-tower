use crate::{
    audio::{sfx_ids, SfxPlayer},
    sprite::{enemy::Enemy, particle::Particle},
    Level, Sprite,
};
use cgmath::{Deg, Matrix4, Vector2};

use input_config::InputConfig;

pub mod display;
pub mod hiscore;
pub mod input_config;
pub mod player;
pub mod update_game;

//Constants
//Force of gravity on all sprites
pub const GRAVITY: f32 = 16.0;
pub const DEFAULT_PLAYER_HEALTH: i32 = 4;
pub const DAMAGE_COOLDOWN: f32 = 0.3;
pub const ATTACK_COOLDOWN: f32 = 0.5;
pub const ATTACK_TIMER: f32 = 0.2;
pub const PLAYER_WIDTH: f32 = 0.75;
pub const PLAYER_HEIGHT: f32 = 0.8125;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum GameScreen {
    MainMenu,
    Game,
    Paused,
    GameOver,
    WinScreen,
    HighScores,
    AboutScreen,
}

#[derive(Eq, PartialEq)]
pub enum Weapon {
    Sword,
    Bow,
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
    pub arrows: u32,
    pub weapon: Weapon,
}

impl Player {
    pub fn start_state() -> Self {
        Self {
            player_spr: Sprite::new(
                1.0,
                1.0 - (1.0 - PLAYER_HEIGHT) / 2.0,
                PLAYER_WIDTH,
                PLAYER_HEIGHT,
            ),
            score: 0,
            player_health: DEFAULT_PLAYER_HEALTH,
            max_player_health: DEFAULT_PLAYER_HEALTH,
            damage_cooldown: 0.0,
            falling: false,
            climbing: false,
            attack_cooldown: 0.0,
            attack_timer: 0.0,
            arrows: 3,
            weapon: Weapon::Sword,
        }
    }

    //Returns true if we applied damage to the player, false otherwise
    pub fn apply_damage(&mut self, amount: i32) -> bool {
        if self.damage_cooldown <= 0.0 && amount > 0 && self.player_health > 0 {
            self.player_health -= amount;
            self.damage_cooldown = DAMAGE_COOLDOWN;
            return true;
        }

        false
    }

    //Returns if the player is falling
    pub fn falling(&self) -> bool {
        self.falling
    }

    //Returns if the player is climbing
    pub fn climbing(&self) -> bool {
        self.climbing
    }

    pub fn shoot(&mut self) -> Option<Sprite> {
        if self.attack_cooldown <= 0.0 && self.arrows > 0 {
            self.attack_cooldown = ATTACK_COOLDOWN;
            self.attack_timer = ATTACK_TIMER;
            let player_pos = self.player_spr.position;
            let mut arrow = Sprite::new(player_pos.x, player_pos.y - 0.1, 0.5, 0.5);

            let (offset, vel_x) = if self.player_spr.flipped {
                (-0.8, -6.0 + self.player_spr.velocity.x)
            } else {
                (0.8, 6.0 + self.player_spr.velocity.x)
            };
            arrow.position.x += offset;
            arrow.velocity.x = vel_x;
            arrow.flipped = self.player_spr.flipped;
            self.arrows -= 1;

            return Some(arrow);
        }
        None
    }

    pub fn attack(&mut self) {
        if self.attack_cooldown <= 0.0 {
            self.attack_cooldown = ATTACK_COOLDOWN;
            self.attack_timer = ATTACK_TIMER;
        }
    }

    //Returns None if the attack cooldown isn't at 0 yet
    pub fn attack_hitbox(&self) -> Option<Sprite> {
        if self.attack_timer < 0.0 || self.player_health <= 0 || self.weapon == Weapon::Bow {
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

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Projectile {
    Fireball,
    Arrow,
    Destroyed,
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
    pub projectiles: Vec<(Projectile, Sprite)>,
    pub particles: Vec<Particle>,
    pub input: InputConfig,
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
            projectiles: vec![],
            particles: vec![],
            input: InputConfig::new("input_settings"),
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

    pub fn player_attack(&mut self) {
        match self.player.weapon {
            Weapon::Sword => self.player.attack(),
            Weapon::Bow => {
                let spr = self.player.shoot();
                if let Some(arrow) = spr {
                    self.projectiles.push((Projectile::Arrow, arrow));
                }
            }
        }
    }

    fn handle_up_key(&mut self, sfx_player: &SfxPlayer) {
        if !self.player.falling() && !self.player.climbing() {
            self.set_player_velocity_y(player::PLAYER_JUMP_SPEED);
            sfx_player.play(sfx_ids::JUMP);
        } else if self.player.climbing() {
            self.set_player_velocity_y(player::PLAYER_CLIMB_SPEED);
        }
    }

    fn handle_down_key(&mut self) {
        if self.player.climbing() {
            self.set_player_velocity_y(-player::PLAYER_CLIMB_SPEED);
        }
    }

    pub fn handle_key_press(&mut self, scancode: input_config::KeyId, sfx_player: &SfxPlayer) {
        let action = self.input.get_action(scancode).unwrap_or("".to_string());
        if action == "Escape" {
            match self.game_screen {
                GameScreen::Game => self.game_screen = GameScreen::Paused,
                GameScreen::Paused => self.game_screen = GameScreen::Game,
                _ => self.game_screen = GameScreen::MainMenu,
            }
        }

        //Ignore key presses if we aren't in the actual game
        if self.game_screen != GameScreen::Game {
            return;
        }

        if action == "Up" {
            self.handle_up_key(sfx_player);
        } else if action == "Down" {
            self.handle_down_key();
        } else if action == "Left" {
            self.set_player_velocity_x(-player::PLAYER_SPEED);
        } else if action == "Right" {
            self.set_player_velocity_x(player::PLAYER_SPEED);
        } else if action == "Attack" {
            self.player_attack();
        }

        if action == "Sword" {
            self.player.weapon = Weapon::Sword;
        } else if action == "Bow" {
            self.player.weapon = Weapon::Bow;
        }
    }

    pub fn handle_key_release(&mut self, scancode: input_config::KeyId) {
        //Ignore key presses if we aren't in the actual game
        if self.game_screen != GameScreen::Game {
            return;
        }

        let action = self.input.get_action(scancode).unwrap_or("".to_string());
        if action == "Up" || action == "Down" {
            if self.player.climbing() {
                self.set_player_velocity_y(0.0);
            }
        } else if action == "Left" {
            if self.player_velocity().x < 0.0 {
                self.set_player_velocity_x(0.0);
            }
        } else if action == "Right" {
            if self.player_velocity().x > 0.0 {
                self.set_player_velocity_x(0.0);
            }
        }
    }
}
