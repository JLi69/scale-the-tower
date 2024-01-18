#![windows_subsystem = "windows"]
#![allow(clippy::zero_ptr)]

extern crate gl;
extern crate glfw;

mod audio;
mod game;
mod gfx;
mod level;
mod shader;
mod sprite;
mod ui;

use crate::audio::sfx_ids;
use audio::SfxPlayer;
use cgmath::Matrix4;
use game::{hiscore, GameScreen, State};
use glfw::Context;
use level::room_template;
use level::Level;
use sprite::Sprite;
use std::{sync::mpsc::Receiver, time::Instant};

fn get_glfw_window_info(window: &glfw::Window) -> ui::WindowInfo {
    let (mouse_x, mouse_y) = window.get_cursor_pos();
    let (win_w, win_h) = window.get_size();
    ui::WindowInfo {
        mouse_x: mouse_x as f32,
        mouse_y: mouse_y as f32,
        win_w: win_w as f32,
        win_h: win_h as f32,
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
    sfx_player: &SfxPlayer,
) {
    let key_id = key as i32;
    if action == glfw::Action::Press {
        state.handle_key_press(key_id, sfx_player);
    } else if action == glfw::Action::Release {
        state.handle_key_release(key_id);
    }
}

fn process_events(
    _window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
    state: &mut State,
    sfx_player: &SfxPlayer,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            //Window resize
            glfw::WindowEvent::FramebufferSize(width, height) => {
                handle_window_resize(width, height, state);
            }
            //Key input
            glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                handle_key_input(key, scancode, action, modifiers, state, sfx_player);
            }
            _ => {}
        }
    }
}

fn process_button_action(button_action: ui::ButtonAction, state: &mut State) {
    match button_action {
        ui::ButtonAction::QuitGame => {
            std::process::exit(0);
        }
        ui::ButtonAction::GotoMainMenu => state.game_screen = GameScreen::MainMenu,
        ui::ButtonAction::GotoHighScores => state.game_screen = GameScreen::HighScores,
        ui::ButtonAction::GotoAbout => state.game_screen = GameScreen::AboutScreen,
        ui::ButtonAction::StartGame => {
            let persp_matrix = state.perspective;
            *state = State::starting_state();
            state.perspective = persp_matrix;
            state.game_screen = GameScreen::Game;
            //Attempt to load room templates
            let room_templates = room_template::load_room_templates("assets/room_templates");
            let (level, enemies) = Level::generate_level(&room_templates);
            state.level = level;
            state.enemies = enemies;
            state.level.build_chunks();
        }
    }
}

fn load_icon(window: &mut glfw::Window) {
    match gfx::load_image_pixels("assets/appicon.png") {
        Ok((buf, info)) => {
            window.set_icon_from_pixels(vec![glfw::PixelImage {
                width: info.width,
                height: info.height,
                pixels: buf,
            }]);
        }
        Err(msg) => eprintln!("{msg}"),
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
    load_icon(&mut window);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
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
    let background_shader = shader::program_from_vert_and_frag(
        "assets/shaders/rect_vert.glsl",
        "assets/shaders/background_frag.glsl",
    );
    //Load Textures
    let sprite_textures = gfx::load_texture("assets/textures/sprites.png");
    let tile_textures = gfx::load_texture("assets/textures/tiles.png");
    let icons = gfx::load_texture("assets/textures/icons.png");
    //Initialize the current state of the application
    let mut state = State::starting_state();

    let pause_menu = ui::Menu::create_pause_menu();
    let main_menu = ui::Menu::create_main_menu();
    let gameover_menu = ui::Menu::create_gameover_menu();
    let hiscore_menu = ui::Menu::create_hiscore_menu();
    let win_screen = ui::Menu::create_win_screen();
    let about_screen = ui::Menu::create_about_screen();

    let mut dt = 0.0f32;
    let mut animation_timer = 0.0f32;
    let mut highscores = hiscore::load_highscores("hiscores");

    let sfx_player = SfxPlayer::init();

    while !window.should_close() {
        let start = Instant::now();
        process_events(&mut window, &events, &mut state, &sfx_player);
        let win_info = get_glfw_window_info(&window);

        let view_matrix = Matrix4::from_translation(cgmath::vec3(
            -state.player_position().x,
            -state.player_position().y,
            level::LEVEL_Z,
        ));

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }

        level_shader.use_program();
        level_shader.uniform_matrix4f("uPerspective", &state.perspective);
        level_shader.uniform_matrix4f("uView", &view_matrix);
        let transform_matrix = Matrix4::from_scale(0.5);
        level_shader.uniform_matrix4f("uTransform", &transform_matrix);
        level_shader.uniform_float("uAnimationTimer", (animation_timer).fract() * 2.0);
        sprite_shader.use_program();
        sprite_shader.uniform_matrix4f("uPerspective", &state.perspective);
        sprite_shader.uniform_matrix4f("uView", &view_matrix);
        sprite_shader.uniform_float("uTexScale", 1.0 / 8.0);

        match state.game_screen {
            GameScreen::MainMenu | GameScreen::HighScores | GameScreen::AboutScreen => {
                tile_textures.bind();
                background_shader.use_program();
                background_shader.uniform_vec2f(
                    "uScreenDimensions",
                    win_info.win_w,
                    win_info.win_h,
                );
                rect_vao.draw_arrays();
            }
            GameScreen::Game | GameScreen::Paused | GameScreen::WinScreen => {
                //Display level
                tile_textures.bind();
                level_shader.use_program();
                state.level.display(&state.player_position());
                //Display player sprite
                rect_vao.bind();
                sprite_shader.use_program();
                sprite_textures.bind();
                state.player.display_player(&rect_vao, &sprite_shader);
                //Display tiles that the player can interact with
                cube_vao.bind();
                sprite_shader.uniform_bool("uFlipped", false);
                state.level.display_interactive_tiles(
                    &cube_vao,
                    &sprite_shader,
                    &state.player_position(),
                    animation_timer,
                );
                rect_vao.bind();
                state.display_enemies(&rect_vao, &sprite_shader);
                state.display_projectiles(&rect_vao, &sprite_shader);
                state.display_particles(&rect_vao, &sprite_shader);
            }
            GameScreen::GameOver => {
                //Display level
                tile_textures.bind();
                level_shader.use_program();
                state.level.display(&state.player_position());
                //Display tiles that the player can interact with
                sprite_shader.use_program();
                sprite_textures.bind();
                cube_vao.bind();
                sprite_shader.uniform_bool("uFlipped", false);
                state.level.display_interactive_tiles(
                    &cube_vao,
                    &sprite_shader,
                    &state.player_position(),
                    animation_timer,
                );
                rect_vao.bind();
                state.display_enemies(&rect_vao, &sprite_shader);
                state.display_projectiles(&rect_vao, &sprite_shader);
                state.display_particles(&rect_vao, &sprite_shader);
            }
        }

        //Display text
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        rect_vao.bind();
        icons.bind();
        text_shader.use_program();
        text_shader.uniform_float("uTexScale", 1.0 / ui::ICONS_TEXTURE_SCALE);
        text_shader.uniform_vec2f("uScreenDimensions", win_info.win_w, win_info.win_h);
        text_shader.uniform_vec4f("uColor", 1.0, 1.0, 1.0, 1.0);

        match state.game_screen {
            GameScreen::MainMenu => {
                main_menu.display(&rect_vao, &text_shader, &win_info);
            }
            GameScreen::AboutScreen => {
                about_screen.display(&rect_vao, &text_shader, &win_info);
            }
            GameScreen::WinScreen => {
                text_shader.use_program();
                ui::display_ascii_text_centered(
                    &rect_vao,
                    &text_shader,
                    format!("score:{}", state.player.score).as_bytes(),
                    0.0,
                    96.0,
                    8.0,
                );

                if state.new_highscore {
                    ui::display_ascii_text_centered(
                        &rect_vao,
                        &text_shader,
                        b"New High Score!",
                        0.0,
                        72.0,
                        8.0,
                    );
                }

                win_screen.display(&rect_vao, &text_shader, &win_info);
            }
            GameScreen::HighScores => {
                hiscore::display_hiscores(&rect_vao, &text_shader, &highscores);
                hiscore_menu.display(&rect_vao, &text_shader, &win_info);
            }
            GameScreen::Game => {
                state
                    .player
                    .display_player_stats(&rect_vao, &text_shader, &window);
                //Make the screen flash red if the player takes damage
                if state.player.damage_cooldown > 0.0 {
                    rect_shader.use_program();
                    rect_shader.uniform_vec4f(
                        "uColor",
                        1.0,
                        0.0,
                        0.0,
                        state.player.damage_cooldown,
                    );
                    rect_vao.draw_arrays();
                }
            }
            GameScreen::Paused => {
                state
                    .player
                    .display_player_stats(&rect_vao, &text_shader, &window);
                rect_shader.use_program();
                rect_shader.uniform_vec4f("uColor", 0.6, 0.6, 0.6, 0.4);
                rect_vao.draw_arrays();
                text_shader.use_program();
                pause_menu.display(&rect_vao, &text_shader, &win_info);
            }
            GameScreen::GameOver => {
                rect_shader.use_program();
                rect_shader.uniform_vec4f("uColor", 1.0, 0.0, 0.0, 0.4);
                rect_vao.draw_arrays();

                text_shader.use_program();
                ui::display_ascii_text_centered(
                    &rect_vao,
                    &text_shader,
                    format!("score:{}", state.player.score).as_bytes(),
                    0.0,
                    48.0,
                    8.0,
                );

                if state.new_highscore {
                    ui::display_ascii_text_centered(
                        &rect_vao,
                        &text_shader,
                        b"New High Score!",
                        0.0,
                        24.0,
                        8.0,
                    );
                }

                gameover_menu.display(&rect_vao, &text_shader, &win_info);
            }
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        //Handle interaction with menu
        let left_mouse_held = window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press;
        if left_mouse_held && !state.left_mouse_held {
            let button_action = match state.game_screen {
                GameScreen::Game => None,
                GameScreen::AboutScreen => about_screen.get_clicked_button_action(&win_info),
                GameScreen::WinScreen => win_screen.get_clicked_button_action(&win_info),
                GameScreen::HighScores => hiscore_menu.get_clicked_button_action(&win_info),
                GameScreen::Paused => pause_menu.get_clicked_button_action(&win_info),
                GameScreen::GameOver => gameover_menu.get_clicked_button_action(&win_info),
                GameScreen::MainMenu => main_menu.get_clicked_button_action(&win_info),
            };

            if let Some(action) = button_action {
                sfx_player.play(sfx_ids::SELECT);
                process_button_action(action, &mut state);
            }
        }

        state.left_mouse_held = left_mouse_held;

        if state.game_screen == GameScreen::Game {
            state.update_game_screen(dt, &sfx_player);
            state.check_gameover(&mut highscores, &sfx_player);
        } else if state.game_screen == GameScreen::GameOver {
            state.update_enemies(dt, &sfx_player);
            state.update_projectiles(dt);
            state.update_particles(dt);
        }

        if state.game_screen == GameScreen::Game {
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        } else {
            window.set_cursor_mode(glfw::CursorMode::Normal);
        }

        //Update the animation timer
        animation_timer += dt;

        gfx::output_gl_errors();
        window.swap_buffers();
        glfw.poll_events();
        //Calculate the amount of time passed in a single frame
        let end = Instant::now();
        dt = end.duration_since(start).as_secs_f32();
    }

    hiscore::write_highscores("hiscores", &highscores);

    Ok(())
}
