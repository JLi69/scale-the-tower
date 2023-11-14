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
}

pub struct Button {
    pub text: Vec<u8>,
    //Position in pixels
    pub x: f32,
    pub y: f32,
    pub ch_sz: f32,
    pub click_action: ButtonAction,
}

impl Button {
    //Create a new button
    pub fn new(
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
            click_action: action,
        }
    }

    //Displays the button to the screen, if the mouse is hovering over
    //the button than the button will be darkened to indicate the mouse
    //is hovering over it
    pub fn display(
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

    //Gets the width of the button
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

pub fn create_pause_menu() -> Vec<Button> {
    vec![
        //Go to main menu
        Button::new(b"Main Menu", 0.0, 0.0, 16.0, ButtonAction::GotoMainMenu),
        //Quit game
        Button::new(b"Quit", 0.0, -48.0, 16.0, ButtonAction::QuitGame),
    ]
}

pub fn create_main_menu() -> Vec<Button> {
    vec![
        //Start game
        Button::new(b"Start!", 0.0, -0.0, 16.0, ButtonAction::StartGame),
        //Quit game
        Button::new(b"Quit", 0.0, -120.0, 16.0, ButtonAction::QuitGame),
        //Go to credits
        Button::new(b"Credits", 0.0, -60.0, 16.0, ButtonAction::QuitGame),
    ]
}

pub fn display_menu(
    menu: &Vec<Button>,
    rect_vao: &VertexArrayObject,
    text_shader: &ShaderProgram,
    win_info: &WindowInfo,
) {
    for button in menu {
        button.display(rect_vao, text_shader, win_info)
    }
}

pub fn get_clicked_button_action(
    menu: &Vec<Button>,
    win_info: &WindowInfo,
) -> Option<ButtonAction> {
    for button in menu {
        if button.mouse_hovering(win_info) {
            return Some(button.click_action);
        }
    }
    None
}
