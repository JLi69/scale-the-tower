#![allow(clippy::zero_ptr)]

extern crate gl;
extern crate glfw;

mod gfx;
mod level;
mod shader;
mod sprite;

use cgmath::{Deg, Matrix4};
use glfw::Context;
use level::room_template;
use level::Level;
use sprite::Sprite;
use std::{sync::mpsc::Receiver, time::Instant};

//Structure to store the current state of the application and allow us
//to pass it to different functions so that it can be modified
struct State {
    perspective: Matrix4<f32>,
    player: Sprite,
}

//Handle window resizing
fn handle_window_resize(w: i32, h: i32, state: &mut State) {
    unsafe {
        gl::Viewport(0, 0, w, h);
    }
    //Update the perspective matrix
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
        if key == glfw::Key::Up && !state.player.falling() && !state.player.climbing() {
            state.player.velocity.y = sprite::PLAYER_JUMP_SPEED;
        } else if key == glfw::Key::Up && state.player.climbing() {
            state.player.velocity.y = sprite::PLAYER_CLIMB_SPEED;
        } else if key == glfw::Key::Down && state.player.climbing() {
            state.player.velocity.y = -sprite::PLAYER_CLIMB_SPEED;
        } else if key == glfw::Key::Left {
            state.player.velocity.x = -sprite::PLAYER_SPEED;
        } else if key == glfw::Key::Right {
            state.player.velocity.x = sprite::PLAYER_SPEED;
        }
    } else if action == glfw::Action::Release {
        if (key == glfw::Key::Up || key == glfw::Key::Down) && state.player.climbing() {
            state.player.velocity.y = 0.0;
        } else if key == glfw::Key::Left && state.player.velocity.x < 0.0 {
            state.player.velocity.x = 0.0;
        } else if key == glfw::Key::Right && state.player.velocity.x > 0.0 {
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
            //Window resize
            glfw::WindowEvent::FramebufferSize(width, height) => {
                handle_window_resize(width, height, state);
            }
            //Key input
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
        match glfw.create_window(800, 600, "Scale the Tower", glfw::WindowMode::Windowed) {
            Some(win) => win,
            _ => panic!("Failed to create window!"),
        };
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    //Attempt to load OpenGL functions
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    //Attempt to load room templates
    let room_templates = room_template::load_room_templates("assets/room_templates");

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::CULL_FACE);
        gl::ClearColor(0.5, 0.8, 1.0, 1.0);
    }

    let _cube_vao = gfx::VertexArrayObject::create_cube();
    let rect_vao = gfx::VertexArrayObject::create_rectangle();

    //Load Shaders
    let sprite_shader = shader::program_from_vert_and_frag(
        "assets/shaders/sprite_vert.glsl",
        "assets/shaders/sprite_frag.glsl",
    );

    let level_shader = shader::program_from_vert_and_frag(
        "assets/shaders/level_vert.glsl",
        "assets/shaders/level_frag.glsl",
    );

    //Load Textures
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

    //Initialize the current state of the application
    let mut state = State {
        perspective: cgmath::perspective(Deg(75.0), 800.0 / 600.0, 0.1, 1000.0),
        player: Sprite::new(1.0, 1.0, 0.8, 1.0),
    };

    let mut level = Level::generate_level(&room_templates);
    level.build_chunks();

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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }

        //Display level
        tile_textures.bind();
        level_shader.use_program();
        level_shader.uniform_matrix4f("uPerspective", &state.perspective);
        level_shader.uniform_matrix4f("uView", &view_matrix);
        let transform_matrix = Matrix4::from_scale(0.5);
        level_shader.uniform_matrix4f("uTransform", &transform_matrix);
        level.display();

        //Display the player sprite
        rect_vao.bind();
        sprite_shader.use_program();
        sprite_shader.uniform_matrix4f("uPerspective", &state.perspective);
        sprite_shader.uniform_matrix4f("uView", &view_matrix);
        sprite_shader.uniform_float("uTexScale", 1.0 / 8.0);
        sprite_shader.uniform_bool("uFlipped", state.player.flipped);
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

        //Calculate the amount of time passed in a single frame
        let end = Instant::now();
        dt = end.duration_since(start).as_secs_f32();
    }

    Ok(())
}
