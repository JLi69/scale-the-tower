use super::Sprite;
use crate::{gfx::VertexArrayObject, shader::ShaderProgram};
use cgmath::Matrix4;

pub enum EnemyType {
    Slime,
}

pub struct Enemy {
    pub sprite: Sprite,
    pub enemy_type: EnemyType,
}

impl Enemy {
    //Create a new enemy
    pub fn new(x: f32, y: f32, w: f32, h: f32, enemy: EnemyType) -> Self {
        let mut spr = Sprite::new(x, y, w, h);

        match enemy {
            EnemyType::Slime => spr.set_animation(0.5, 0, 1),
        }

        Self {
            sprite: spr,
            enemy_type: enemy,
        }
    }

    pub fn display(&self, rect_vao: &VertexArrayObject, shader_program: &ShaderProgram) {
        shader_program.uniform_bool("uFlipped", self.sprite.flipped);

        let transform_matrix = Matrix4::from_translation(cgmath::vec3(
            self.sprite.position.x,
            self.sprite.position.y,
            0.0,
        )) * Matrix4::from_scale(0.5);
        shader_program.uniform_matrix4f("uTransform", &transform_matrix);

        //Apply texture
        match self.enemy_type {
            EnemyType::Slime => {
                shader_program.uniform_vec2f(
                    "uTexOffset",
                    1.0 / 8.0 * self.sprite.current_frame() as f32,
                    1.0 / 8.0,
                );
            }
        }

        rect_vao.draw_arrays();
    }
}
