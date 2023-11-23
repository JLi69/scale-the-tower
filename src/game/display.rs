use super::{Player, Projectile, State, ATTACK_TIMER};
use crate::{
    gfx::VertexArrayObject, level::display_level::SPRITE_RENDER_DISTANCE, shader::ShaderProgram, ui,
};
use cgmath::{Matrix4, Rad};

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

        //Attack animation
        let attack_hitbox = self.attack_hitbox();
        sprite_shader.uniform_bool("uFlipped", false);
        if let Some(hitbox) = attack_hitbox {
            let flip_matrix = if self.player_spr.flipped {
                Matrix4::from_angle_y(Rad(std::f32::consts::PI))
            } else {
                Matrix4::from_angle_y(Rad(0.0f32))
            };

            let transform_matrix =
                Matrix4::from_translation(cgmath::vec3(hitbox.position.x, hitbox.position.y, 0.0))
                    * Matrix4::from_scale(0.35)
                    * flip_matrix
                    * Matrix4::from_translation(cgmath::vec3(-1.0, -1.0, 0.0))
                    * Matrix4::from_angle_z(Rad(((std::f32::consts::PI / 2.0
                        * self.attack_timer
                        / ATTACK_TIMER)
                        * 1.8
                        - 0.8 * std::f32::consts::PI / 2.0)
                        .max(0.0)))
                    * Matrix4::from_translation(cgmath::vec3(1.0, 0.0, 0.0))
                    * Matrix4::from_angle_z(Rad(-std::f32::consts::PI / 4.0));
            sprite_shader.uniform_matrix4f("uTransform", &transform_matrix);
            sprite_shader.uniform_vec2f("uTexOffset", 0.0, 3.0 / 8.0);

            unsafe {
                gl::Disable(gl::CULL_FACE);
            }
            rect_vao.draw_arrays();
            unsafe {
                gl::Enable(gl::CULL_FACE);
            }
        }
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

impl State {
    pub fn display_enemies(&self, rect_vao: &VertexArrayObject, shader_program: &ShaderProgram) {
        for enemy in &self.enemies {
            if (enemy.sprite.position.y - self.player_position().y).abs() > SPRITE_RENDER_DISTANCE {
                continue;
            }

            enemy.display(rect_vao, shader_program);
        }
    }

    pub fn display_particles(&self, rect_vao: &VertexArrayObject, shader_program: &ShaderProgram) {
        for particle in &self.particles {
            if (particle.sprite.position.y - self.player_position().y).abs()
                > SPRITE_RENDER_DISTANCE
            {
                continue;
            }

            particle.display(rect_vao, shader_program);
        }
    }

    pub fn display_projectiles(
        &self,
        rect_vao: &VertexArrayObject,
        shader_program: &ShaderProgram,
    ) {
        for (projectile_type, spr) in &self.projectiles {
            if (spr.position.y - self.player_position().y).abs() > SPRITE_RENDER_DISTANCE {
                continue;
            }

            //Apply texture
            match *projectile_type {
                Projectile::Fireball => {
                    let transform_matrix = Matrix4::from_translation(cgmath::vec3(
                        spr.position.x,
                        spr.position.y,
                        0.0,
                    )) * Matrix4::from_scale(0.5 * 0.3);
                    shader_program.uniform_matrix4f("uTransform", &transform_matrix);
                    shader_program.uniform_vec2f("uTexOffset", 3.0 / 8.0, 3.0 / 8.0);
                }
                _ => {}
            }

            rect_vao.draw_arrays();
        }
    }
}
