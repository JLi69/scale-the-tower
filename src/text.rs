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
