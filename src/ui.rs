use std::io::Read;

use crate::gfx::VertexArrayObject;
use crate::shader::ShaderProgram;

pub const ICONS_TEXTURE_SCALE: f32 = 16.0;

//Displays a string of text on the screen
//text is an array of bytes representing an ascii string
//top left of text is (x, y) and ch_size is in pixels
pub fn display_ascii_text(
    rect_vao: &VertexArrayObject,
    shader_program: &ShaderProgram,
    text: &[u8],
    x: f32,
    y: f32,
    ch_size: f32,
) {
    shader_program.uniform_float("uScale", ch_size);
    for (i, c) in text.iter().enumerate() {
        shader_program.uniform_vec2f("uPosition", x + ch_size * i as f32 * 2.0, y);

        let ch = if c.is_ascii_alphabetic() {
            text[i].to_ascii_uppercase()
        } else {
            text[i]
        };

        let tex_x = ((ch - b' ') % ICONS_TEXTURE_SCALE as u8) as f32 * 1.0 / ICONS_TEXTURE_SCALE;
        let tex_y =
            ((ch - b' ') / ICONS_TEXTURE_SCALE as u8 + 2) as f32 * 1.0 / ICONS_TEXTURE_SCALE;
        shader_program.uniform_vec2f("uTexOffset", tex_x, tex_y);

        rect_vao.draw_arrays();
    }
}

//Displays a string of text on the screen
//text is an array of bytes representing an ascii string
//center of text is (x, y) and ch_size is in pixels
pub fn display_ascii_text_centered(
    rect_vao: &VertexArrayObject,
    shader_program: &ShaderProgram,
    text: &[u8],
    x: f32,
    y: f32,
    ch_size: f32,
) {
    shader_program.uniform_float("uScale", ch_size);
    for (i, c) in text.iter().enumerate() {
        shader_program.uniform_vec2f(
            "uPosition",
            x + ch_size * i as f32 * 2.0 - ch_size * text.len() as f32 + ch_size,
            y,
        );

        let ch = if c.is_ascii_alphabetic() {
            text[i].to_ascii_uppercase()
        } else {
            text[i]
        };

        let tex_x = ((ch - b' ') % ICONS_TEXTURE_SCALE as u8) as f32 * 1.0 / ICONS_TEXTURE_SCALE;
        let tex_y =
            ((ch - b' ') / ICONS_TEXTURE_SCALE as u8 + 2) as f32 * 1.0 / ICONS_TEXTURE_SCALE;
        shader_program.uniform_vec2f("uTexOffset", tex_x, tex_y);

        rect_vao.draw_arrays();
    }
}

pub fn display_health_bar(
    rect_vao: &VertexArrayObject,
    shader_program: &ShaderProgram,
    health: i32,
    max_health: i32,
    x: f32,
    y: f32,
) {
    //Display player health
    shader_program.uniform_float("uScale", 12.0);
    shader_program.uniform_vec2f("uTexOffset", 0.0, 1.0 / 16.0);

    if health < 0 {
        shader_program.uniform_vec2f("uTexOffset", 1.0 / 16.0, 1.0 / 16.0);
    }

    for hp in 0..max_health {
        if hp == health {
            shader_program.uniform_vec2f("uTexOffset", 1.0 / 16.0, 1.0 / 16.0);
        }
        shader_program.uniform_vec2f("uPosition", x + hp as f32 * 24.0, y);
        rect_vao.draw_arrays();
    }
}

pub struct WindowInfo {
    pub win_w: f32,
    pub win_h: f32,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

#[derive(Copy, Clone)]
pub enum ButtonAction {
    QuitGame,
    GotoMainMenu,
    StartGame,
    GotoHighScores,
    GotoAbout,
}

pub struct MenuElement {
    pub text: Vec<u8>,
    //Position in pixels
    pub x: f32,
    pub y: f32,
    pub ch_sz: f32,
    pub click_action: Option<ButtonAction>,
}

impl MenuElement {
    //Create a new menu element
    pub fn button(
        ascii_text: &[u8],
        posx: f32,
        posy: f32,
        ch_size: f32,
        action: ButtonAction,
    ) -> Self {
        Self {
            text: Vec::from(ascii_text),
            x: posx,
            y: posy,
            ch_sz: ch_size,
            click_action: Some(action),
        }
    }

    pub fn text(ascii_text: &[u8], posx: f32, posy: f32, ch_size: f32) -> Self {
        Self {
            text: Vec::from(ascii_text),
            x: posx,
            y: posy,
            ch_sz: ch_size,
            click_action: None,
        }
    }

    //Displays the button to the screen, if the mouse is hovering over
    //the button than the button will be darkened to indicate the mouse
    //is hovering over it
    pub fn display_button(
        &self,
        rect_vao: &VertexArrayObject,
        shader_program: &ShaderProgram,
        win_info: &WindowInfo,
    ) {
        if self.mouse_hovering(win_info) {
            shader_program.uniform_vec4f("uColor", 0.5, 0.5, 0.5, 1.0);
        } else {
            shader_program.uniform_vec4f("uColor", 1.0, 1.0, 1.0, 1.0);
        }

        display_ascii_text_centered(
            rect_vao,
            shader_program,
            &self.text,
            self.x,
            self.y,
            self.ch_sz,
        );
    }

    //Displays text onto the screen as a menu element
    pub fn display_text(&self, rect_vao: &VertexArrayObject, shader_program: &ShaderProgram) {
        shader_program.uniform_vec4f("uColor", 1.0, 1.0, 1.0, 1.0);
        display_ascii_text_centered(
            rect_vao,
            shader_program,
            &self.text,
            self.x,
            self.y,
            self.ch_sz,
        );
    }

    //Gets the width of the menu element
    pub fn width(&self) -> f32 {
        2.0 * self.ch_sz * self.text.len() as f32
    }

    //Returns if the mouse is hovering over the button
    pub fn mouse_hovering(&self, win_info: &WindowInfo) -> bool {
        win_info.mouse_x - win_info.win_w / 2.0 >= self.x - self.width() / 2.0
            && win_info.mouse_x - win_info.win_w / 2.0 <= self.x + self.width() / 2.0
            && win_info.win_h / 2.0 - win_info.mouse_y >= self.y - self.ch_sz
            && win_info.win_h / 2.0 - win_info.mouse_y <= self.y + self.ch_sz
    }
}

pub struct Menu {
    text: Vec<MenuElement>,
    buttons: Vec<MenuElement>,
}

impl Menu {
    pub fn create_main_menu() -> Self {
        Self {
            buttons: vec![
                //Start game
                MenuElement::button(b"Start!", 0.0, -0.0, 16.0, ButtonAction::StartGame),
                //Go to highscores
                MenuElement::button(
                    b"High Scores",
                    0.0,
                    -60.0,
                    16.0,
                    ButtonAction::GotoHighScores,
                ),
                //Go to about page
                MenuElement::button(b"About", 0.0, -120.0, 16.0, ButtonAction::GotoAbout),
                //Quit game
                MenuElement::button(b"Quit", 0.0, -180.0, 16.0, ButtonAction::QuitGame),
            ],
            text: vec![
                MenuElement::text(b"Scale the Tower", 0.0, 180.0, 22.0),
                MenuElement::text(b"Created for the 2023 Game Off Jam", 0.0, 80.0, 8.0),
            ],
        }
    }

    pub fn create_hiscore_menu() -> Self {
        Self {
            buttons: vec![
                //Go to main menu
                MenuElement::button(b"Main Menu", 0.0, -192.0, 16.0, ButtonAction::GotoMainMenu),
            ],
            text: vec![MenuElement::text(b"High Scores", 0.0, 192.0, 16.0)],
        }
    }

    pub fn create_pause_menu() -> Self {
        Self {
            buttons: vec![
                //Go to main menu
                MenuElement::button(b"Main Menu", 0.0, 0.0, 16.0, ButtonAction::GotoMainMenu),
                //Quit game
                MenuElement::button(b"Quit", 0.0, -48.0, 16.0, ButtonAction::QuitGame),
            ],
            text: vec![
                MenuElement::text(b"Paused", 0.0, 128.0, 32.0),
                MenuElement::text(b"Press Escape to Unpause", 0.0, 48.0, 8.0),
            ],
        }
    }

    pub fn create_gameover_menu() -> Self {
        Self {
            buttons: vec![
                //Go to main menu
                MenuElement::button(b"Main Menu", 0.0, -24.0, 16.0, ButtonAction::GotoMainMenu),
                //Quit game
                MenuElement::button(b"Quit", 0.0, -72.0, 16.0, ButtonAction::QuitGame),
            ],
            text: vec![MenuElement::text(b"Game Over!", 0.0, 96.0, 24.0)],
        }
    }

    pub fn create_win_screen() -> Self {
        Self {
            buttons: vec![
                //Go to main menu
                MenuElement::button(b"Main Menu", 0.0, -88.0, 16.0, ButtonAction::GotoMainMenu),
                //Quit game
                MenuElement::button(b"Quit", 0.0, -136.0, 16.0, ButtonAction::QuitGame),
            ],
            text: vec![
                MenuElement::text(b"You Did It!", 0.0, 180.0, 24.0),
                MenuElement::text(b"You scaled the tower!", 0.0, 126.0, 8.0),
            ],
        }
    }

    pub fn create_about_screen() -> Self {
        let about_text = match std::fs::File::open("assets/about.txt") {
            Ok(mut file) => {
                let mut about = String::new();
                let res = file.read_to_string(&mut about);
                if let Err(msg) = res {
                    eprintln!("{msg}"); 
                }

                about
                    .lines()
                    .enumerate()
                    .map(|(i, line)| MenuElement::text(line.as_bytes(), 0.0, i as f32 * -20.0 + 160.0, 8.0))
                    .collect()
            }
            Err(msg) => {
                eprintln!("{msg}");
                vec![]
            },
        };

        Self {
            buttons: vec![
                MenuElement::button(b"Main Menu", 0.0, -256.0, 16.0, ButtonAction::GotoMainMenu),
            ],
            text: about_text,
        }
    }

    pub fn display(
        &self,
        rect_vao: &VertexArrayObject,
        text_shader: &ShaderProgram,
        win_info: &WindowInfo,
    ) {
        for text in &self.text {
            text.display_text(rect_vao, text_shader)
        }

        for button in &self.buttons {
            button.display_button(rect_vao, text_shader, win_info)
        }
    }

    pub fn get_clicked_button_action(&self, win_info: &WindowInfo) -> Option<ButtonAction> {
        for button in &self.buttons {
            if button.mouse_hovering(win_info) {
                return button.click_action;
            }
        }
        None
    }
}
