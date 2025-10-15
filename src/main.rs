use macroquad::prelude::*;

mod config;
mod defaults;
mod game;
mod models;
mod rendering;
mod scripting;

use crate::config::GameConfig;
use game::update_all_systems;
use models::GameState;
use rendering::{render_game, render_ui};

/// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Ghost Engine".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let space_texture = match load_texture("assets/background/space_2.png").await {
        Ok(tex) => {
            println!("✓ Space background loaded successfully!");
            Some(tex)
        }
        Err(e) => {
            println!("✗ Failed to load background: {e}");
            None
        }
    };

    let mut game_state = GameState::new();

    loop {
        let delta = get_frame_time();

        // Hot-reload config with R key
        if is_key_pressed(KeyCode::R) {
            match GameConfig::try_load_from_file() {
                Ok(new_config) => {
                    println!("✓ Config reloaded from config.toml!");
                    game_state.apply_config(&new_config);
                }
                Err(e) => {
                    println!("✗ Failed to reload config: {e}");
                }
            }
        }

        // Update background scroll offset
        game_state.bg_scroll_offset += game_state.config.background.scroll_speed * delta;

        // Update game logic
        update_all_systems(&mut game_state, delta);

        render_game(&game_state, &space_texture);
        render_ui(&game_state);

        next_frame().await
    }
}
