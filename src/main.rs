#![allow(clippy::zero_ptr)]

extern crate gl;
extern crate glfw;

mod gfx;
mod level;
mod shader;
mod sprite;
mod text;

use cgmath::{Deg, Matrix4};
use gfx::Texture;
use glfw::Context;
use level::room_template;
use level::{Level, Tile};
use sprite::Sprite;
use std::{sync::mpsc::Receiver, time::Instant};

const DEFAULT_PLAYER_HEALTH: i32 = 4;
const MAX_SAFE_FALL_SPEED: f32 = 14.0;
const DAMAGE_COOLDOWN: f32 = 0.3;

#[derive(Eq, PartialEq, Copy, Clone)]
enum GameScreen {
    Game,
    Paused,
    GameOver
}

//Structure to store the current state of the application and allow us
//to pass it to different functions so that it can be modified
pub struct State {
    perspective: Matrix4<f32>,
    player: Sprite,
    score: u32,

    player_health: i32,
    max_player_health: i32,
    damage_cooldown: f32,

    game_screen: GameScreen
}

fn apply_damage(state: &mut State, amount: i32) {
    if state.damage_cooldown <= 0.0 && amount > 0 {
        state.player_health -= amount;
        state.damage_cooldown = DAMAGE_COOLDOWN;
    }
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
        if key == glfw::Key::Escape {
            match state.game_screen {
                GameScreen::Game => state.game_screen = GameScreen::Paused,
                GameScreen::Paused => state.game_screen = GameScreen::Game,
                _ => {}
            }
        }

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
        } else if key == glfw::Key::Left && state.player.velocity.x < 0.0
            || key == glfw::Key::Right && state.player.velocity.x > 0.0
        {
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

fn load_texture(path: &str) -> Texture {
    match gfx::Texture::load_from_file(path) {
        Ok(texture) => texture,
        Err(msg) => {
            eprintln!("{msg}");
            gfx::Texture::new()
        }
    }
}

fn update_game_screen(state: &mut State, level: &mut Level, dt: f32) {
    //Update the player
    let falling = state.player.falling();
    let velocity_y = state.player.velocity.y;
    state.player.update(dt, level);
    //Hit the ground, apply fall damage if player is travelling fast enough
    if falling && !state.player.falling() && velocity_y < -MAX_SAFE_FALL_SPEED {
        apply_damage(
            state,
            -((velocity_y + MAX_SAFE_FALL_SPEED) / 12.0).floor() as i32,
        );
    }
    //Kill player instantly upon contact with lava
    if state.player.touching_tile(Tile::Lava, level) {
        state.player_health = 0;
    }
    //Kill player instantly when jumping onto spikes
    if state.player.touching_tile(Tile::Spikes, level) &&
        state.player.velocity.y < -1.0 {
        state.player_health = 0;
    }
    state.player.update_animation_frame(dt);
    state.player.update_animation_state();
    level.update_interactive_tiles(state);
    state.damage_cooldown -= dt;

    if state.player_health <= 0 {
        state.game_screen = GameScreen::GameOver; 
    }
}

fn display_player(
    rect_vao: &gfx::VertexArrayObject,
    sprite_shader: &shader::ShaderProgram,
    state: &State,
) {
    //Display the player sprite 
    sprite_shader.uniform_bool("uFlipped", state.player.flipped);
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
}

fn display_game_screen_hud(
    rect_vao: &gfx::VertexArrayObject,
    text_shader: &shader::ShaderProgram,
    state: &State,
    window: &glfw::Window
) {  
    let (win_w, win_h) = window.get_size();

    text::display_ascii_text(
        rect_vao,
        text_shader,
        format!("score:{}", state.score).as_bytes(),
        -win_w as f32 / 2.0 + 24.0,
        win_h as f32 / 2.0 - 48.0,
        8.0,
    );
    text::display_ascii_text(
        rect_vao,
        text_shader,
        format!("height:{}m", state.player.position.y.round() as i32 - 1).as_bytes(),
        -win_w as f32 / 2.0 + 24.0,
        win_h as f32 / 2.0 - 72.0,
        8.0,
    );
    text::display_health_bar(
        rect_vao,
        text_shader,
        state.player_health,
        state.max_player_health,
        -win_w as f32 / 2.0 + 24.0,
        win_h as f32 / 2.0 - 24.0,
    ); 
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
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
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

    let cube_vao = gfx::VertexArrayObject::create_cube();
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

    let text_shader = shader::program_from_vert_and_frag(
        "assets/shaders/text_vert.glsl",
        "assets/shaders/text_frag.glsl",
    );

    let rect_shader = shader::program_from_vert_and_frag(
        "assets/shaders/rect_vert.glsl",
        "assets/shaders/rect_frag.glsl",
    );

    //Load Textures
    let sprite_textures = load_texture("assets/textures/sprites.png");
    let tile_textures = load_texture("assets/textures/tiles.png");
    let icons = load_texture("assets/textures/icons.png");

    //Initialize the current state of the application
    let mut state = State {
        perspective: cgmath::perspective(Deg(75.0), 800.0 / 600.0, 0.1, 1000.0),
        player: Sprite::new(1.0, 1.0, 0.8, 1.0),
        score: 0,
        player_health: DEFAULT_PLAYER_HEALTH,
        max_player_health: DEFAULT_PLAYER_HEALTH,
        damage_cooldown: 0.0,
        game_screen: GameScreen::Game
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

        match state.game_screen {
            GameScreen::Game | GameScreen::Paused => {
                //Display level 
                tile_textures.bind();
                level_shader.use_program();
                level_shader.uniform_matrix4f("uPerspective", &state.perspective);
                level_shader.uniform_matrix4f("uView", &view_matrix);
                let transform_matrix = Matrix4::from_scale(0.5);
                level_shader.uniform_matrix4f("uTransform", &transform_matrix);
                level.display(); 
                //Display player sprite 
                rect_vao.bind();
                sprite_shader.use_program();
                sprite_shader.uniform_matrix4f("uPerspective", &state.perspective);
                sprite_shader.uniform_matrix4f("uView", &view_matrix);
                sprite_shader.uniform_float("uTexScale", 1.0 / 8.0);
                sprite_textures.bind();
                display_player(&rect_vao, &sprite_shader, &state);
                //Display tiles that the player can interact with
                cube_vao.bind();
                sprite_shader.uniform_bool("uFlipped", false);
                level.display_interactive_tiles(&cube_vao, &sprite_shader, &state.player.position);    
            }
            GameScreen::GameOver => {
                //Display level 
                tile_textures.bind();
                level_shader.use_program();
                level_shader.uniform_matrix4f("uPerspective", &state.perspective);
                level_shader.uniform_matrix4f("uView", &view_matrix);
                let transform_matrix = Matrix4::from_scale(0.5);
                level_shader.uniform_matrix4f("uTransform", &transform_matrix);
                level.display(); 
                //Display tiles that the player can interact with
                sprite_shader.use_program();
                sprite_shader.uniform_matrix4f("uPerspective", &state.perspective);
                sprite_shader.uniform_matrix4f("uView", &view_matrix);
                sprite_shader.uniform_float("uTexScale", 1.0 / 8.0);
                sprite_textures.bind();
                cube_vao.bind();
                sprite_shader.uniform_bool("uFlipped", false);
                level.display_interactive_tiles(&cube_vao, &sprite_shader, &state.player.position);
            }
        }    

        //Display text 
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        icons.bind(); 
        text_shader.use_program();
        text_shader.uniform_float("uTexScale", 1.0 / text::ICONS_TEXTURE_SCALE);
        let (win_w, win_h) = window.get_size();
        text_shader.uniform_vec2f("uScreenDimensions", win_w as f32, win_h as f32);
        
        match state.game_screen {
            GameScreen::Game => {
                display_game_screen_hud(&rect_vao, &text_shader, &state, &window);
                //Make the screen flash red if the player takes damage
                if state.damage_cooldown > 0.0 {
                    rect_shader.use_program();
                    rect_shader.uniform_vec4f("uColor", 1.0, 0.0, 0.0, state.damage_cooldown);
                    rect_vao.draw_arrays();
                } 
            }
            GameScreen::Paused => { 
                display_game_screen_hud(&rect_vao, &text_shader, &state, &window);
                rect_shader.use_program();
                rect_shader.uniform_vec4f("uColor", 0.6, 0.6, 0.6, 0.4);
                rect_vao.draw_arrays();
                text_shader.use_program();
                text::display_ascii_text_centered(
                    &rect_vao,
                    &text_shader,
                    b"Paused",
                    0.0,
                    96.0,
                    24.0
                );
                text::display_ascii_text_centered(
                    &rect_vao,
                    &text_shader,
                    b"Press Escape to Unpause",
                    0.0,
                    48.0,
                    8.0
                );
            }
            GameScreen::GameOver => {
                rect_shader.use_program();
                rect_shader.uniform_vec4f("uColor", 1.0, 0.0, 0.0, 0.4);
                rect_vao.draw_arrays();
                
                text_shader.use_program();
                text::display_ascii_text_centered(
                    &rect_vao,
                    &text_shader,
                    b"Game Over",
                    0.0,
                    96.0,
                    24.0
                );

                text::display_ascii_text_centered(
                    &rect_vao,
                    &text_shader,
                    format!("score:{}", state.score).as_bytes(),
                    0.0,
                    48.0,
                    8.0,
                );
            }
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        match state.game_screen {
            GameScreen::Game => { 
                update_game_screen(&mut state, &mut level, dt); 
            }
            GameScreen::Paused => {}
            GameScreen::GameOver => {}
        }

        gfx::output_gl_errors();
        window.swap_buffers();
        glfw.poll_events();
        //Calculate the amount of time passed in a single frame
        let end = Instant::now();
        dt = end.duration_since(start).as_secs_f32();
    }

    Ok(())
}
