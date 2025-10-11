use macroquad::prelude::*;

mod config;
mod defaults;
mod game;
mod models;
mod rendering;

use crate::config::GameConfig;
use game::update_all_systems;
use models::GameState;
use rendering::{render_game, render_ui};

/// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Ghost Ally Shooter".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::new();

    loop {
        let delta = get_frame_time();

        // Hot-reload config with R key (development only)
        if is_key_pressed(KeyCode::R) {
            match GameConfig::try_load_from_file() {
                Ok(new_config) => {
                    println!("✔ Config reloaded from config.toml!");
                    game_state.apply_config(&new_config);
                }
                Err(e) => {
                    println!("✘ Failed to reload config.toml: {}", e);
                }
            }
        }

        // Update game logic
        update_all_systems(&mut game_state, delta);

        // Render everything
        clear_background(Color::from_rgba(10, 10, 30, 255));
        render_game(&game_state);
        render_ui(&game_state);

        // Display FPS (debug)
        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            screen_height() - 10.0,
            20.0,
            GREEN,
        );

        next_frame().await
    }
}
