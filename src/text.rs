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
            y
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

pub struct Button {
    pub text: Vec<u8>,
    //Position in pixels
    pub x: f32,
    pub y: f32,
    pub ch_sz: f32
}

impl Button {
    pub fn new(ascii_text: &[u8], posx: f32, posy: f32, ch_size: f32) -> Self {
        Self {
            text: Vec::from(ascii_text),
            x: posx,
            y: posy,
            ch_sz: ch_size
        }
    }

    pub fn display(
        &self, 
        rect_vao: &VertexArrayObject, 
        shader_program: &ShaderProgram,
        mouse_x: f32,
        mouse_y: f32,
        win_w: f32,
        win_h: f32
    ) {
        if self.mouse_hovering(mouse_x, mouse_y, win_w, win_h) { 
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
            self.ch_sz
        );
    }

    pub fn width(&self) -> f32 {
        2.0 * self.ch_sz * self.text.len() as f32
    }

    pub fn mouse_hovering(&self, mouse_x: f32, mouse_y: f32, win_w: f32, win_h: f32) -> bool {
        mouse_x - win_w / 2.0 >= self.x - self.width() / 2.0 &&
            mouse_x - win_w / 2.0 <= self.x + self.width() / 2.0 &&
            win_h / 2.0 - mouse_y >= self.y - self.ch_sz &&
            win_h / 2.0 - mouse_y <= self.y + self.ch_sz
    }
}
