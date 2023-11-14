use super::Player;
use crate::{gfx::VertexArrayObject, shader::ShaderProgram, ui};
use cgmath::Matrix4;

impl Player {
    pub fn display_player(&self, rect_vao: &VertexArrayObject, sprite_shader: &ShaderProgram) {
        //Display the player sprite
        sprite_shader.uniform_bool("uFlipped", self.player_spr.flipped);
        let transform_matrix = Matrix4::from_translation(cgmath::vec3(
            self.player_spr.position.x,
            self.player_spr.position.y,
            0.0,
        )) * Matrix4::from_scale(0.5);
        sprite_shader.uniform_matrix4f("uTransform", &transform_matrix);
        sprite_shader.uniform_vec2f(
            "uTexOffset",
            1.0 / 8.0 * self.player_spr.current_frame() as f32,
            0.0,
        );
        rect_vao.draw_arrays();
    }

    pub fn display_player_stats(
        &self,
        rect_vao: &VertexArrayObject,
        text_shader: &ShaderProgram,
        window: &glfw::Window,
    ) {
        let (win_w, win_h) = window.get_size();

        ui::display_ascii_text(
            rect_vao,
            text_shader,
            format!("score:{}", self.score).as_bytes(),
            -win_w as f32 / 2.0 + 24.0,
            win_h as f32 / 2.0 - 48.0,
            8.0,
        );
        ui::display_ascii_text(
            rect_vao,
            text_shader,
            format!("height:{}m", self.player_spr.position.y.round() as i32 - 1).as_bytes(),
            -win_w as f32 / 2.0 + 24.0,
            win_h as f32 / 2.0 - 72.0,
            8.0,
        );
        ui::display_health_bar(
            rect_vao,
            text_shader,
            self.player_health,
            self.max_player_health,
            -win_w as f32 / 2.0 + 24.0,
            win_h as f32 / 2.0 - 24.0,
        );
    }
}
