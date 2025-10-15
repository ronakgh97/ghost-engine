use crate::models::*;
use macroquad::prelude::*;

/// Update player position and state
pub fn update_player(state: &mut GameState, delta: f32) {
    // Keep player on screen
    state.player.pos.x = state.player.pos.x.clamp(15.0, screen_width() - 15.0);

    // Player can only move in bottom half of screen
    state.player.pos.y = state
        .player
        .pos
        .y
        .clamp(screen_height() / 2.0, screen_height() - 15.0);

    // Update hit flash timer
    if state.player.hit_flash_timer > 0.0 {
        state.player.hit_flash_timer -= delta;
    }

    // Check for game over
    if state.player.stats.health <= 0.0 {
        // TODO: Implement game over screen
        state.player.stats.health = 0.0;
    }
}
