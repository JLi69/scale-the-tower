extern crate gl;
extern crate glfw;

mod shader;
mod level;
mod gfx;
mod sprite;

use glfw::Context;
use level::Level;
use cgmath::{Matrix4, Deg};
use std::{sync::mpsc::Receiver, time::Instant};
use sprite::Sprite;

struct State {
    perspective: Matrix4<f32>,
    player: Sprite
}

fn handle_window_resize(w: i32, h: i32, state: &mut State) {
    unsafe {
        gl::Viewport(0, 0, w, h);
    }
    state.perspective = cgmath::perspective(
        cgmath::Deg(75.0),
        w as f32 / h as f32,
        0.1,
        1000.0,
    )
}

//Handle key input
fn handle_key_input(
    key: glfw::Key, 
    _scancode: glfw::Scancode, 
    action: glfw::Action,
    _modifiers: glfw::Modifiers,
    state: &mut State
) {
    if action == glfw::Action::Press {
        if key == glfw::Key::Up && !state.player.falling { 
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

    let shaders = [
        shader::create_and_compile_shader("assets/shaders/vert.glsl", gl::VERTEX_SHADER),
        shader::create_and_compile_shader("assets/shaders/frag.glsl", gl::FRAGMENT_SHADER)
    ];
    let program = shader::ShaderProgram::create_program();
    program.add_shaders(&shaders);
    program.use_program(); 

    let mut state = State {
        perspective: cgmath::perspective(Deg(75.0), 800.0 / 600.0, 0.1, 1000.0),
        player: Sprite::new(8.0, 8.0, 1.0, 1.0)
    };

    let level = Level::test_level();

    let mut dt = 0.0f32;
    while !window.should_close() {
        let start = Instant::now();

        process_events(&mut window, &events, &mut state);

        program.uniform_matrix4f("uPerspective", &state.perspective);
        let view_matrix = Matrix4::from_translation(
                cgmath::vec3(-state.player.position.x, -state.player.position.y, level::LEVEL_Z) 
            );
        program.uniform_matrix4f("uView", &view_matrix);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT); 
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }

        //Display level 
        level.display(&program, &cube_vao);

        rect_vao.bind();
        program.uniform_vec4f("uColor", 1.0, 1.0, 0.0, 1.0); 
        let transform_matrix = 
            Matrix4::from_translation(
                cgmath::vec3(
                    state.player.position.x,
                    state.player.position.y,
                    0.0)
                ) * 
            Matrix4::from_scale(0.5);
        program.uniform_matrix4f("uTransform", &transform_matrix); 
        rect_vao.draw_arrays();

        //Update the player
        state.player.update(dt, &level);

        gfx::output_gl_errors();
        window.swap_buffers();
        glfw.poll_events();

        let end = Instant::now();
        dt = end.duration_since(start).as_secs_f32();
    }

    Ok(())
}
