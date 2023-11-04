extern crate gl;
extern crate glfw;

mod gfx;
mod level;
mod shader;
mod sprite;

use cgmath::{Deg, Matrix4};
use glfw::Context;
use level::Level;
use sprite::Sprite;
use std::{sync::mpsc::Receiver, time::Instant};

struct State {
    perspective: Matrix4<f32>,
    player: Sprite,
}

fn handle_window_resize(w: i32, h: i32, state: &mut State) {
    unsafe {
        gl::Viewport(0, 0, w, h);
    }
    state.perspective = cgmath::perspective(cgmath::Deg(75.0), w as f32 / h as f32, 0.1, 1000.0)
}

//Handle key input
fn handle_key_input(
    key: glfw::Key,
    _scancode: glfw::Scancode,
    action: glfw::Action,
    _modifiers: glfw::Modifiers,
    state: &mut State,
) {
    if action == glfw::Action::Press {
        if key == glfw::Key::Up && !state.player.falling() {
            state.player.velocity.y = sprite::PLAYER_JUMP_SPEED;
        } else if key == glfw::Key::Left {
            state.player.velocity.x = -sprite::PLAYER_SPEED;
        } else if key == glfw::Key::Right {
            state.player.velocity.x = sprite::PLAYER_SPEED;
        }
    } else if action == glfw::Action::Release {
        if key == glfw::Key::Left {
            state.player.velocity.x = 0.0;
        } else if key == glfw::Key::Right {
            state.player.velocity.x = 0.0;
        }
    }
}

fn process_events(
    _window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
    state: &mut State,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                handle_window_resize(width, height, state);
            }
            glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                handle_key_input(key, scancode, action, modifiers, state);
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), String> {
    //Attempt to initialize glfw
    let mut glfw = glfw::init_no_callbacks().map_err(|e| e.to_string())?;
    //Attempt to create window
    let (mut window, events) =
        match glfw.create_window(800, 600, "Github Game Off 2023", glfw::WindowMode::Windowed) {
            Some(win) => win,
            _ => panic!("Failed to create window!"),
        };
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    //Attempt to load OpenGL functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::CULL_FACE);
    }

    let cube_vao = gfx::VertexArrayObject::create_cube();
    let rect_vao = gfx::VertexArrayObject::create_rectangle();

    let sprite_shaders = [
        shader::create_and_compile_shader("assets/shaders/sprite_vert.glsl", gl::VERTEX_SHADER),
        shader::create_and_compile_shader("assets/shaders/sprite_frag.glsl", gl::FRAGMENT_SHADER),
    ];
    let sprite_shader = shader::ShaderProgram::create_program();
    sprite_shader.add_shaders(&sprite_shaders);

    //Textures
    let sprite_textures = match gfx::Texture::load_from_file("assets/textures/sprites.png") {
        Ok(texture) => texture,
        Err(msg) => {
            eprintln!("{msg}");
            gfx::Texture::new()
        }
    };

    let tile_textures = match gfx::Texture::load_from_file("assets/textures/tiles.png") {
        Ok(texture) => texture,
        Err(msg) => {
            eprintln!("{msg}");
            gfx::Texture::new()
        }
    };

    let mut state = State {
        perspective: cgmath::perspective(Deg(75.0), 800.0 / 600.0, 0.1, 1000.0),
        player: Sprite::new(8.0, 8.0, 1.0, 1.0),
    };

    let level = Level::test_level();

    state.player.update_animation_state();

    let mut dt = 0.0f32;
    while !window.should_close() {
        let start = Instant::now();

        process_events(&mut window, &events, &mut state);

        let view_matrix = Matrix4::from_translation(cgmath::vec3(
            -state.player.position.x,
            -state.player.position.y,
            level::LEVEL_Z,
        ));
        sprite_shader.use_program();
        sprite_shader.uniform_matrix4f("uPerspective", &state.perspective);
        sprite_shader.uniform_matrix4f("uView", &view_matrix);
        sprite_shader.uniform_float("uTexScale", 1.0 / 8.0);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }

        //Display level
        sprite_shader.uniform_bool("uFlipped", false);
        tile_textures.bind();
        level.display(&sprite_shader, &cube_vao);

        sprite_shader.uniform_bool("uFlipped", state.player.flipped);
        rect_vao.bind();
        sprite_textures.bind();
        let transform_matrix = Matrix4::from_translation(cgmath::vec3(
            state.player.position.x,
            state.player.position.y,
            0.0,
        )) * Matrix4::from_scale(0.5);
        sprite_shader.uniform_matrix4f("uTransform", &transform_matrix);
        sprite_shader.uniform_vec2f(
            "uTexOffset",
            1.0 / 8.0 * state.player.current_frame() as f32,
            0.0,
        );
        rect_vao.draw_arrays();

        //Update the player
        state.player.update(dt, &level);
        state.player.update_animation_frame(dt);
        state.player.update_animation_state();

        gfx::output_gl_errors();
        window.swap_buffers();
        glfw.poll_events();

        let end = Instant::now();
        dt = end.duration_since(start).as_secs_f32();
    }

    Ok(())
}
