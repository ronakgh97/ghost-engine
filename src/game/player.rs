use crate::models::*;
use macroquad::prelude::*;

/// Update player position and state
pub fn update_player(state: &mut GameState, _delta: f32) {
    // Keep player on screen
    state.player.pos.x = state.player.pos.x.clamp(15.0, screen_width() - 15.0);

    // Player can only move in bottom half of screen
    state.player.pos.y = state
        .player
        .pos
        .y
        .clamp(screen_height() / 2.0, screen_height() - 15.0);

    // Check for game over
    if state.player.stats.health <= 0.0 {
        // TODO: Implement game over screen
        state.player.stats.health = 0.0;
    }
}
